//! Effort sub-operations for `projex efforts`.

use clap::{Args, Subcommand};
use serde_json::json;

use super::require_org;
use crate::client::ApiClient;
use crate::config::types::OutputFormat;
use crate::error::Result;
use crate::output;

/// Arguments for `projex efforts`.
#[derive(Debug, Args)]
pub struct EffortsArgs {
    #[command(subcommand)]
    pub command: EffortsCmds,
}

/// Effort / work-hour operations.
#[derive(Debug, Subcommand)]
pub enum EffortsCmds {
    /// List effort records.
    List(EffortsListArgs),
    /// Log effort against a work item.
    Create(EffortsCreateArgs),
}

/// Arguments for `projex efforts list`.
#[derive(Debug, Args)]
pub struct EffortsListArgs {
    /// Start date filter (YYYY-MM-DD, optional).
    #[arg(long)]
    pub start_date: Option<String>,
    /// End date filter (YYYY-MM-DD, optional).
    #[arg(long)]
    pub end_date: Option<String>,
}

/// Arguments for `projex efforts create`.
#[derive(Debug, Args)]
pub struct EffortsCreateArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Work item ID. Get via: yunxiao projex workitems search --space-id <SPACE_ID> --category <CATEGORY>
    #[arg(long)]
    pub workitem_id: String,
    /// Duration in hours.
    #[arg(long)]
    pub duration: f64,
    /// Description (optional).
    #[arg(long)]
    pub description: Option<String>,
}

/// Execute effort sub-operations.
pub(super) async fn exec_efforts(
    args: &EffortsArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        EffortsCmds::List(l) => {
            let mut params: Vec<(&str, &str)> = vec![];
            if let Some(ref sd) = l.start_date {
                params.push(("startDate", sd.as_str()));
            }
            if let Some(ref ed) = l.end_date {
                params.push(("endDate", ed.as_str()));
            }
            let data = client
                .get(
                    &format!("/oapi/v1/projex/organizations/{oid}/effortRecords"),
                    &params,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        EffortsCmds::Create(c) => {
            let mut body = json!({
                "workitemId": c.workitem_id,
                "duration": c.duration,
            });
            if let Some(ref d) = c.description {
                body["description"] = json!(d);
            }
            let data = client
                .post(
                    &format!(
                        "/oapi/v1/projex/organizations/{oid}/workitems/{}/effortRecords",
                        c.workitem_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}
