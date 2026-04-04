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
    _token: String,
    /// Per-request timeout.
    _timeout: Duration,
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
            _token: token.to_string(),
            _timeout: duration,
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
        assert_eq!(client._token, "test-token");
        assert_eq!(client._timeout, Duration::from_secs(30));
    }

    #[test]
    fn api_client_new_custom_endpoint() {
        let client = ApiClient::new("tok", "https://custom.example.com", 120).unwrap();
        assert_eq!(client.base_url, "https://custom.example.com");
    }

    #[test]
    fn api_client_new_different_timeouts() {
        let client = ApiClient::new("tok", "https://api.test.com", 5).unwrap();
        assert_eq!(client._timeout, Duration::from_secs(5));

        let client = ApiClient::new("tok", "https://api.test.com", 300).unwrap();
        assert_eq!(client._timeout, Duration::from_secs(300));
    }
}
