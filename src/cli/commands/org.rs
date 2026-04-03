//! `org` subcommand – organization and member management.
//!
//! Provides operations for querying organization info, listing members,
//! departments, and roles via the Yunxiao API.

use clap::{Args, Subcommand};

use crate::client::ApiClient;
use crate::config;
use crate::config::types::OutputFormat;
use crate::error::Result;
use crate::output;

/// Arguments for the `org` subcommand.
#[derive(Debug, Args)]
pub struct OrgArgs {
    /// Organization operation to perform.
    #[command(subcommand)]
    pub command: OrgCommands,
}

/// Available organization operations.
#[derive(Debug, Subcommand)]
pub enum OrgCommands {
    /// Get information about the current organization.
    Info,
    /// List all organizations the current user belongs to.
    List,
    /// Manage organization members.
    Members(MembersArgs),
    /// Manage organization departments.
    Departments(DepartmentsArgs),
    /// Manage organization roles.
    Roles(RolesArgs),
}

// ─────────────────────────── Members ────────────────────────────────────

/// Arguments for `org members`.
#[derive(Debug, Args)]
pub struct MembersArgs {
    /// Members sub-operation.
    #[command(subcommand)]
    pub command: MembersCommands,
}

/// Available member operations.
#[derive(Debug, Subcommand)]
pub enum MembersCommands {
    /// List organization members.
    List(MembersListArgs),
    /// Get a specific member by user ID.
    Get(MemberGetArgs),
    /// Search members by keyword.
    Search(MemberSearchArgs),
}

/// Arguments for `org members list`.
#[derive(Debug, Args)]
pub struct MembersListArgs {
    /// Page number (1-based).
    #[arg(long, default_value = "1")]
    pub page: u32,
    /// Number of results per page.
    #[arg(long, default_value = "20")]
    pub per_page: u32,
}

/// Arguments for `org members get`.
#[derive(Debug, Args)]
pub struct MemberGetArgs {
    /// User ID to look up.
    pub user_id: String,
}

/// Arguments for `org members search`.
#[derive(Debug, Args)]
pub struct MemberSearchArgs {
    /// Search query string (name or email).
    pub query: String,
}

// ─────────────────────────── Departments ────────────────────────────────

/// Arguments for `org departments`.
#[derive(Debug, Args)]
pub struct DepartmentsArgs {
    /// Departments sub-operation.
    #[command(subcommand)]
    pub command: DepartmentsCommands,
}

/// Available department operations.
#[derive(Debug, Subcommand)]
pub enum DepartmentsCommands {
    /// List all departments.
    List,
    /// Get a specific department by ID.
    Get(DepartmentGetArgs),
}

/// Arguments for `org departments get`.
#[derive(Debug, Args)]
pub struct DepartmentGetArgs {
    /// Department ID.
    pub id: String,
}

// ─────────────────────────── Roles ──────────────────────────────────────

/// Arguments for `org roles`.
#[derive(Debug, Args)]
pub struct RolesArgs {
    /// Roles sub-operation.
    #[command(subcommand)]
    pub command: RolesCommands,
}

/// Available role operations.
#[derive(Debug, Subcommand)]
pub enum RolesCommands {
    /// List all roles.
    List,
    /// Get a specific role by ID.
    Get(RoleGetArgs),
}

/// Arguments for `org roles get`.
#[derive(Debug, Args)]
pub struct RoleGetArgs {
    /// Role ID.
    pub id: String,
}

// ─────────────────────────── Execute ────────────────────────────────────

/// Execute the `org` subcommand tree.
pub async fn execute(
    args: &OrgArgs,
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
        OrgCommands::Info => {
            let path = match &org_id {
                Some(id) => format!("/oapi/v1/platform/organizations/{id}"),
                None => "/oapi/v1/platform/user".to_string(),
            };
            let data = client.get(&path, &[]).await?;
            output::print_output(&data, format)?;
        }
        OrgCommands::List => {
            let data = client.get("/oapi/v1/platform/organizations", &[]).await?;
            output::print_output(&data, format)?;
        }
        OrgCommands::Members(m) => match &m.command {
            MembersCommands::List(la) => {
                let oid = require_org_id(&org_id)?;
                let page = la.page.to_string();
                let per_page = la.per_page.to_string();
                let data = client
                    .get(
                        &format!("/oapi/v1/platform/organizations/{oid}/members"),
                        &[("page", page.as_str()), ("perPage", per_page.as_str())],
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
            MembersCommands::Get(g) => {
                let oid = require_org_id(&org_id)?;
                let data = client
                    .get(
                        &format!(
                            "/oapi/v1/platform/organizations/{oid}/members/{}",
                            g.user_id
                        ),
                        &[],
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
            MembersCommands::Search(s) => {
                let oid = require_org_id(&org_id)?;
                let body = serde_json::json!({"query": s.query});
                let data = client
                    .post(
                        &format!("/oapi/v1/platform/organizations/{oid}/members:search"),
                        &body,
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
        },
        OrgCommands::Departments(d) => match &d.command {
            DepartmentsCommands::List => {
                let oid = require_org_id(&org_id)?;
                let data = client
                    .get(
                        &format!("/oapi/v1/platform/organizations/{oid}/departments"),
                        &[],
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
            DepartmentsCommands::Get(g) => {
                let oid = require_org_id(&org_id)?;
                let data = client
                    .get(
                        &format!("/oapi/v1/platform/organizations/{oid}/departments/{}", g.id),
                        &[],
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
        },
        OrgCommands::Roles(r) => match &r.command {
            RolesCommands::List => {
                let oid = require_org_id(&org_id)?;
                let data = client
                    .get(&format!("/oapi/v1/platform/organizations/{oid}/roles"), &[])
                    .await?;
                output::print_output(&data, format)?;
            }
            RolesCommands::Get(g) => {
                let oid = require_org_id(&org_id)?;
                let data = client
                    .get(
                        &format!("/oapi/v1/platform/organizations/{oid}/roles/{}", g.id),
                        &[],
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
        },
    }
    Ok(())
}

/// Helper to require an organization ID, returning a friendly error if missing.
fn require_org_id(org_id: &Option<String>) -> Result<&str> {
    org_id.as_deref().ok_or_else(|| {
        crate::error::CliError::Config(
            "Organization ID required. Set via --org-id, YUNXIAO_CLI_ORG_ID, or `yunxiao config set organization_id <id>`.".into(),
        )
    })
}
