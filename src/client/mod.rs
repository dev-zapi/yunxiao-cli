//! HTTP client layer for the YunXiao (Alibaba Cloud DevOps) API.
//!
//! Wraps [`reqwest::Client`] with pre-configured base URL, authentication
//! header (`x-devops-pat`), and timeout settings.

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
    /// * `token`   – Personal access token for `x-devops-pat` header.
    /// * `domain`  – API domain (e.g. `openapi-rdc.aliyuncs.com`).
    /// * `timeout` – Request timeout in seconds.
    pub fn new(token: &str, domain: &str, timeout: u64) -> Result<Self> {
        let mut default_headers = HeaderMap::new();
        // Authentication header required by the Yunxiao API.
        default_headers.insert(
            "x-devops-pat",
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

        let base_url = format!("https://{domain}");
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
