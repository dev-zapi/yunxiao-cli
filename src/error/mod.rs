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
