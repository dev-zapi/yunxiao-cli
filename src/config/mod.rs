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

/// Resolve the output format with priority: CLI arg → env var → config → default (JSON).
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
    OutputFormat::Json
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

/// Resolve the API domain with priority: CLI arg → env var → config → default.
pub fn resolve_domain(cli_domain: Option<&str>) -> String {
    // 1. CLI argument
    if let Some(d) = cli_domain {
        return d.to_string();
    }

    // 2. Environment variable
    if let Ok(d) = std::env::var("YUNXIAO_CLI_DOMAIN") {
        if !d.is_empty() {
            return d;
        }
    }

    // 3. Config file
    if let Ok(cfg) = load_config() {
        if let Some(d) = cfg.domain {
            if !d.is_empty() {
                return d;
            }
        }
    }

    // 4. Default
    DEFAULT_DOMAIN.to_string()
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
