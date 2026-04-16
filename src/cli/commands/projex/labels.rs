//! Label sub-operations for `projex labels`.

use clap::{Args, Subcommand};
use serde_json::json;

use super::require_org;
use crate::client::ApiClient;
use crate::config::types::OutputFormat;
use crate::error::Result;
use crate::output;

/// Arguments for `projex labels`.
#[derive(Debug, Args)]
pub struct LabelsArgs {
    #[command(subcommand)]
    pub command: LabelsCmds,
}

/// Label operations.
#[derive(Debug, Subcommand)]
pub enum LabelsCmds {
    /// List labels in a project.
    List(LabelListArgs),
    /// Create a new label.
    Create(LabelCreateArgs),
    /// Update an existing label.
    Update(LabelUpdateArgs),
}

/// Arguments for `projex labels list`.
#[derive(Debug, Args)]
pub struct LabelListArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Filter results locally by label name (case-insensitive substring).
    #[arg(long)]
    pub keyword: Option<String>,
}

/// Arguments for `projex labels create`.
#[derive(Debug, Args)]
pub struct LabelCreateArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Label name.
    #[arg(long)]
    pub name: String,
    /// Label color (e.g., #A773E0, #FF0000).
    #[arg(long)]
    pub color: String,
}

/// Arguments for `projex labels update`.
#[derive(Debug, Args)]
pub struct LabelUpdateArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Label ID. Get via: yunxiao projex labels list --space-id <SPACE_ID>
    #[arg(long)]
    pub label_id: String,
    /// New label name (optional).
    #[arg(long)]
    pub name: Option<String>,
    /// New label color (e.g., #A773E0, #FF0000) (optional).
    #[arg(long)]
    pub color: Option<String>,
}

/// Execute label sub-operations.
pub(super) async fn exec_labels(
    args: &LabelsArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        LabelsCmds::List(l) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/projex/organizations/{oid}/projects/{}/labels",
                        l.space_id
                    ),
                    &[],
                )
                .await?;

            let filtered = if let Some(ref kw) = l.keyword {
                let kw_lower = kw.to_lowercase();
                if let Some(arr) = data.as_array() {
                    let kept: Vec<serde_json::Value> = arr
                        .iter()
                        .filter(|item| {
                            item.get("name")
                                .and_then(|v| v.as_str())
                                .map(|name| name.to_lowercase().contains(&kw_lower))
                                .unwrap_or(false)
                        })
                        .cloned()
                        .collect();
                    serde_json::Value::Array(kept)
                } else {
                    data
                }
            } else {
                data
            };

            output::print_output(&filtered, format)?;
        }
        LabelsCmds::Create(c) => {
            let body = json!({
                "name": c.name,
                "color": c.color,
            });
            let data = client
                .post(
                    &format!(
                        "/oapi/v1/projex/organizations/{oid}/projects/{}/labels",
                        c.space_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        LabelsCmds::Update(u) => {
            let mut body = json!({});
            if let Some(ref n) = u.name {
                body["name"] = json!(n);
            }
            if let Some(ref c) = u.color {
                body["color"] = json!(c);
            }
            let data = client
                .put(
                    &format!(
                        "/oapi/v1/projex/organizations/{oid}/projects/{}/labels/{}",
                        u.space_id, u.label_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}
