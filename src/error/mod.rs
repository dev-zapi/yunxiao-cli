//! Error types for the YunXiao CLI.
//!
//! Provides a unified error type [`CliError`] that wraps all possible error
//! sources encountered during CLI execution, including configuration, auth,
//! API, cache, IO, HTTP, and JSON errors.

use thiserror::Error;

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

    /// Cache layer errors (read/write failures, corrupt data, etc.)
    #[error("Cache error: {0}")]
    Cache(String),

    /// Standard IO errors propagated from file system operations.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// HTTP transport errors from the reqwest client.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

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
