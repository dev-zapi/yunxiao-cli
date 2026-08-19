//! Error types for the YunXiao CLI.
//!
//! Provides a unified error type [`CliError`] that wraps all possible error
//! sources encountered during CLI execution, including configuration, auth,
//! API, cache, IO, HTTP, and JSON errors.

use serde::Deserialize;
use thiserror::Error;

/// YunXiao API error response structure.
#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct YunxiaoApiError {
    pub errorCode: String,
    pub errorMessage: String,
    #[serde(default)]
    pub requestId: Option<String>,
}

/// Structured details returned by a non-successful YunXiao API response.
#[derive(Debug, Clone)]
pub struct ApiError {
    /// HTTP response status.
    pub status: u16,
    /// YunXiao error code when the response provides one.
    pub code: Option<String>,
    /// YunXiao error message, or the unstructured response body.
    pub message: String,
    /// Request URL.
    pub url: String,
    /// YunXiao request ID when the response provides one.
    pub request_id: Option<String>,
}

impl ApiError {
    /// Build structured error details from an HTTP response body.
    pub fn from_response(body: &str, status: u16, url: &str) -> Self {
        if let Ok(api_error) = serde_json::from_str::<YunxiaoApiError>(body) {
            Self {
                status,
                code: Some(api_error.errorCode),
                message: api_error.errorMessage,
                url: url.to_string(),
                request_id: api_error.requestId,
            }
        } else {
            Self {
                status,
                code: None,
                message: body.to_string(),
                url: url.to_string(),
                request_id: None,
            }
        }
    }
}

/// User-friendly error messages for common YunXiao API error codes.
fn get_user_friendly_message(code: &str, details: &str, status: u16) -> String {
    match (status, code) {
        // 401 Unauthorized
        (401, "InvalidToken") | (401, "InvalidTokenError") => {
            format!("令牌无效，请检查 Token 是否正确配置。\n详情: {details}")
        }
        (401, "ExpiredTokenError") => {
            format!("令牌已过期，请重新获取 Token。\n详情: {details}")
        }
        (401, "Unauthorized") => {
            format!("未授权访问，请检查 Token 是否有效。\n详情: {details}")
        }

        // 403 Forbidden
        (403, "Forbidden.InvalidUser.UserNotInCurrentOrganization") => {
            format!("当前用户未加入该组织，请联系组织管理员添加。\n详情: {details}")
        }
        (403, "Forbidden.InvalidOrganizationMember") => {
            format!("当前用户在组织中无效，没有操作权限。\n详情: {details}")
        }
        (403, "Forbidden") => {
            format!("无权限访问此资源。可能原因：\n  1. Token 缺少相应权限\n  2. 未加入该组织\n  3. 没有该资源的访问权限\n详情: {details}")
        }

        // 404 Not Found
        (404, "NotFound") => {
            format!("资源不存在，请检查请求路径和参数是否正确。\n详情: {details}")
        }

        // 400 Bad Request
        (400, "BadRequest") => {
            format!("请求参数错误，请检查参数格式是否正确。\n详情: {details}")
        }
        (400, "UnsupportedInCurrentEnv") => {
            format!("当前环境版本不支持此 API。\n详情: {details}")
        }
        (400, "UnsupportedCurrentTokenType") => {
            format!("当前令牌类型不支持此 API，请查阅文档确认鉴权方式。\n详情: {details}")
        }

        // 429 Too Many Requests
        (429, "TooManyRequests") => {
            format!("请求过于频繁，请稍后重试。\n详情: {details}")
        }

        // 500 Internal Server Error
        (500, "InternalServerError") => {
            format!("服务器内部错误，请稍后重试。\n详情: {details}")
        }

        // Default: include the error code
        _ => {
            format!("API 错误 [{code}]: {details}")
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(code) = &self.code {
            let friendly = get_user_friendly_message(code, &self.message, self.status);
            let request_info = self
                .request_id
                .as_ref()
                .map(|id| format!("\n请求ID: {id}"))
                .unwrap_or_default();
            write!(f, "{friendly}\nURL: {}{request_info}", self.url)
        } else {
            let message = match self.status {
                401 => format!(
                    "认证失败 (HTTP 401)\n可能原因：\n  1. Token 无效或已过期\n  2. Token 未正确配置\nURL: {}\n响应: {}",
                    self.url, self.message
                ),
                403 => format!(
                    "权限不足 (HTTP 403)\n可能原因：\n  1. Token 缺少相应权限\n  2. 未加入该组织\n  3. 没有该资源的访问权限\nURL: {}\n响应: {}",
                    self.url, self.message
                ),
                404 => format!(
                    "资源不存在 (HTTP 404)\nURL: {}\n响应: {}",
                    self.url, self.message
                ),
                429 => format!(
                    "请求过于频繁 (HTTP 429)，请稍后重试\nURL: {}\n响应: {}",
                    self.url, self.message
                ),
                500..=599 => format!(
                    "服务器错误 (HTTP {})，请稍后重试\nURL: {}\n响应: {}",
                    self.status, self.url, self.message
                ),
                _ => format!("HTTP {} from {}: {}", self.status, self.url, self.message),
            };
            write!(f, "{message}")
        }
    }
}

/// Parse API error response and return a user-friendly error message.
pub fn parse_api_error(body: &str, status: u16, url: &str) -> String {
    ApiError::from_response(body, status, url).to_string()
}

/// Unified error type for all CLI operations.
#[derive(Error, Debug)]
pub enum CliError {
    /// Configuration-related errors (missing file, invalid format, etc.)
    #[error("Configuration error: {0}")]
    Config(String),

    /// Authentication errors (missing token, expired, invalid, etc.)
    #[error("Authentication error: {0}")]
    Auth(String),

    /// API request or response errors (bad status, unexpected body, etc.)
    #[error("API error: {0}")]
    Api(String),

    /// Structured YunXiao API error response.
    #[error("API error: {0}")]
    ApiResponse(ApiError),

    /// Cache layer errors (read/write failures, corrupt data, etc.)
    #[error("Cache error: {0}")]
    Cache(String),

    /// Standard IO errors propagated from file system operations.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// HTTP transport errors from the reqwest client.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// HTTP middleware errors from reqwest-middleware.
    #[error("HTTP middleware error: {0}")]
    HttpMiddleware(#[from] reqwest_middleware::Error),

    /// JSON serialization / deserialization errors.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Catch-all for errors that don't fit other categories.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Convenience result alias used throughout the CLI crate.
pub type Result<T> = std::result::Result<T, CliError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_error_config_display() {
        let err = CliError::Config("bad format".into());
        assert!(err.to_string().contains("Configuration error"));
        assert!(err.to_string().contains("bad format"));
    }

    #[test]
    fn cli_error_auth_display() {
        let err = CliError::Auth("token expired".into());
        assert!(err.to_string().contains("Authentication error"));
        assert!(err.to_string().contains("token expired"));
    }

    #[test]
    fn cli_error_api_display() {
        let err = CliError::Api("404 Not Found".into());
        assert!(err.to_string().contains("API error"));
        assert!(err.to_string().contains("404 Not Found"));
    }

    #[test]
    fn api_error_preserves_response_metadata() {
        let error = ApiError::from_response(
            r#"{"errorCode":"InvaildData.Failed","errorMessage":"标签未找到","requestId":"request-1"}"#,
            400,
            "https://api.example.test/workitems",
        );

        assert_eq!(error.status, 400);
        assert_eq!(error.code.as_deref(), Some("InvaildData.Failed"));
        assert_eq!(error.message, "标签未找到");
        assert_eq!(error.request_id.as_deref(), Some("request-1"));

        let display = CliError::ApiResponse(error).to_string();
        assert!(display.contains("InvaildData.Failed"));
        assert!(display.contains("https://api.example.test/workitems"));
        assert!(display.contains("request-1"));
    }

    #[test]
    fn cli_error_cache_display() {
        let err = CliError::Cache("corrupt file".into());
        assert!(err.to_string().contains("Cache error"));
        assert!(err.to_string().contains("corrupt file"));
    }

    #[test]
    fn cli_error_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
        let cli_err: CliError = io_err.into();
        assert!(matches!(cli_err, CliError::Io(_)));
        assert!(cli_err.to_string().contains("file missing"));
    }

    #[test]
    fn cli_error_from_json_error() {
        let json_result: std::result::Result<serde_json::Value, _> =
            serde_json::from_str("not json");
        let json_err = json_result.unwrap_err();
        let cli_err: CliError = json_err.into();
        assert!(matches!(cli_err, CliError::Json(_)));
    }

    #[test]
    fn cli_error_from_anyhow() {
        let anyhow_err = anyhow::anyhow!("something went wrong");
        let cli_err: CliError = anyhow_err.into();
        assert!(matches!(cli_err, CliError::Other(_)));
        assert!(cli_err.to_string().contains("something went wrong"));
    }

    #[test]
    fn result_alias_ok() {
        fn returns_ok() -> Result<i32> {
            Ok(42)
        }
        let r = returns_ok();
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), 42);
    }

    #[test]
    fn result_alias_err() {
        let r: Result<i32> = Err(CliError::Config("test".into()));
        assert!(r.is_err());
    }
}
