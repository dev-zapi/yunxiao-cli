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

    /// API endpoint address (full URL or bare domain).
    ///
    /// Accepts a full URL (e.g. `https://openapi-rdc.aliyuncs.com`) or a bare
    /// domain (e.g. `openapi-rdc.aliyuncs.com`).  Bare domains are
    /// automatically prefixed with `https://`.
    ///
    /// Defaults to `https://openapi-rdc.aliyuncs.com` (central edition).
    #[serde(alias = "domain")]
    pub endpoint: Option<String>,

    /// Active organization identifier.
    pub organization_id: Option<String>,

    /// Default output format when `--output` is not passed.
    pub default_output: Option<OutputFormat>,

    /// Default log level when `--log-level` is not passed.
    pub log_level: Option<LogLevel>,

    /// HTTP request timeout in **seconds**. Defaults to `30`.
    pub timeout: Option<u64>,
}

/// Default API endpoint URL used when none is configured.
pub const DEFAULT_ENDPOINT: &str = "https://openapi-rdc.aliyuncs.com";

/// Legacy constant kept for backward compatibility in tests.
pub const DEFAULT_DOMAIN: &str = "openapi-rdc.aliyuncs.com";

/// Default HTTP timeout in seconds.
pub const DEFAULT_TIMEOUT: u64 = 30;

impl Default for CliConfig {
    /// Returns a config with all fields set to `None`, relying on the
    /// resolution layer to apply sensible defaults.
    fn default() -> Self {
        Self {
            token: None,
            endpoint: None,
            organization_id: None,
            default_output: None,
            log_level: None,
            timeout: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ────── OutputFormat ──────

    #[test]
    fn output_format_parse_valid_values() {
        assert_eq!("json".parse::<OutputFormat>().unwrap(), OutputFormat::Json);
        assert_eq!("JSON".parse::<OutputFormat>().unwrap(), OutputFormat::Json);
        assert_eq!("text".parse::<OutputFormat>().unwrap(), OutputFormat::Text);
        assert_eq!(
            "table".parse::<OutputFormat>().unwrap(),
            OutputFormat::Table
        );
        assert_eq!(
            "markdown".parse::<OutputFormat>().unwrap(),
            OutputFormat::Markdown
        );
        assert_eq!(
            "md".parse::<OutputFormat>().unwrap(),
            OutputFormat::Markdown
        );
    }

    #[test]
    fn output_format_parse_invalid() {
        assert!("xml".parse::<OutputFormat>().is_err());
        assert!("".parse::<OutputFormat>().is_err());
    }

    #[test]
    fn output_format_display() {
        assert_eq!(OutputFormat::Json.to_string(), "json");
        assert_eq!(OutputFormat::Text.to_string(), "text");
        assert_eq!(OutputFormat::Table.to_string(), "table");
        assert_eq!(OutputFormat::Markdown.to_string(), "markdown");
    }

    #[test]
    fn output_format_serde_roundtrip() {
        let json_str = serde_json::to_string(&OutputFormat::Json).unwrap();
        assert_eq!(json_str, "\"json\"");
        let parsed: OutputFormat = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed, OutputFormat::Json);
    }

    // ────── LogLevel ──────

    #[test]
    fn log_level_parse_valid_values() {
        assert_eq!("debug".parse::<LogLevel>().unwrap(), LogLevel::Debug);
        assert_eq!("INFO".parse::<LogLevel>().unwrap(), LogLevel::Info);
        assert_eq!("Warn".parse::<LogLevel>().unwrap(), LogLevel::Warn);
        assert_eq!("error".parse::<LogLevel>().unwrap(), LogLevel::Error);
    }

    #[test]
    fn log_level_parse_invalid() {
        assert!("trace".parse::<LogLevel>().is_err());
        assert!("".parse::<LogLevel>().is_err());
    }

    #[test]
    fn log_level_display() {
        assert_eq!(LogLevel::Debug.to_string(), "debug");
        assert_eq!(LogLevel::Info.to_string(), "info");
        assert_eq!(LogLevel::Warn.to_string(), "warn");
        assert_eq!(LogLevel::Error.to_string(), "error");
    }

    #[test]
    fn log_level_to_filter() {
        assert_eq!(LogLevel::Debug.to_level_filter(), log::LevelFilter::Debug);
        assert_eq!(LogLevel::Info.to_level_filter(), log::LevelFilter::Info);
        assert_eq!(LogLevel::Warn.to_level_filter(), log::LevelFilter::Warn);
        assert_eq!(LogLevel::Error.to_level_filter(), log::LevelFilter::Error);
    }

    #[test]
    fn log_level_serde_roundtrip() {
        let json_str = serde_json::to_string(&LogLevel::Info).unwrap();
        assert_eq!(json_str, "\"info\"");
        let parsed: LogLevel = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed, LogLevel::Info);
    }

    // ────── CliConfig ──────

    #[test]
    fn cli_config_default_all_none() {
        let cfg = CliConfig::default();
        assert!(cfg.token.is_none());
        assert!(cfg.endpoint.is_none());
        assert!(cfg.organization_id.is_none());
        assert!(cfg.default_output.is_none());
        assert!(cfg.log_level.is_none());
        assert!(cfg.timeout.is_none());
    }

    #[test]
    fn cli_config_serde_roundtrip() {
        let cfg = CliConfig {
            token: Some("test-token-123".into()),
            endpoint: Some("https://custom.example.com".into()),
            organization_id: Some("org-abc".into()),
            default_output: Some(OutputFormat::Table),
            log_level: Some(LogLevel::Debug),
            timeout: Some(60),
        };
        let toml_str = toml::to_string_pretty(&cfg).unwrap();
        let parsed: CliConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.token, cfg.token);
        assert_eq!(parsed.endpoint, cfg.endpoint);
        assert_eq!(parsed.organization_id, cfg.organization_id);
        assert_eq!(parsed.default_output, cfg.default_output);
        assert_eq!(parsed.log_level, cfg.log_level);
        assert_eq!(parsed.timeout, cfg.timeout);
    }

    #[test]
    fn cli_config_deserialize_empty_toml() {
        let cfg: CliConfig = toml::from_str("").unwrap();
        assert!(cfg.token.is_none());
        assert!(cfg.endpoint.is_none());
    }

    #[test]
    fn cli_config_deserialize_partial_toml() {
        let toml_str = r#"
            token = "my-token"
            timeout = 45
        "#;
        let cfg: CliConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.token, Some("my-token".into()));
        assert_eq!(cfg.timeout, Some(45));
        assert!(cfg.endpoint.is_none());
        assert!(cfg.default_output.is_none());
    }

    #[test]
    fn default_constants() {
        assert_eq!(DEFAULT_ENDPOINT, "https://openapi-rdc.aliyuncs.com");
        assert_eq!(DEFAULT_DOMAIN, "openapi-rdc.aliyuncs.com");
        assert_eq!(DEFAULT_TIMEOUT, 30);
    }

    #[test]
    fn cli_config_deserialize_legacy_domain() {
        let toml_str = r#"
            domain = "openapi-rdc-sg.aliyuncs.com"
        "#;
        let cfg: CliConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.endpoint, Some("openapi-rdc-sg.aliyuncs.com".into()));
    }
}
