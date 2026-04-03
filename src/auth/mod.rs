//! Authentication / token management layer for the YunXiao CLI.
//!
//! Wraps the configuration layer to provide convenient helpers for
//! storing, retrieving, and clearing the personal access token.

use crate::config;
use crate::error::{CliError, Result};
use log::debug;

/// Save a personal access token to the config file.
///
/// Updates the existing config (if any) without overwriting other fields.
pub fn save_token(token: &str) -> Result<()> {
    let mut cfg = config::load_config()?;
    cfg.token = Some(token.to_string());
    config::save_config(&cfg)?;
    debug!("Token saved to config file");
    Ok(())
}

/// Retrieve the stored token from the config file.
///
/// Returns `Ok(None)` if no token has been saved.
pub fn get_token() -> Result<Option<String>> {
    let cfg = config::load_config()?;
    Ok(cfg.token)
}

/// Remove the stored token from the config file.
pub fn clear_token() -> Result<()> {
    let mut cfg = config::load_config()?;
    cfg.token = None;
    config::save_config(&cfg)?;
    debug!("Token cleared from config file");
    Ok(())
}

/// Retrieve the stored token, returning an error if none is found.
///
/// This is the authoritative way for commands to obtain the token; it checks
/// the environment variable `YUNXIAO_CLI_TOKEN` before falling back to the
/// config file.
pub fn require_token() -> Result<String> {
    // Check env var first
    if let Ok(t) = std::env::var("YUNXIAO_CLI_TOKEN") {
        if !t.is_empty() {
            return Ok(t);
        }
    }

    // Then config file
    let cfg = config::load_config()?;
    cfg.token.filter(|t| !t.is_empty()).ok_or_else(|| {
        CliError::Auth(
            "No API token found. Run `yunxiao auth login` or set YUNXIAO_CLI_TOKEN.".into(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn require_token_from_env() {
        std::env::set_var("YUNXIAO_CLI_TOKEN", "env-test-token");
        let token = require_token();
        assert!(token.is_ok());
        assert_eq!(token.unwrap(), "env-test-token");
        std::env::remove_var("YUNXIAO_CLI_TOKEN");
    }

    #[test]
    fn require_token_empty_env_var_is_rejected() {
        std::env::set_var("YUNXIAO_CLI_TOKEN", "");
        // It should fall through to config file, not return empty string
        let token = require_token();
        // This may succeed or fail depending on config file presence,
        // but it should never return an empty string
        if let Ok(t) = token {
            assert!(!t.is_empty());
        }
        // If Err, that's expected when no config file token either
        std::env::remove_var("YUNXIAO_CLI_TOKEN");
    }

    #[test]
    fn get_token_returns_option() {
        let result = get_token();
        assert!(result.is_ok());
        // Returns Some or None depending on config state
    }
}
