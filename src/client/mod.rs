//! HTTP client layer for the YunXiao (Alibaba Cloud DevOps) API.
//!
//! Wraps [`reqwest::Client`] with pre-configured base URL, authentication
//! header (`x-yunxiao-token`), and timeout settings.

use crate::error::{CliError, Result};
use log::{debug, warn};
use reqwest::header::{HeaderMap, HeaderValue};
use std::time::Duration;
use std::time::SystemTime;

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
    /// Underlying reqwest async client.
    http: reqwest::Client,
    /// Base URL including scheme, e.g. `https://openapi-rdc.aliyuncs.com`.
    base_url: String,
    /// Personal access token sent in every request.
    #[allow(dead_code)]
    token: String,
    /// Per-request timeout.
    #[allow(dead_code)]
    timeout: Duration,
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
        let http = reqwest::Client::builder()
            .default_headers(default_headers)
            .timeout(duration)
            .build()?;

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

        let req_builder = self.http.get(&url).query(params);
        let debug_info = DebugInfo {
            method: "GET".to_string(),
            url: url.clone(),
            headers: Vec::new(),
            body: None,
            response_headers: Vec::new(),
        };

        let resp = req_builder.send().await?;

        self.handle_response(resp, debug_info).await
    }

    /// Perform a POST request with a JSON body.
    pub async fn post(&self, path: &str, body: &serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        debug!("POST {}", url);

        let body_str = body.to_string();
        let req_builder = self.http.post(&url).json(body);
        let debug_info = DebugInfo {
            method: "POST".to_string(),
            url: url.clone(),
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            body: Some(body_str),
            response_headers: Vec::new(),
        };

        let resp = req_builder.send().await?;

        self.handle_response(resp, debug_info).await
    }

    /// Perform a PUT request with a JSON body.
    pub async fn put(&self, path: &str, body: &serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        debug!("PUT {}", url);

        let body_str = body.to_string();
        let req_builder = self.http.put(&url).json(body);
        let debug_info = DebugInfo {
            method: "PUT".to_string(),
            url: url.clone(),
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            body: Some(body_str),
            response_headers: Vec::new(),
        };

        let resp = req_builder.send().await?;

        self.handle_response(resp, debug_info).await
    }

    /// Perform a DELETE request.
    pub async fn delete(&self, path: &str, params: &[(&str, &str)]) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        debug!("DELETE {} params={:?}", url, params);

        let req_builder = self.http.delete(&url).query(params);
        let debug_info = DebugInfo {
            method: "DELETE".to_string(),
            url: url.clone(),
            headers: Vec::new(),
            body: None,
            response_headers: Vec::new(),
        };

        let resp = req_builder.send().await?;

        self.handle_response(resp, debug_info).await
    }

    // ──────────────────────── Internal helpers ───────────────────────────

    /// Save debug information to ~/.share/yunxiao-cli/ directory when API call fails.
    fn save_debug_info(&self, debug_info: &DebugInfo, status: u16, response_body: &str) {
        // Get home directory
        let home_dir = match dirs::home_dir() {
            Some(path) => path,
            None => {
                warn!("Could not determine home directory, skipping debug file save");
                return;
            }
        };

        let share_dir = home_dir.join(".share").join("yunxiao-cli");
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

    /// Process an HTTP response, returning the JSON body on success or a
    /// [`CliError::Api`] on non-2xx status codes.
    async fn handle_response(
        &self,
        resp: reqwest::Response,
        mut debug_info: DebugInfo,
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
            // Extract response headers before consuming the response body
            debug_info.response_headers = resp
                .headers()
                .iter()
                .map(|(k, v)| {
                    let key = k.to_string();
                    let value = v.to_str().unwrap_or("[binary]").to_string();
                    (key, value)
                })
                .collect();

            let body = resp.text().await.unwrap_or_default();
            let status_code = status.as_u16();

            // Save debug information to .share directory
            self.save_debug_info(&debug_info, status_code, &body);

            Err(CliError::Api(format!("HTTP {status} from {url}: {body}")))
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

    #[tokio::test]
    async fn api_client_get_with_mock() {
        // Set up a mock server that returns a JSON response
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/oapi/v1/user/current")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"user-1","name":"Test User"}"#)
            .create_async()
            .await;

        // Create client pointing to mock server
        let domain = server.host_with_port().to_string();
        // Build client manually to avoid https
        let http = reqwest::Client::builder()
            .default_headers({
                let mut h = reqwest::header::HeaderMap::new();
                h.insert(
                    "x-yunxiao-token",
                    reqwest::header::HeaderValue::from_static("test-token"),
                );
                h
            })
            .build()
            .unwrap();
        let client = ApiClient {
            http,
            base_url: format!("http://{domain}"),
            token: "test-token".into(),
            timeout: Duration::from_secs(30),
        };

        let result = client.get("/oapi/v1/user/current", &[]).await;
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data["id"], "user-1");
        assert_eq!(data["name"], "Test User");

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn api_client_post_with_mock() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/oapi/v1/workitems")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"wi-123","status":"created"}"#)
            .create_async()
            .await;

        let domain = server.host_with_port().to_string();
        let http = reqwest::Client::builder()
            .default_headers({
                let mut h = reqwest::header::HeaderMap::new();
                h.insert(
                    "x-yunxiao-token",
                    reqwest::header::HeaderValue::from_static("test-token"),
                );
                h.insert(
                    reqwest::header::CONTENT_TYPE,
                    reqwest::header::HeaderValue::from_static("application/json"),
                );
                h
            })
            .build()
            .unwrap();
        let client = ApiClient {
            http,
            base_url: format!("http://{domain}"),
            token: "test-token".into(),
            timeout: Duration::from_secs(30),
        };

        let body = serde_json::json!({"subject": "Test Item"});
        let result = client.post("/oapi/v1/workitems", &body).await;
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data["id"], "wi-123");

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn api_client_handles_404_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/oapi/v1/not-found")
            .with_status(404)
            .with_body(r#"{"error":"not found"}"#)
            .create_async()
            .await;

        let domain = server.host_with_port().to_string();
        let http = reqwest::Client::new();
        let client = ApiClient {
            http,
            base_url: format!("http://{domain}"),
            token: "test-token".into(),
            timeout: Duration::from_secs(30),
        };

        let result = client.get("/oapi/v1/not-found", &[]).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, CliError::Api(_)));
        assert!(err.to_string().contains("404"));

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn api_client_handles_204_no_content() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("DELETE", "/oapi/v1/items/1")
            .with_status(204)
            .with_body("")
            .create_async()
            .await;

        let domain = server.host_with_port().to_string();
        let http = reqwest::Client::new();
        let client = ApiClient {
            http,
            base_url: format!("http://{domain}"),
            token: "test-token".into(),
            timeout: Duration::from_secs(30),
        };

        let result = client.delete("/oapi/v1/items/1", &[]).await;
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data["status"], "ok");

        mock.assert_async().await;
    }
}
