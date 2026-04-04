//! HTTP client layer for the YunXiao (Alibaba Cloud DevOps) API.
//!
//! Wraps [`reqwest::Client`] with pre-configured base URL, authentication
//! header (`x-yunxiao-token`), and timeout settings.

use crate::error::{CliError, Result};
use log::debug;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Middleware, Next};
use reqwest::{Request, Response};
use std::time::Duration;
use http::Extensions;

/// Captured headers from request and response for debugging.
#[derive(Debug, Clone, Default)]
pub struct CapturedHeaders {
    pub request_headers: Vec<(String, String)>,
    pub response_headers: Vec<(String, String)>,
}

/// Middleware to capture request and response headers.
pub struct HeaderCaptureMiddleware;

#[async_trait::async_trait]
impl Middleware for HeaderCaptureMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> std::result::Result<Response, reqwest_middleware::Error> {
        // Capture request headers
        let request_headers: Vec<(String, String)> = req
            .headers()
            .iter()
            .map(|(k, v)| {
                let key = k.to_string();
                let value = v.to_str().unwrap_or("[binary]").to_string();
                (key, value)
            })
            .collect();

        // Execute request
        let resp = next.run(req, extensions).await?;

        // Capture response headers
        let response_headers: Vec<(String, String)> = resp
            .headers()
            .iter()
            .map(|(k, v)| {
                let key = k.to_string();
                let value = v.to_str().unwrap_or("[binary]").to_string();
                (key, value)
            })
            .collect();

        // Store captured headers in extensions for access by the caller
        extensions.insert(CapturedHeaders {
            request_headers,
            response_headers,
        });

        Ok(resp)
    }
}

/// Debug information for a failed API request.
#[derive(Debug)]
struct DebugInfo {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
    response_headers: Vec<(String, String)>,
}

/// API client for all YunXiao REST calls.
///
/// Constructed via [`ApiClient::new`] and used throughout command handlers
/// to perform authenticated HTTP requests against the DevOps API.
pub struct ApiClient {
    /// Underlying reqwest async client with middleware.
    http: ClientWithMiddleware,
    /// Base URL including scheme, e.g. `https://openapi-rdc.aliyuncs.com`.
    base_url: String,
    /// Personal access token sent in every request.
    #[allow(dead_code)]
    token: String,
    /// Per-request timeout.
    #[allow(dead_code)]
    timeout: Duration,
}

/// Response data including headers and body.
#[derive(Debug)]
pub struct ApiResponse {
    pub headers: HeaderMap,
    pub body: serde_json::Value,
}

impl ApiClient {
    /// Create a new API client.
    ///
    /// # Arguments
    /// * `token`    – Personal access token for `x-yunxiao-token` header.
    /// * `endpoint` – Full API endpoint URL (e.g. `https://openapi-rdc.aliyuncs.com`).
    /// * `timeout`  – Request timeout in seconds.
    pub fn new(token: &str, endpoint: &str, timeout: u64) -> Result<Self> {
        let mut default_headers = HeaderMap::new();
        // Authentication header required by the Yunxiao API.
        default_headers.insert(
            "x-yunxiao-token",
            HeaderValue::from_str(token)
                .map_err(|e| CliError::Auth(format!("Invalid token value: {e}")))?,
        );
        default_headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        default_headers.insert(
            reqwest::header::ACCEPT,
            HeaderValue::from_static("application/json"),
        );
        default_headers.insert(
            reqwest::header::USER_AGENT,
            HeaderValue::from_static(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            )),
        );

        let duration = Duration::from_secs(timeout);
        let reqwest_client = reqwest::Client::builder()
            .default_headers(default_headers)
            .timeout(duration)
            .build()?;

        // Build client with middleware
        let http = ClientBuilder::new(reqwest_client)
            .with(HeaderCaptureMiddleware)
            .build();

        let base_url = endpoint.to_string();
        debug!("ApiClient created for {}", base_url);

        Ok(Self {
            http,
            base_url,
            token: token.to_string(),
            timeout: duration,
        })
    }

    /// Perform a GET request.
    ///
    /// # Arguments
    /// * `path`   – Request path, e.g. `/oapi/v1/user/current`.
    /// * `params` – Query string key-value pairs.
    pub async fn get(&self, path: &str, params: &[(&str, &str)]) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        debug!("GET {} params={:?}", url, params);

        let resp = self.http.get(&url).query(params).send().await?;

        self.handle_response(resp).await
    }

    /// Perform a POST request with a JSON body.
    pub async fn post(&self, path: &str, body: &serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        debug!("POST {}", url);
        debug!("Request body: {}", body);

        let body_str = body.to_string();
        let resp = self.http
            .post(&url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body_str)
            .send()
            .await?;

        self.handle_response(resp).await
    }

    /// Perform a PUT request with a JSON body.
    pub async fn put(&self, path: &str, body: &serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        debug!("PUT {}", url);
        debug!("Request body: {}", body);

        let body_str = body.to_string();
        let resp = self.http
            .put(&url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body_str)
            .send()
            .await?;

        self.handle_response(resp).await
    }

    /// Perform a DELETE request.
    pub async fn delete(&self, path: &str, params: &[(&str, &str)]) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        debug!("DELETE {} params={:?}", url, params);

        let resp = self.http.delete(&url).query(params).send().await?;

        self.handle_response(resp).await
    }

    /// Perform a POST request with a JSON body and return response with headers.
    pub async fn post_with_headers(
        &self,
        path: &str,
        body: &serde_json::Value,
    ) -> Result<ApiResponse> {
        let url = format!("{}{}", self.base_url, path);
        debug!("POST {}", url);
        debug!("Request body: {}", body);

        let body_str = body.to_string();
        let resp = self.http
            .post(&url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body_str)
            .send()
            .await?;

        self.handle_response_with_headers(resp).await
    }

    /// Handle response and return both headers and body.
    async fn handle_response_with_headers(
        &self,
        resp: reqwest::Response,
    ) -> Result<ApiResponse> {
        let status = resp.status();
        let url = resp.url().to_string();
        let response_headers: HeaderMap = resp.headers().clone();

        debug!("Response status: {}", status);
        debug!("Response URL: {}", url);

        if status.is_success() {
            let text = resp.text().await?;
            if text.is_empty() {
                return Ok(ApiResponse {
                    headers: response_headers,
                    body: serde_json::json!({"status": "ok"}),
                });
            }
            let value: serde_json::Value = serde_json::from_str(&text).map_err(|e| {
                CliError::Api(format!("Failed to parse response JSON from {url}: {e}"))
            })?;
            Ok(ApiResponse {
                headers: response_headers,
                body: value,
            })
        } else {
            let body = resp.text().await.unwrap_or_default();
            Err(CliError::Api(format!("HTTP {status} from {url}: {body}")))
        }
    }

    /// Process an HTTP response, returning the JSON body on success or a
    /// [`CliError::Api`] on non-2xx status codes.
    async fn handle_response(
        &self,
        resp: reqwest::Response,
    ) -> Result<serde_json::Value> {
        let status = resp.status();
        let url = resp.url().to_string();

        if status.is_success() {
            // Some endpoints return 204 No Content
            let text = resp.text().await?;
            if text.is_empty() {
                return Ok(serde_json::json!({"status": "ok"}));
            }
            let value: serde_json::Value = serde_json::from_str(&text).map_err(|e| {
                CliError::Api(format!("Failed to parse response JSON from {url}: {e}"))
            })?;
            Ok(value)
        } else {
            let body = resp.text().await.unwrap_or_default();
            Err(CliError::Api(format!("HTTP {status} from {url}: {body}")))
        }
    }

    /// Internal method to execute request with captured headers for debugging.
    /// This is used when you need to capture headers on error.
    async fn execute_with_debug(
        &self,
        method: &str,
        path: &str,
        params: Option<&[(&str, &str)]>,
        body: Option<&serde_json::Value>,
    ) -> Result<(serde_json::Value, CapturedHeaders)> {
        let url = format!("{}{}", self.base_url, path);
        
        let mut req_builder = match method {
            "GET" => self.http.get(&url),
            "POST" => self.http.post(&url),
            "PUT" => self.http.put(&url),
            "DELETE" => self.http.delete(&url),
            _ => return Err(CliError::Api(format!("Unsupported method: {method}"))),
        };

        if let Some(p) = params {
            req_builder = req_builder.query(p);
        }

        if let Some(b) = body {
            let body_str = b.to_string();
            req_builder = req_builder
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(body_str);
        }

        // Build the request to capture headers
        let request = req_builder.try_clone().ok_or_else(|| {
            CliError::Api("Failed to clone request".to_string())
        })?.build()?;

        let request_headers: Vec<(String, String)> = request
            .headers()
            .iter()
            .map(|(k, v)| {
                let key = k.to_string();
                let value = v.to_str().unwrap_or("[binary]").to_string();
                (key, value)
            })
            .collect();

        // Execute request
        let resp = self.http.execute(request).await?;
        let status = resp.status();
        let response_headers: Vec<(String, String)> = resp
            .headers()
            .iter()
            .map(|(k, v)| {
                let key = k.to_string();
                let value = v.to_str().unwrap_or("[binary]").to_string();
                (key, value)
            })
            .collect();

        let captured = CapturedHeaders {
            request_headers,
            response_headers,
        };

        if status.is_success() {
            let text = resp.text().await?;
            if text.is_empty() {
                return Ok((serde_json::json!({"status": "ok"}), captured));
            }
            let value: serde_json::Value = serde_json::from_str(&text).map_err(|e| {
                CliError::Api(format!("Failed to parse response JSON from {url}: {e}"))
            })?;
            Ok((value, captured))
        } else {
            let body_str = resp.text().await.unwrap_or_default();
            
            // Create debug info for error reporting
            let debug_info = DebugInfo {
                method: method.to_string(),
                url: url.clone(),
                headers: captured.request_headers.clone(),
                body: body.map(|b| b.to_string()),
                response_headers: captured.response_headers.clone(),
            };
            
            self.save_debug_info(&debug_info, status.as_u16(), &body_str);
            
            Err(CliError::Api(format!("HTTP {status} from {url}: {body_str}")))
        }
    }

    /// Save debug information to $XDG_DATA_HOME/yunxiao-cli/ directory when API call fails.
    fn save_debug_info(&self, debug_info: &DebugInfo, status: u16, response_body: &str) {
        use std::time::SystemTime;
        use log::warn;
        
        // Get data directory (XDG_DATA_HOME, defaults to ~/.local/share)
        let data_dir = match dirs::data_dir() {
            Some(path) => path,
            None => {
                warn!("Could not determine data directory, skipping debug file save");
                return;
            }
        };

        let share_dir = data_dir.join("yunxiao-cli");
        if !share_dir.exists() {
            if let Err(e) = std::fs::create_dir_all(&share_dir) {
                warn!("Failed to create {} directory: {}", share_dir.display(), e);
                return;
            }
        }

        // Generate timestamp for unique filename
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let filename = format!(
            "api-error-{}-{}-{}.txt",
            timestamp,
            debug_info.method.to_lowercase(),
            status
        );
        let filepath = share_dir.join(&filename);

        // Build debug output
        let mut output = String::new();
        output.push_str("============================================\n");
        output.push_str("API ERROR DEBUG INFORMATION\n");
        output.push_str("============================================\n\n");

        // Request information
        output.push_str("[REQUEST]\n");
        output.push_str(&format!("Method: {}\n", debug_info.method));
        output.push_str(&format!("URL: {}\n", debug_info.url));

        if !debug_info.headers.is_empty() {
            output.push_str("\nRequest Headers:\n");
            for (key, value) in &debug_info.headers {
                // Mask sensitive headers
                let display_value = if key.to_lowercase() == "authorization"
                    || key.to_lowercase() == "x-yunxiao-token"
                {
                    "***REDACTED***"
                } else {
                    value
                };
                output.push_str(&format!("  {}: {}\n", key, display_value));
            }
        }

        if let Some(body) = &debug_info.body {
            output.push_str("\nRequest Body:\n");
            // Pretty print JSON if possible
            match serde_json::from_str::<serde_json::Value>(body) {
                Ok(json) => {
                    if let Ok(pretty) = serde_json::to_string_pretty(&json) {
                        output.push_str(&pretty);
                    } else {
                        output.push_str(body);
                    }
                }
                Err(_) => output.push_str(body),
            }
            output.push('\n');
        }

        // Response information
        output.push_str("\n--------------------------------------------\n");
        output.push_str("[RESPONSE]\n");
        output.push_str(&format!("Status Code: {}\n", status));

        if !debug_info.response_headers.is_empty() {
            output.push_str("\nResponse Headers:\n");
            for (key, value) in &debug_info.response_headers {
                output.push_str(&format!("  {}: {}\n", key, value));
            }
        }

        output.push_str("\nResponse Body:\n");

        // Try to pretty print JSON response
        match serde_json::from_str::<serde_json::Value>(response_body) {
            Ok(json) => {
                if let Ok(pretty) = serde_json::to_string_pretty(&json) {
                    output.push_str(&pretty);
                } else {
                    output.push_str(response_body);
                }
            }
            Err(_) => output.push_str(response_body),
        }
        output.push('\n');

        output.push_str("\n============================================\n");
        output.push_str(&format!("Debug file: {}\n", filepath.display()));
        output.push_str("============================================\n");

        // Write to file
        match std::fs::write(&filepath, output) {
            Ok(_) => {
                // Use eprintln! to always show this message regardless of log level
                eprintln!(
                    "[INFO] API error debug details saved to: {}",
                    filepath.display()
                );
            }
            Err(e) => {
                eprintln!("[WARN] API error debug info save failed: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_client_new_valid() {
        let client = ApiClient::new("test-token", "https://openapi-rdc.aliyuncs.com", 30);
        assert!(client.is_ok());
        let client = client.unwrap();
        assert_eq!(client.base_url, "https://openapi-rdc.aliyuncs.com");
        assert_eq!(client.token, "test-token");
        assert_eq!(client.timeout, Duration::from_secs(30));
    }

    #[test]
    fn api_client_new_custom_endpoint() {
        let client = ApiClient::new("tok", "https://custom.example.com", 120).unwrap();
        assert_eq!(client.base_url, "https://custom.example.com");
    }

    #[test]
    fn api_client_new_different_timeouts() {
        let client = ApiClient::new("tok", "https://api.test.com", 5).unwrap();
        assert_eq!(client.timeout, Duration::from_secs(5));

        let client = ApiClient::new("tok", "https://api.test.com", 300).unwrap();
        assert_eq!(client.timeout, Duration::from_secs(300));
    }
}
