//! `auth` subcommand – authentication management.
//!
//! Provides token login / logout, status display, and a `whoami` call
//! that queries the current user from the API.

use clap::{Args, Subcommand};
use serde_json::json;

use crate::auth;
use crate::client::ApiClient;
use crate::config;
use crate::config::types::OutputFormat;
use crate::error::Result;
use crate::output;

/// Arguments for the `auth` subcommand.
#[derive(Debug, Args)]
pub struct AuthArgs {
    /// Auth sub-operation to perform.
    #[command(subcommand)]
    pub command: AuthCommands,
}

/// Available authentication operations.
#[derive(Debug, Subcommand)]
pub enum AuthCommands {
    /// Save a personal access token for API authentication.
    Login(LoginArgs),
    /// Remove the saved token.
    Logout,
    /// Show the current authentication status (token present, masked).
    Status,
    /// Query the API for the currently authenticated user.
    Whoami,
}

/// Arguments for `auth login`.
#[derive(Debug, Args)]
pub struct LoginArgs {
    /// Personal access token. If omitted, reads from stdin.
    #[arg(long)]
    pub token: Option<String>,
}

/// Execute the `auth` subcommand tree.
pub async fn execute(
    args: &AuthArgs,
    format: &OutputFormat,
    cli_token: Option<&str>,
    cli_endpoint: Option<&str>,
    cli_timeout: Option<u64>,
) -> Result<()> {
    match &args.command {
        AuthCommands::Login(login) => cmd_login(login).await,
        AuthCommands::Logout => cmd_logout(format).await,
        AuthCommands::Status => cmd_status(format).await,
        AuthCommands::Whoami => cmd_whoami(format, cli_token, cli_endpoint, cli_timeout).await,
    }
}

/// `auth login` – save token to config file.
async fn cmd_login(args: &LoginArgs) -> Result<()> {
    let token = match &args.token {
        Some(t) => t.clone(),
        None => {
            // Read from stdin
            eprintln!("Paste your personal access token:");
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf)?;
            buf.trim().to_string()
        }
    };

    if token.is_empty() {
        return Err(crate::error::CliError::Auth(
            "Token cannot be empty.".into(),
        ));
    }

    auth::save_token(&token)?;
    eprintln!("Token saved successfully.");
    Ok(())
}

/// `auth logout` – clear stored token.
async fn cmd_logout(format: &OutputFormat) -> Result<()> {
    auth::clear_token()?;
    let data = json!({"status": "logged out", "message": "Token has been removed."});
    output::print_output(&data, format)?;
    Ok(())
}

/// `auth status` – display whether a token is stored and show a masked version.
async fn cmd_status(format: &OutputFormat) -> Result<()> {
    let token = auth::get_token()?;
    let data = match token {
        Some(t) => {
            let masked = mask_token(&t);
            json!({
                "authenticated": true,
                "token": masked,
                "source": "config file"
            })
        }
        None => {
            // Also check env var
            if let Ok(t) = std::env::var("YUNXIAO_CLI_TOKEN") {
                if !t.is_empty() {
                    let masked = mask_token(&t);
                    return output::print_output(
                        &json!({
                            "authenticated": true,
                            "token": masked,
                            "source": "environment variable"
                        }),
                        format,
                    );
                }
            }
            json!({
                "authenticated": false,
                "message": "No token found. Run `yunxiao auth login` to authenticate."
            })
        }
    };
    output::print_output(&data, format)?;
    Ok(())
}

/// `auth whoami` – call the API to get the current user.
async fn cmd_whoami(
    format: &OutputFormat,
    cli_token: Option<&str>,
    cli_endpoint: Option<&str>,
    cli_timeout: Option<u64>,
) -> Result<()> {
    let token = config::resolve_token(cli_token)?;
    let endpoint = config::resolve_endpoint(cli_endpoint);
    let timeout = config::resolve_timeout(cli_timeout);

    let client = ApiClient::new(&token, &endpoint, timeout)?;
    let data = client.get("/oapi/v1/platform/user", &[]).await?;
    output::print_output(&data, format)?;
    Ok(())
}

/// Mask a token, showing only the first 4 and last 4 characters.
fn mask_token(token: &str) -> String {
    let len = token.len();
    if len <= 8 {
        return "****".to_string();
    }
    let prefix = &token[..4];
    let suffix = &token[len - 4..];
    format!("{prefix}****{suffix}")
}
