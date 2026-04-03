//! `config` subcommand – local configuration management.
//!
//! Allows users to get, set, list, and delete individual config values,
//! and to display the config file path.

use clap::{Args, Subcommand};
use serde_json::json;

use crate::config;
use crate::config::types::{LogLevel, OutputFormat};
use crate::error::{CliError, Result};
use crate::output;

/// Arguments for the `config` subcommand.
#[derive(Debug, Args)]
pub struct ConfigArgs {
    /// Config operation to perform.
    #[command(subcommand)]
    pub command: ConfigCommands,
}

/// Available config operations.
#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    /// Get the value of a configuration key.
    Get(GetArgs),
    /// Set a configuration key to a new value.
    Set(SetArgs),
    /// List all configuration values.
    List,
    /// Delete (unset) a configuration key.
    Delete(DeleteArgs),
    /// Show the path to the config file.
    Path,
}

/// Arguments for `config get`.
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Configuration key to retrieve (token, domain, organization_id, default_output, log_level, timeout).
    pub key: String,
}

/// Arguments for `config set`.
#[derive(Debug, Args)]
pub struct SetArgs {
    /// Configuration key to set.
    pub key: String,
    /// New value for the key.
    pub value: String,
}

/// Arguments for `config delete`.
#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Configuration key to unset.
    pub key: String,
}

/// Execute the `config` subcommand tree.
pub async fn execute(args: &ConfigArgs, format: &OutputFormat) -> Result<()> {
    match &args.command {
        ConfigCommands::Get(a) => cmd_get(a, format),
        ConfigCommands::Set(a) => cmd_set(a, format),
        ConfigCommands::List => cmd_list(format),
        ConfigCommands::Delete(a) => cmd_delete(a, format),
        ConfigCommands::Path => cmd_path(format),
    }
}

/// `config get <key>` – read a single value.
fn cmd_get(args: &GetArgs, format: &OutputFormat) -> Result<()> {
    let cfg = config::load_config()?;
    let value = match args.key.as_str() {
        "token" => cfg
            .token
            .as_deref()
            .map(|t| {
                // Mask the token for safety
                if t.len() > 8 {
                    format!("{}****{}", &t[..4], &t[t.len() - 4..])
                } else {
                    "****".to_string()
                }
            })
            .unwrap_or_default(),
        "domain" => cfg.domain.unwrap_or_default(),
        "organization_id" => cfg.organization_id.unwrap_or_default(),
        "default_output" => cfg
            .default_output
            .map(|o| o.to_string())
            .unwrap_or_default(),
        "log_level" => cfg
            .log_level
            .map(|l| l.to_string())
            .unwrap_or_default(),
        "timeout" => cfg
            .timeout
            .map(|t| t.to_string())
            .unwrap_or_default(),
        other => {
            return Err(CliError::Config(format!(
                "Unknown config key '{other}'. Valid keys: token, domain, organization_id, default_output, log_level, timeout"
            )));
        }
    };
    let data = json!({ args.key.clone(): value });
    output::print_output(&data, format)?;
    Ok(())
}

/// `config set <key> <value>` – update a single value.
fn cmd_set(args: &SetArgs, format: &OutputFormat) -> Result<()> {
    let mut cfg = config::load_config()?;
    match args.key.as_str() {
        "token" => cfg.token = Some(args.value.clone()),
        "domain" => cfg.domain = Some(args.value.clone()),
        "organization_id" => cfg.organization_id = Some(args.value.clone()),
        "default_output" => {
            let fmt = args
                .value
                .parse::<OutputFormat>()
                .map_err(CliError::Config)?;
            cfg.default_output = Some(fmt);
        }
        "log_level" => {
            let lvl = args
                .value
                .parse::<LogLevel>()
                .map_err(CliError::Config)?;
            cfg.log_level = Some(lvl);
        }
        "timeout" => {
            let secs: u64 = args
                .value
                .parse()
                .map_err(|_| CliError::Config("Timeout must be a positive integer.".into()))?;
            cfg.timeout = Some(secs);
        }
        other => {
            return Err(CliError::Config(format!(
                "Unknown config key '{other}'. Valid keys: token, domain, organization_id, default_output, log_level, timeout"
            )));
        }
    }
    config::save_config(&cfg)?;
    let data = json!({"status": "ok", "key": args.key, "value": args.value});
    output::print_output(&data, format)?;
    Ok(())
}

/// `config list` – show all current config values.
fn cmd_list(format: &OutputFormat) -> Result<()> {
    let cfg = config::load_config()?;
    let data = json!({
        "token": cfg.token.as_deref().map(|t| {
            if t.len() > 8 {
                format!("{}****{}", &t[..4], &t[t.len()-4..])
            } else {
                "****".to_string()
            }
        }).unwrap_or_else(|| "(not set)".to_string()),
        "domain": cfg.domain.unwrap_or_else(|| config::DEFAULT_DOMAIN.to_string()),
        "organization_id": cfg.organization_id.unwrap_or_else(|| "(not set)".to_string()),
        "default_output": cfg.default_output.map(|o| o.to_string()).unwrap_or_else(|| "json".to_string()),
        "log_level": cfg.log_level.map(|l| l.to_string()).unwrap_or_else(|| "warn".to_string()),
        "timeout": cfg.timeout.unwrap_or(config::DEFAULT_TIMEOUT),
    });
    output::print_output(&data, format)?;
    Ok(())
}

/// `config delete <key>` – unset a config value.
fn cmd_delete(args: &DeleteArgs, format: &OutputFormat) -> Result<()> {
    let mut cfg = config::load_config()?;
    match args.key.as_str() {
        "token" => cfg.token = None,
        "domain" => cfg.domain = None,
        "organization_id" => cfg.organization_id = None,
        "default_output" => cfg.default_output = None,
        "log_level" => cfg.log_level = None,
        "timeout" => cfg.timeout = None,
        other => {
            return Err(CliError::Config(format!(
                "Unknown config key '{other}'. Valid keys: token, domain, organization_id, default_output, log_level, timeout"
            )));
        }
    }
    config::save_config(&cfg)?;
    let data = json!({"status": "ok", "key": args.key, "message": "Key deleted."});
    output::print_output(&data, format)?;
    Ok(())
}

/// `config path` – print the config file location.
fn cmd_path(format: &OutputFormat) -> Result<()> {
    let path = config::config_file_path();
    let exists = path.exists();
    let data = json!({
        "path": path.display().to_string(),
        "exists": exists,
    });
    output::print_output(&data, format)?;
    Ok(())
}
