//! `packages` subcommand – package repository management.
//!
//! Manages artifact repositories and individual artifacts via the
//! Yunxiao Packages API.

use clap::{Args, Subcommand};

use crate::client::ApiClient;
use crate::config;
use crate::config::types::OutputFormat;
use crate::error::Result;
use crate::output;

/// Arguments for the `packages` subcommand.
#[derive(Debug, Args)]
pub struct PackagesArgs {
    #[command(subcommand)]
    pub command: PackagesCommands,
}

/// Top-level packages operations.
#[derive(Debug, Subcommand)]
pub enum PackagesCommands {
    /// Manage package repositories.
    Repos(PkgReposArgs),
    /// Manage artifacts within a repository.
    Artifacts(ArtifactsArgs),
}

// ─────────────────────────── Repos ──────────────────────────────────────

/// Arguments for `packages repos`.
#[derive(Debug, Args)]
pub struct PkgReposArgs {
    #[command(subcommand)]
    pub command: PkgReposCmds,
}

/// Package repository operations.
#[derive(Debug, Subcommand)]
pub enum PkgReposCmds {
    /// List package repositories.
    List(PkgReposListArgs),
}

/// Arguments for `packages repos list`.
#[derive(Debug, Args)]
pub struct PkgReposListArgs {
    /// Filter by repository type (e.g. maven, npm, docker, generic).
    #[arg(long, name = "type")]
    pub repo_type: Option<String>,
}

// ─────────────────────────── Artifacts ──────────────────────────────────

/// Arguments for `packages artifacts`.
#[derive(Debug, Args)]
pub struct ArtifactsArgs {
    #[command(subcommand)]
    pub command: ArtifactsCmds,
}

/// Artifact operations.
#[derive(Debug, Subcommand)]
pub enum ArtifactsCmds {
    /// List artifacts in a repository.
    List(ArtifactsListArgs),
    /// Get artifact details.
    Get(ArtifactsGetArgs),
}

/// Arguments for `packages artifacts list`.
#[derive(Debug, Args)]
pub struct ArtifactsListArgs {
    /// Repository ID.
    #[arg(long)]
    pub repo_id: String,
    /// Search keyword (optional).
    #[arg(long)]
    pub keyword: Option<String>,
    /// Page number.
    #[arg(long, default_value = "1")]
    pub page: u32,
    /// Results per page.
    #[arg(long, default_value = "20")]
    pub per_page: u32,
}

/// Arguments for `packages artifacts get`.
#[derive(Debug, Args)]
pub struct ArtifactsGetArgs {
    /// Repository ID.
    #[arg(long)]
    pub repo_id: String,
    /// Artifact ID.
    #[arg(long)]
    pub artifact_id: String,
}

// ─────────────────────────── Execute ────────────────────────────────────

/// Execute the `packages` subcommand tree.
pub async fn execute(
    args: &PackagesArgs,
    format: &OutputFormat,
    cli_token: Option<&str>,
    cli_endpoint: Option<&str>,
    cli_timeout: Option<u64>,
    cli_org_id: Option<&str>,
) -> Result<()> {
    let token = config::resolve_token(cli_token)?;
    let endpoint = config::resolve_endpoint(cli_endpoint);
    let timeout = config::resolve_timeout(cli_timeout);
    let org_id = config::resolve_org_id(cli_org_id);
    let client = ApiClient::new(&token, &endpoint, timeout)?;

    match &args.command {
        PackagesCommands::Repos(r) => exec_repos(r, &client, &org_id, format).await,
        PackagesCommands::Artifacts(a) => exec_artifacts(a, &client, &org_id, format).await,
    }
}

/// Helper: require org ID.
fn require_org(org_id: &Option<String>) -> Result<&str> {
    org_id.as_deref().ok_or_else(|| {
        crate::error::CliError::Config(
            "Organization ID required. Set via --org-id, YUNXIAO_CLI_ORG_ID, or config.".into(),
        )
    })
}

/// Execute repo sub-operations.
async fn exec_repos(
    args: &PkgReposArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        PkgReposCmds::List(l) => {
            let mut params: Vec<(&str, &str)> = vec![];
            if let Some(ref t) = l.repo_type {
                params.push(("type", t.as_str()));
            }
            let data = client
                .get(
                    &format!("/oapi/v1/packages/organizations/{oid}/repositories"),
                    &params,
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

/// Execute artifact sub-operations.
async fn exec_artifacts(
    args: &ArtifactsArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        ArtifactsCmds::List(l) => {
            let page = l.page.to_string();
            let per_page = l.per_page.to_string();
            let mut params: Vec<(&str, &str)> = vec![
                ("page", page.as_str()),
                ("perPage", per_page.as_str()),
            ];
            if let Some(ref kw) = l.keyword {
                params.push(("keyword", kw.as_str()));
            }
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/packages/organizations/{oid}/repositories/{}/artifacts",
                        l.repo_id
                    ),
                    &params,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        ArtifactsCmds::Get(g) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/packages/organizations/{oid}/repositories/{}/artifacts/{}",
                        g.repo_id, g.artifact_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}
