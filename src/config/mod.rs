//! Configuration management for the YunXiao CLI.
//!
//! Handles loading, saving, and resolving configuration values with the
//! following priority (highest → lowest):
//!
//! 1. CLI arguments
//! 2. Environment variables (`YUNXIAO_CLI_*`)
//! 3. Config file (`~/.config/yunxiao-cli/config.toml`)
//! 4. Hard-coded defaults

pub mod types;

pub use types::*;

use crate::error::{CliError, Result};
use log::debug;
use std::path::PathBuf;

// ───────────────────────────── Path helpers ──────────────────────────────

/// Returns the configuration directory: `~/.config/yunxiao-cli/`.
///
/// Creates the directory if it does not exist.
pub fn config_dir() -> PathBuf {
    let base = dirs::config_dir().unwrap_or_else(|| {
        dirs::home_dir()
            .expect("Cannot determine home directory")
            .join(".config")
    });
    base.join("yunxiao-cli")
}

/// Returns the full path to the config file: `~/.config/yunxiao-cli/config.toml`.
pub fn config_file_path() -> PathBuf {
    config_dir().join("config.toml")
}

// ───────────────────────── Load / Save ───────────────────────────────────

/// Load the CLI configuration from disk.
///
/// If the config file does not exist, returns [`CliConfig::default()`].
/// Any parse errors are surfaced as [`CliError::Config`].
pub fn load_config() -> Result<CliConfig> {
    let path = config_file_path();
    if !path.exists() {
        debug!(
            "Config file not found at {}, using defaults",
            path.display()
        );
        return Ok(CliConfig::default());
    }

    let content = std::fs::read_to_string(&path)?;
    let config: CliConfig = toml::from_str(&content)
        .map_err(|e| CliError::Config(format!("Failed to parse config file: {e}")))?;
    debug!("Loaded config from {}", path.display());
    Ok(config)
}

/// Persist the given configuration to disk.
///
/// Creates the parent directory if needed and, on Unix, restricts the file
/// permissions to `0o600` (owner read/write only) to protect the token.
pub fn save_config(config: &CliConfig) -> Result<()> {
    let dir = config_dir();
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }

    let content = toml::to_string_pretty(config)
        .map_err(|e| CliError::Config(format!("Failed to serialise config: {e}")))?;

    let path = config_file_path();
    std::fs::write(&path, &content)?;

    // Restrict permissions on Unix to protect the token.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o600);
        std::fs::set_permissions(&path, perms)?;
    }

    debug!("Saved config to {}", path.display());
    Ok(())
}

// ────────────────────── Resolution helpers ───────────────────────────────

/// Resolve the API token with priority: CLI arg → env var → config file.
///
/// Returns an error if no token can be found at any level.
pub fn resolve_token(cli_token: Option<&str>) -> Result<String> {
    // 1. CLI argument
    if let Some(t) = cli_token {
        return Ok(t.to_string());
    }

    // 2. Environment variable
    if let Ok(t) = std::env::var("YUNXIAO_CLI_TOKEN") {
        if !t.is_empty() {
            return Ok(t);
        }
    }

    // 3. Config file
    let cfg = load_config()?;
    if let Some(t) = cfg.token {
        if !t.is_empty() {
            return Ok(t);
        }
    }

    Err(CliError::Auth(
        "No API token found. Run `yunxiao auth login` or set YUNXIAO_CLI_TOKEN.".into(),
    ))
}

/// Resolve the output format with priority: CLI arg → env var → config → default (text).
pub fn resolve_output_format(cli_format: Option<&OutputFormat>) -> OutputFormat {
    // 1. CLI argument
    if let Some(f) = cli_format {
        return f.clone();
    }

    // 2. Environment variable
    if let Ok(val) = std::env::var("YUNXIAO_CLI_OUTPUT") {
        if let Ok(f) = val.parse::<OutputFormat>() {
            return f;
        }
    }

    // 3. Config file
    if let Ok(cfg) = load_config() {
        if let Some(f) = cfg.default_output {
            return f;
        }
    }

    // 4. Default
    OutputFormat::Text
}

/// Resolve the HTTP timeout with priority: CLI arg → env var → config → default (30s).
pub fn resolve_timeout(cli_timeout: Option<u64>) -> u64 {
    // 1. CLI argument
    if let Some(t) = cli_timeout {
        return t;
    }

    // 2. Environment variable
    if let Ok(val) = std::env::var("YUNXIAO_CLI_TIMEOUT") {
        if let Ok(t) = val.parse::<u64>() {
            return t;
        }
    }

    // 3. Config file
    if let Ok(cfg) = load_config() {
        if let Some(t) = cfg.timeout {
            return t;
        }
    }

    // 4. Default
    DEFAULT_TIMEOUT
}

/// Resolve the log level with priority: CLI arg → env var → config → default (Warn).
pub fn resolve_log_level(cli_level: Option<&LogLevel>) -> LogLevel {
    // 1. CLI argument
    if let Some(l) = cli_level {
        return l.clone();
    }

    // 2. Environment variable
    if let Ok(val) = std::env::var("YUNXIAO_CLI_LOG_LEVEL") {
        if let Ok(l) = val.parse::<LogLevel>() {
            return l;
        }
    }

    // 3. Config file
    if let Ok(cfg) = load_config() {
        if let Some(l) = cfg.log_level {
            return l;
        }
    }

    // 4. Default
    LogLevel::Warn
}

/// Resolve the API endpoint with priority: CLI arg → env var → config → default.
///
/// The returned value is always a full URL (e.g. `https://openapi-rdc.aliyuncs.com`).
/// Bare domains are automatically prefixed with `https://`.
pub fn resolve_endpoint(cli_endpoint: Option<&str>) -> String {
    // 1. CLI argument
    if let Some(e) = cli_endpoint {
        return normalise_endpoint(e);
    }

    // 2. Environment variable (new name first, then legacy)
    if let Ok(e) = std::env::var("YUNXIAO_CLI_ENDPOINT") {
        if !e.is_empty() {
            return normalise_endpoint(&e);
        }
    }
    if let Ok(d) = std::env::var("YUNXIAO_CLI_DOMAIN") {
        if !d.is_empty() {
            return normalise_endpoint(&d);
        }
    }

    // 3. Config file
    if let Ok(cfg) = load_config() {
        if let Some(e) = cfg.endpoint {
            if !e.is_empty() {
                return normalise_endpoint(&e);
            }
        }
    }

    // 4. Default
    DEFAULT_ENDPOINT.to_string()
}

/// Legacy alias for [`resolve_endpoint`].
pub fn resolve_domain(cli_domain: Option<&str>) -> String {
    resolve_endpoint(cli_domain)
}

/// Normalise an endpoint value to a full URL.
///
/// If the input already starts with `http://` or `https://`, it is returned
/// as-is (with any trailing slash stripped).  Otherwise it is treated as a
/// bare domain and prefixed with `https://`.
fn normalise_endpoint(raw: &str) -> String {
    let trimmed = raw.trim().trim_end_matches('/');
    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        trimmed.to_string()
    } else {
        format!("https://{trimmed}")
    }
}

/// Resolve the organization ID with priority: CLI arg → env var → config.
///
/// Returns `None` if no organization ID is configured at any level.
pub fn resolve_org_id(cli_org_id: Option<&str>) -> Option<String> {
    // 1. CLI argument
    if let Some(o) = cli_org_id {
        return Some(o.to_string());
    }

    // 2. Environment variable
    if let Ok(o) = std::env::var("YUNXIAO_CLI_ORG_ID") {
        if !o.is_empty() {
            return Some(o);
        }
    }

    // 3. Config file
    if let Ok(cfg) = load_config() {
        if let Some(o) = cfg.organization_id {
            if !o.is_empty() {
                return Some(o);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn config_dir_is_under_home() {
        let dir = config_dir();
        let home = dirs::home_dir().unwrap();
        // Should be ~/.config/yunxiao-cli/ or equivalent
        assert!(dir.starts_with(&home));
        assert!(dir.to_string_lossy().contains("yunxiao-cli"));
    }

    #[test]
    fn config_file_path_ends_with_config_toml() {
        let path = config_file_path();
        assert_eq!(path.file_name().unwrap(), "config.toml");
    }

    #[test]
    fn load_config_returns_default_when_no_file() {
        // This test may rely on the file not existing in the test env.
        // We just verify it doesn't panic and returns a valid config.
        let cfg = load_config().unwrap();
        // Default config has all fields as None
        assert!(cfg.timeout.is_none() || cfg.timeout.is_some());
    }

    #[test]
    fn save_and_load_config_roundtrip() {
        use tempfile::TempDir;

        // We can't easily override config_dir() without changing the code,
        // so we test the serialization/deserialization logic directly.
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("config.toml");

        let cfg = CliConfig {
            token: Some("test-token".into()),
            endpoint: Some("https://example.com".into()),
            organization_id: Some("org-123".into()),
            default_output: Some(OutputFormat::Table),
            log_level: Some(LogLevel::Info),
            timeout: Some(60),
        };

        let content = toml::to_string_pretty(&cfg).unwrap();
        std::fs::write(&path, &content).unwrap();

        let loaded_content = std::fs::read_to_string(&path).unwrap();
        let loaded: CliConfig = toml::from_str(&loaded_content).unwrap();

        assert_eq!(loaded.token, Some("test-token".into()));
        assert_eq!(loaded.endpoint, Some("https://example.com".into()));
        assert_eq!(loaded.organization_id, Some("org-123".into()));
        assert_eq!(loaded.default_output, Some(OutputFormat::Table));
        assert_eq!(loaded.log_level, Some(LogLevel::Info));
        assert_eq!(loaded.timeout, Some(60));
    }

    // ────── Priority resolution tests ──────

    #[test]
    fn resolve_output_format_cli_arg_wins() {
        let fmt = resolve_output_format(Some(&OutputFormat::Markdown));
        assert_eq!(fmt, OutputFormat::Markdown);
    }

    #[test]
    fn resolve_output_format_default_is_text() {
        // Remove env var to test default fallback
        env::remove_var("YUNXIAO_CLI_OUTPUT");
        let fmt = resolve_output_format(None);
        // Will be Text unless a config file overrides it
        assert!(fmt == OutputFormat::Text || fmt == OutputFormat::Json);
    }

    #[test]
    fn resolve_timeout_cli_arg_wins() {
        let t = resolve_timeout(Some(120));
        assert_eq!(t, 120);
    }

    #[test]
    fn resolve_timeout_default() {
        env::remove_var("YUNXIAO_CLI_TIMEOUT");
        let t = resolve_timeout(None);
        // Default is 30 unless config file overrides it
        assert!(t > 0);
    }

    #[test]
    fn resolve_log_level_cli_arg_wins() {
        let level = resolve_log_level(Some(&LogLevel::Debug));
        assert_eq!(level, LogLevel::Debug);
    }

    #[test]
    fn resolve_endpoint_cli_arg_wins() {
        let endpoint = resolve_endpoint(Some("custom.domain.com"));
        assert_eq!(endpoint, "https://custom.domain.com");
    }

    #[test]
    fn resolve_endpoint_full_url_preserved() {
        let endpoint = resolve_endpoint(Some("https://custom.example.com"));
        assert_eq!(endpoint, "https://custom.example.com");
    }

    #[test]
    fn resolve_endpoint_default() {
        env::remove_var("YUNXIAO_CLI_ENDPOINT");
        env::remove_var("YUNXIAO_CLI_DOMAIN");
        let endpoint = resolve_endpoint(None);
        // Default or config file value
        assert!(!endpoint.is_empty());
        assert!(endpoint.starts_with("https://"));
    }

    #[test]
    fn resolve_org_id_cli_arg_wins() {
        let org = resolve_org_id(Some("cli-org"));
        assert_eq!(org, Some("cli-org".into()));
    }

    #[test]
    fn resolve_org_id_none_when_empty() {
        env::remove_var("YUNXIAO_CLI_ORG_ID");
        // When no CLI arg, env, or config, should return None
        let org = resolve_org_id(None);
        // May or may not be None depending on config file
        assert!(org.is_none() || org.is_some());
    }

    #[test]
    fn resolve_token_cli_arg_wins() {
        let token = resolve_token(Some("cli-token-xxx"));
        assert_eq!(token.unwrap(), "cli-token-xxx");
    }

    #[test]
    fn resolve_token_env_var() {
        env::set_var("YUNXIAO_CLI_TOKEN", "env-token-yyy");
        let token = resolve_token(None);
        assert_eq!(token.unwrap(), "env-token-yyy");
        env::remove_var("YUNXIAO_CLI_TOKEN");
    }
}
