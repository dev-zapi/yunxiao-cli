//! Version sub-operations for `projex versions`.

use clap::{Args, Subcommand};
use serde_json::json;

use super::require_org;
use crate::client::ApiClient;
use crate::config::types::OutputFormat;
use crate::error::Result;
use crate::output;

/// Arguments for `projex versions`.
#[derive(Debug, Args)]
pub struct VersionsArgs {
    #[command(subcommand)]
    pub command: VersionsCmds,
}

/// Version / release operations.
#[derive(Debug, Subcommand)]
pub enum VersionsCmds {
    /// List versions in a space.
    List(VersionsListArgs),
    /// Create a new version.
    Create(VersionsCreateArgs),
    /// Update an existing version.
    Update(VersionsUpdateArgs),
    /// Delete a version.
    Delete(VersionsDeleteArgs),
}

/// Arguments for `projex versions list`.
#[derive(Debug, Args)]
pub struct VersionsListArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
}

/// Arguments for `projex versions create`.
#[derive(Debug, Args)]
pub struct VersionsCreateArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Version name.
    #[arg(long)]
    pub name: String,
    /// Description (optional).
    #[arg(long)]
    pub description: Option<String>,
}

/// Arguments for `projex versions update`.
#[derive(Debug, Args)]
pub struct VersionsUpdateArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Version ID. Get via: yunxiao projex versions list --space-id <SPACE_ID>
    #[arg(long)]
    pub version_id: String,
    /// New name (optional).
    #[arg(long)]
    pub name: Option<String>,
    /// New description (optional).
    #[arg(long)]
    pub description: Option<String>,
    /// New status (optional).
    #[arg(long)]
    pub status: Option<String>,
}

/// Arguments for `projex versions delete`.
#[derive(Debug, Args)]
pub struct VersionsDeleteArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Version ID. Get via: yunxiao projex versions list --space-id <SPACE_ID>
    #[arg(long)]
    pub version_id: String,
}

/// Execute version sub-operations.
pub(super) async fn exec_versions(
    args: &VersionsArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        VersionsCmds::List(l) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/projex/organizations/{oid}/projects/{}/versions",
                        l.space_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        VersionsCmds::Create(c) => {
            let mut body = json!({"name": c.name});
            if let Some(ref d) = c.description {
                body["description"] = json!(d);
            }
            let data = client
                .post(
                    &format!(
                        "/oapi/v1/projex/organizations/{oid}/projects/{}/versions",
                        c.space_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        VersionsCmds::Update(u) => {
            let mut body = json!({});
            if let Some(ref n) = u.name {
                body["name"] = json!(n);
            }
            if let Some(ref d) = u.description {
                body["description"] = json!(d);
            }
            if let Some(ref s) = u.status {
                body["status"] = json!(s);
            }
            let data = client
                .put(
                    &format!(
                        "/oapi/v1/projex/organizations/{oid}/projects/{}/versions/{}",
                        u.space_id, u.version_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        VersionsCmds::Delete(d) => {
            let data = client
                .delete(
                    &format!(
                        "/oapi/v1/projex/organizations/{oid}/projects/{}/versions/{}",
                        d.space_id, d.version_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}
