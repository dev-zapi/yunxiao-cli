//! Configuration types and default values for the YunXiao CLI.
//!
//! Defines the structures that are serialised to / deserialised from the
//! persistent config file at `~/.config/yunxiao-cli/config.toml`.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

// ───────────────────────────── OutputFormat ──────────────────────────────

/// Output format for CLI results.
///
/// Controls how command output is rendered to the terminal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    /// Pretty-printed JSON.
    Json,
    /// Plain text (key: value pairs or raw strings).
    Text,
    /// ASCII table using comfy-table.
    Table,
    /// Markdown-formatted output.
    Markdown,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Text => write!(f, "text"),
            OutputFormat::Table => write!(f, "table"),
            OutputFormat::Markdown => write!(f, "markdown"),
        }
    }
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "text" => Ok(OutputFormat::Text),
            "table" => Ok(OutputFormat::Table),
            "markdown" | "md" => Ok(OutputFormat::Markdown),
            other => Err(format!(
                "Unknown output format '{other}'. Valid values: json, text, table, markdown"
            )),
        }
    }
}

// ──────────────────────────── LogLevel ───────────────────────────────────

/// Log level configuration.
///
/// Maps directly to the standard `log` crate levels.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    /// Verbose diagnostic information.
    Debug,
    /// General informational messages.
    Info,
    /// Potential problems that are not yet errors.
    Warn,
    /// Actionable error messages.
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}

impl FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            other => Err(format!(
                "Unknown log level '{other}'. Valid values: debug, info, warn, error"
            )),
        }
    }
}

impl LogLevel {
    /// Convert to a `log::LevelFilter` suitable for initialising the logger.
    pub fn to_level_filter(&self) -> log::LevelFilter {
        match self {
            LogLevel::Debug => log::LevelFilter::Debug,
            LogLevel::Info => log::LevelFilter::Info,
            LogLevel::Warn => log::LevelFilter::Warn,
            LogLevel::Error => log::LevelFilter::Error,
        }
    }
}

// ──────────────────────────── CliConfig ──────────────────────────────────

/// Global CLI configuration persisted to `~/.config/yunxiao-cli/config.toml`.
///
/// All fields are optional; missing values fall back to environment variables
/// or hard-coded defaults during resolution (see [`super::resolve_*`] helpers).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    /// Personal access token for the Yunxiao API.
    pub token: Option<String>,

    /// API endpoint domain. Defaults to `"openapi-rdc.aliyuncs.com"`.
    pub domain: Option<String>,

    /// Active organization identifier.
    pub organization_id: Option<String>,

    /// Default output format when `--output` is not passed.
    pub default_output: Option<OutputFormat>,

    /// Default log level when `--log-level` is not passed.
    pub log_level: Option<LogLevel>,

    /// HTTP request timeout in **seconds**. Defaults to `30`.
    pub timeout: Option<u64>,
}

/// Default API domain used when none is configured.
pub const DEFAULT_DOMAIN: &str = "openapi-rdc.aliyuncs.com";

/// Default HTTP timeout in seconds.
pub const DEFAULT_TIMEOUT: u64 = 30;

impl Default for CliConfig {
    /// Returns a config with all fields set to `None`, relying on the
    /// resolution layer to apply sensible defaults.
    fn default() -> Self {
        Self {
            token: None,
            domain: None,
            organization_id: None,
            default_output: None,
            log_level: None,
            timeout: None,
        }
    }
}
