//! Sprint sub-operations for `projex sprints`.

use clap::{Args, Subcommand};
use serde_json::json;

use super::require_org;
use crate::client::ApiClient;
use crate::config::types::OutputFormat;
use crate::error::Result;
use crate::output;

/// Arguments for `projex sprints`.
#[derive(Debug, Args)]
pub struct SprintsArgs {
    #[command(subcommand)]
    pub command: SprintsCmds,
}

/// Sprint operations.
#[derive(Debug, Subcommand)]
pub enum SprintsCmds {
    /// List sprints in a space.
    List(SprintsListArgs),
    /// Get sprint details.
    Get(SprintsGetArgs),
    /// Create a new sprint.
    Create(SprintsCreateArgs),
    /// Update an existing sprint.
    Update(SprintsUpdateArgs),
}

/// Arguments for `projex sprints list`.
#[derive(Debug, Args)]
pub struct SprintsListArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
}

/// Arguments for `projex sprints get`.
#[derive(Debug, Args)]
pub struct SprintsGetArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Sprint ID. Get via: yunxiao projex sprints list --space-id <SPACE_ID>
    #[arg(long)]
    pub sprint_id: String,
}

/// Arguments for `projex sprints create`.
#[derive(Debug, Args)]
pub struct SprintsCreateArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Sprint name.
    #[arg(long)]
    pub name: String,
    /// Start date (YYYY-MM-DD, optional).
    #[arg(long)]
    pub start_date: Option<String>,
    /// End date (YYYY-MM-DD, optional).
    #[arg(long)]
    pub end_date: Option<String>,
}

/// Arguments for `projex sprints update`.
#[derive(Debug, Args)]
pub struct SprintsUpdateArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Sprint ID. Get via: yunxiao projex sprints list --space-id <SPACE_ID>
    #[arg(long)]
    pub sprint_id: String,
    /// New name (optional).
    #[arg(long)]
    pub name: Option<String>,
    /// New start date (optional).
    #[arg(long)]
    pub start_date: Option<String>,
    /// New end date (optional).
    #[arg(long)]
    pub end_date: Option<String>,
    /// New status (optional).
    #[arg(long)]
    pub status: Option<String>,
}

/// Execute sprint sub-operations.
pub(super) async fn exec_sprints(
    args: &SprintsArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        SprintsCmds::List(l) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/projex/organizations/{oid}/projects/{}/sprints",
                        l.space_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        SprintsCmds::Get(g) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/projex/organizations/{oid}/projects/{}/sprints/{}",
                        g.space_id, g.sprint_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        SprintsCmds::Create(c) => {
            let mut body = json!({"name": c.name});
            if let Some(ref sd) = c.start_date {
                body["startDate"] = json!(sd);
            }
            if let Some(ref ed) = c.end_date {
                body["endDate"] = json!(ed);
            }
            let data = client
                .post(
                    &format!(
                        "/oapi/v1/projex/organizations/{oid}/projects/{}/sprints",
                        c.space_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        SprintsCmds::Update(u) => {
            let mut body = json!({});
            if let Some(ref n) = u.name {
                body["name"] = json!(n);
            }
            if let Some(ref sd) = u.start_date {
                body["startDate"] = json!(sd);
            }
            if let Some(ref ed) = u.end_date {
                body["endDate"] = json!(ed);
            }
            if let Some(ref s) = u.status {
                body["status"] = json!(s);
            }
            let data = client
                .put(
                    &format!(
                        "/oapi/v1/projex/organizations/{oid}/projects/{}/sprints/{}",
                        u.space_id, u.sprint_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}
