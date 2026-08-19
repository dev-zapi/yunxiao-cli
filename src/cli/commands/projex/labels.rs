//! Label sub-operations for `projex labels`.

use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashSet;

use super::require_org;
use crate::cache::{delete_cache, read_cache_with_ttl, write_cache_with_ttl};
use crate::client::ApiClient;
use crate::config::types::OutputFormat;
use crate::error::{CliError, Result};
use crate::output;

const LABEL_CACHE_TTL_SECONDS: u64 = 300;

/// A label resolved from the project-space label directory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct ResolvedLabel {
    pub id: String,
    pub name: String,
}

fn label_cache_key(org_id: &str, space_id: &str) -> String {
    format!("labels_{org_id}_{space_id}")
}

fn labels_from_response(data: &serde_json::Value) -> Vec<ResolvedLabel> {
    let labels = data
        .as_array()
        .or_else(|| data.get("data").and_then(serde_json::Value::as_array));

    labels
        .into_iter()
        .flatten()
        .filter_map(|label| {
            Some(ResolvedLabel {
                id: label.get("id")?.as_str()?.to_string(),
                name: label.get("name")?.as_str()?.to_string(),
            })
        })
        .collect()
}

async fn get_cached_labels(
    client: &ApiClient,
    org_id: &str,
    space_id: &str,
) -> Result<Vec<ResolvedLabel>> {
    let cache_key = label_cache_key(org_id, space_id);
    if let Some(labels) = read_cache_with_ttl::<Vec<ResolvedLabel>>(&cache_key)? {
        return Ok(labels);
    }

    let data = client
        .get(
            &format!("/oapi/v1/projex/organizations/{org_id}/projects/{space_id}/labels"),
            &[],
        )
        .await?;
    let labels = labels_from_response(&data);
    write_cache_with_ttl(&cache_key, &labels, Some(LABEL_CACHE_TTL_SECONDS))?;
    Ok(labels)
}

/// Invalidate cached label data after a label validation error.
pub(super) fn invalidate_label_cache(org_id: &str, space_id: &str) -> Result<()> {
    delete_cache(&label_cache_key(org_id, space_id))
}

/// Return whether a value is a YunXiao label ID rather than a label name.
pub(super) fn is_label_id(value: &str) -> bool {
    value.len() == 26 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

/// Parse comma-separated label references without resolving them.
pub(super) fn parse_label_references(value: &str) -> Result<Vec<String>> {
    let references: Vec<String> = value
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect();

    if references.is_empty() {
        return Err(CliError::Config(
            "--labels requires at least one label ID or exact label name.".into(),
        ));
    }

    Ok(references)
}

fn available_label_names(labels: &[ResolvedLabel]) -> String {
    if labels.is_empty() {
        "(none)".into()
    } else {
        labels
            .iter()
            .map(|label| label.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// Resolve label IDs and exact names to unique label IDs in input order.
pub(super) async fn resolve_label_references(
    client: &ApiClient,
    org_id: &str,
    space_id: &str,
    references: &[String],
) -> Result<Vec<ResolvedLabel>> {
    let labels = get_cached_labels(client, org_id, space_id).await?;
    let mut resolved = Vec::new();
    let mut seen_ids = HashSet::new();

    for reference in references {
        let matches: Vec<&ResolvedLabel> = if is_label_id(reference) {
            labels
                .iter()
                .filter(|label| label.id == *reference)
                .collect()
        } else {
            labels
                .iter()
                .filter(|label| label.name == *reference)
                .collect()
        };

        let label = match matches.as_slice() {
            [label] => *label,
            [] if is_label_id(reference) => {
                return Err(CliError::Config(format!(
                    "Label ID '{reference}' does not exist in project space '{space_id}'. Available labels: {}",
                    available_label_names(&labels)
                )));
            }
            [] => {
                return Err(CliError::Config(format!(
                    "Label name '{reference}' does not exist in project space '{space_id}'. Label names are exact and case-sensitive. Available labels: {}",
                    available_label_names(&labels)
                )));
            }
            _ => {
                return Err(CliError::Config(format!(
                    "Label name '{reference}' is ambiguous in project space '{space_id}'. Matching IDs: {}",
                    matches
                        .iter()
                        .map(|label| label.id.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                )));
            }
        };

        if seen_ids.insert(label.id.clone()) {
            resolved.push(label.clone());
        }
    }

    Ok(resolved)
}

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
            invalidate_label_cache(oid, &c.space_id)?;
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
            invalidate_label_cache(oid, &u.space_id)?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_ids_use_the_fixed_hex_format() {
        assert!(is_label_id("e4995af971162ddb8faa7dc1a1"));
        assert!(is_label_id("E4995AF971162DDB8FAA7DC1A1"));
        assert!(!is_label_id("ready-for-agent"));
        assert!(!is_label_id("e4995af971162ddb8faa7dc1"));
    }

    #[test]
    fn label_references_trim_empty_values() {
        assert_eq!(
            parse_label_references(" ready-for-agent, e4995af971162ddb8faa7dc1a1, ").unwrap(),
            vec![
                "ready-for-agent".to_string(),
                "e4995af971162ddb8faa7dc1a1".to_string()
            ]
        );
        assert!(parse_label_references(" , ").is_err());
    }
}
