//! HTTP client layer for the YunXiao (Alibaba Cloud DevOps) API.
//!
//! Wraps [`reqwest::Client`] with pre-configured base URL, authentication
//! header (`x-yunxiao-token`), and timeout settings.

use crate::error::{CliError, Result};
use log::{debug, error};
use reqwest::header::{HeaderMap, HeaderValue};
use std::time::Duration;

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
    pub async fn get(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        debug!("GET {} params={:?}", url, params);

        let resp = self
            .http
            .get(&url)
            .query(params)
            .send()
            .await?;

        self.handle_response(resp).await
    }

    /// Perform a POST request with a JSON body.
    pub async fn post(
        &self,
        path: &str,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        debug!("POST {}", url);

        let resp = self
            .http
            .post(&url)
            .json(body)
            .send()
            .await?;

        self.handle_response(resp).await
    }

    /// Perform a PUT request with a JSON body.
    pub async fn put(
        &self,
        path: &str,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        debug!("PUT {}", url);

        let resp = self
            .http
            .put(&url)
            .json(body)
            .send()
            .await?;

        self.handle_response(resp).await
    }

    /// Perform a DELETE request.
    pub async fn delete(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        debug!("DELETE {} params={:?}", url, params);

        let resp = self
            .http
            .delete(&url)
            .query(params)
            .send()
            .await?;

        self.handle_response(resp).await
    }

    // ──────────────────────── Internal helpers ───────────────────────────

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
            error!("API error: {} {} – {}", status, url, body);
            Err(CliError::Api(format!(
                "HTTP {status} from {url}: {body}"
            )))
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
        let domain = server
            .host_with_port()
            .to_string();
        // Build client manually to avoid https
        let http = reqwest::Client::builder()
            .default_headers({
                let mut h = reqwest::header::HeaderMap::new();
                h.insert("x-yunxiao-token", reqwest::header::HeaderValue::from_static("test-token"));
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
                h.insert("x-yunxiao-token", reqwest::header::HeaderValue::from_static("test-token"));
                h.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/json"));
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
