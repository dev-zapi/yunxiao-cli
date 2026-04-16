//! Work-item sub-operations for `projex workitems`.

use clap::{Args, Subcommand};
use serde_json::json;

use super::condition::ConditionBuilder;
use super::{
    format_type_to_api, parse_dynamic_fields, print_pagination_info, require_org,
    resolve_description, DescriptionFormat,
};
use crate::client::ApiClient;
use crate::config::types::OutputFormat;
use crate::error::Result;
use crate::output;

/// Arguments for `projex workitems`.
#[derive(Debug, Args)]
pub struct WorkitemsArgs {
    #[command(subcommand)]
    pub command: WorkitemsCmds,
}

/// Work-item operations.
#[derive(Debug, Subcommand)]
pub enum WorkitemsCmds {
    /// Search work items in a project space.
    Search(WiSearchArgs),
    /// Get a single work item.
    Get(WiGetArgs),
    /// Create a new work item.
    Create(WiCreateArgs),
    /// Update an existing work item.
    Update(WiUpdateArgs),
    /// List work-item types in a space.
    Types(WiTypesArgs),
    /// Get field configuration for a work-item type.
    Fields(WiFieldsArgs),
    /// Manage work-item comments.
    Comments(WiCommentsArgs),
    /// Manage work-item attachments.
    Attachments(WiAttachmentsArgs),
    /// Get workflow information for a work item.
    Flow(WiFlowArgs),
}

/// Arguments for `projex workitems search`.
#[derive(Debug, Args)]
pub struct WiSearchArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Work-item category (e.g. Req, Task, Bug). Get via: yunxiao projex workitems types --space-id <SPACE_ID>
    #[arg(long)]
    pub category: String,
    /// Optional keyword filter.
    #[arg(long)]
    pub keyword: Option<String>,
    /// Filter by serial number (e.g. PROJ-123).
    #[arg(long)]
    pub serial_number: Option<String>,
    /// Filter by version ID. Get via: yunxiao projex versions list --space-id <SPACE_ID>
    #[arg(long)]
    pub version_id: Option<String>,
    /// Filter by sprint ID. Get via: yunxiao projex sprints list --space-id <SPACE_ID>
    #[arg(long)]
    pub sprint_id: Option<String>,
    /// Page size.
    #[arg(long, default_value = "20")]
    pub page_size: u32,
    /// Page number.
    #[arg(long, default_value = "1")]
    pub page: u32,
}

/// Arguments for `projex workitems get`.
#[derive(Debug, Args)]
pub struct WiGetArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Work item ID. Get via: yunxiao projex workitems search --space-id <SPACE_ID> --category <CATEGORY>
    #[arg(long)]
    pub workitem_id: String,
}

/// Arguments for `projex workitems create`.
#[derive(Debug, Args)]
pub struct WiCreateArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Category (Req, Task, Bug, etc.). Get via: yunxiao projex workitems types --space-id <SPACE_ID>
    #[arg(long)]
    pub category: String,
    /// Work-item subject / title.
    #[arg(long)]
    pub subject: String,
    /// Assignee user ID. Get via: yunxiao org members list --org-id <ORG_ID>
    #[arg(long)]
    pub assignee: Option<String>,
    /// Sprint ID. Get via: yunxiao projex sprints list --space-id <SPACE_ID>
    #[arg(long)]
    pub sprint_id: Option<String>,
    /// Priority. Get via: yunxiao projex workitems fields --space-id <SPACE_ID> --type-id <TYPE_ID>
    #[arg(long)]
    pub priority: Option<String>,
    /// Work item description (optional, directly input).
    #[arg(long)]
    pub description: Option<String>,
    /// Work item description file path (optional, read from file).
    #[arg(long)]
    pub description_file: Option<String>,
    /// Description format: text (richtext) or markdown (default: markdown).
    #[arg(long, value_enum, default_value = "markdown")]
    pub description_format: DescriptionFormat,
    /// Dynamic field in format "fieldId=value", can be used multiple times.
    /// Use "yunxiao projex workitems fields --space-id <SPACE_ID> --type-id <TYPE_ID>" to get available field IDs.
    #[arg(long = "field")]
    pub fields: Vec<String>,
}

/// Arguments for `projex workitems update`.
#[derive(Debug, Args)]
pub struct WiUpdateArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Work item ID. Get via: yunxiao projex workitems search --space-id <SPACE_ID> --category <CATEGORY>
    #[arg(long)]
    pub workitem_id: String,
    /// New subject (optional).
    #[arg(long)]
    pub subject: Option<String>,
    /// New assignee user ID. Get via: yunxiao org members list --org-id <ORG_ID>
    #[arg(long)]
    pub assignee: Option<String>,
    /// New status. Get via: yunxiao projex workitems fields --space-id <SPACE_ID> --type-id <TYPE_ID>
    #[arg(long)]
    pub status: Option<String>,
    /// New priority. Get via: yunxiao projex workitems fields --space-id <SPACE_ID> --type-id <TYPE_ID>
    #[arg(long)]
    pub priority: Option<String>,
    /// New description (optional, directly input).
    #[arg(long)]
    pub description: Option<String>,
    /// New description file path (optional, read from file).
    #[arg(long)]
    pub description_file: Option<String>,
    /// New description format: text (richtext) or markdown.
    #[arg(long, value_enum)]
    pub description_format: Option<DescriptionFormat>,
    /// Dynamic field in format "fieldId=value", can be used multiple times.
    /// Use "yunxiao projex workitems fields --space-id <SPACE_ID> --type-id <TYPE_ID>" to get available field IDs.
    #[arg(long = "field")]
    pub fields: Vec<String>,
}

/// Arguments for `projex workitems types`.
#[derive(Debug, Args)]
pub struct WiTypesArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Filter by work-item category (e.g. Req, Task, Bug).
    #[arg(long)]
    pub category: Option<String>,
    /// Filter results locally by type name (case-insensitive substring).
    #[arg(long)]
    pub keyword: Option<String>,
}

/// Arguments for `projex workitems fields`.
#[derive(Debug, Args)]
pub struct WiFieldsArgs {
    /// Project ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub project_id: String,
    /// Work-item type ID. Get via: yunxiao projex workitems types --space-id <SPACE_ID>
    #[arg(long)]
    pub type_id: String,
}

/// Arguments for `projex workitems comments`.
#[derive(Debug, Args)]
pub struct WiCommentsArgs {
    #[command(subcommand)]
    pub command: WiCommentsCmds,
}

/// Comment operations.
#[derive(Debug, Subcommand)]
pub enum WiCommentsCmds {
    /// List comments on a work item.
    List(WiCommentsListArgs),
    /// Add a comment to a work item.
    Create(WiCommentsCreateArgs),
}

/// Arguments for comment listing.
#[derive(Debug, Args)]
pub struct WiCommentsListArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Work item ID. Get via: yunxiao projex workitems search --space-id <SPACE_ID> --category <CATEGORY>
    #[arg(long)]
    pub workitem_id: String,
}

/// Arguments for comment creation.
#[derive(Debug, Args)]
pub struct WiCommentsCreateArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Work item ID. Get via: yunxiao projex workitems search --space-id <SPACE_ID> --category <CATEGORY>
    #[arg(long)]
    pub workitem_id: String,
    /// Comment content.
    #[arg(long)]
    pub content: String,
}

/// Arguments for `projex workitems attachments`.
#[derive(Debug, Args)]
pub struct WiAttachmentsArgs {
    #[command(subcommand)]
    pub command: WiAttachmentsCmds,
}

/// Attachment operations.
#[derive(Debug, Subcommand)]
pub enum WiAttachmentsCmds {
    /// List attachments on a work item.
    List(WiAttachmentsListArgs),
}

/// Arguments for attachment listing.
#[derive(Debug, Args)]
pub struct WiAttachmentsListArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Work item ID. Get via: yunxiao projex workitems search --space-id <SPACE_ID> --category <CATEGORY>
    #[arg(long)]
    pub workitem_id: String,
}

/// Arguments for `projex workitems flow`.
#[derive(Debug, Args)]
pub struct WiFlowArgs {
    /// Work item ID. Get via: yunxiao projex workitems search --space-id <SPACE_ID> --category <CATEGORY>
    #[arg(long)]
    pub workitem_id: Option<String>,
    /// Project space ID. Required when using --type-id. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: Option<String>,
    /// Work-item type ID. Required when using --space-id. Get via: yunxiao projex workitems types --space-id <SPACE_ID>
    #[arg(long)]
    pub type_id: Option<String>,
}

/// Execute work-item sub-operations.
pub(super) async fn exec_workitems(
    args: &WorkitemsArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        WorkitemsCmds::Search(s) => exec_workitems_search(s, oid, client, format).await?,
        WorkitemsCmds::Get(g) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/projex/organizations/{oid}/workitems/{}",
                        g.workitem_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        WorkitemsCmds::Create(c) => {
            let mut body = json!({
                "category": c.category,
                "subject": c.subject,
                "spaceId": c.space_id,
            });
            if let Some(ref assignee) = c.assignee {
                body["assignedTo"] = json!(assignee);
            }
            if let Some(ref sid) = c.sprint_id {
                body["sprint"] = json!(sid);
            }
            if let Some(ref prio) = c.priority {
                body["priority"] = json!(prio);
            }

            let desc = resolve_description(c.description.as_ref(), c.description_file.as_ref())?;
            if let Some(ref content) = desc {
                body["description"] = json!(content);
                body["formatType"] = json!(format_type_to_api(c.description_format));
            }

            for (key, value) in parse_dynamic_fields(&c.fields) {
                if key != "description" && key != "formatType" {
                    body[key] = json!(value);
                }
            }

            let data = client
                .post(
                    &format!("/oapi/v1/projex/organizations/{oid}/workitems"),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        WorkitemsCmds::Update(u) => {
            let mut body = json!({});
            if let Some(ref s) = u.subject {
                body["subject"] = json!(s);
            }
            if let Some(ref a) = u.assignee {
                body["assignedTo"] = json!(a);
            }
            if let Some(ref st) = u.status {
                body["status"] = json!(st);
            }
            if let Some(ref p) = u.priority {
                body["priority"] = json!(p);
            }

            let desc = resolve_description(u.description.as_ref(), u.description_file.as_ref())?;
            if let Some(ref content) = desc {
                body["description"] = json!(content);
                if let Some(fmt) = u.description_format {
                    body["formatType"] = json!(format_type_to_api(fmt));
                }
            }

            for (key, value) in parse_dynamic_fields(&u.fields) {
                if key != "description" && key != "formatType" {
                    body[key] = json!(value);
                }
            }

            let data = client
                .put(
                    &format!(
                        "/oapi/v1/projex/organizations/{oid}/workitems/{}",
                        u.workitem_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        WorkitemsCmds::Types(t) => {
            let mut params: Vec<(&str, &str)> = Vec::new();
            let path = if let Some(ref c) = t.category {
                params.push(("category", c.as_str()));
                format!(
                    "/oapi/v1/projex/organizations/{oid}/projects/{}/workitemTypes",
                    t.space_id
                )
            } else {
                format!("/oapi/v1/projex/organizations/{oid}/workitemTypes")
            };

            let data = client.get(&path, &params).await?;

            let filtered = if let Some(ref kw) = t.keyword {
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
        WorkitemsCmds::Fields(f) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/projex/organizations/{oid}/projects/{}/workitemTypes/{}/fields",
                        f.project_id, f.type_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        WorkitemsCmds::Comments(c) => match &c.command {
            WiCommentsCmds::List(l) => {
                let data = client
                    .get(
                        &format!(
                            "/oapi/v1/projex/organizations/{oid}/workitems/{}/comments",
                            l.workitem_id
                        ),
                        &[],
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
            WiCommentsCmds::Create(cr) => {
                let body = json!({"content": cr.content});
                let data = client
                    .post(
                        &format!(
                            "/oapi/v1/projex/organizations/{oid}/workitems/{}/comments",
                            cr.workitem_id
                        ),
                        &body,
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
        },
        WorkitemsCmds::Attachments(a) => match &a.command {
            WiAttachmentsCmds::List(l) => {
                let data = client
                    .get(
                        &format!(
                            "/oapi/v1/projex/organizations/{oid}/workitems/{}/attachments",
                            l.workitem_id
                        ),
                        &[],
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
        },
        WorkitemsCmds::Flow(f) => {
            if let Some(ref workitem_id) = f.workitem_id {
                let data = client
                    .get(
                        &format!(
                            "/oapi/v1/projex/organizations/{oid}/workitems/{}/workflow",
                            workitem_id
                        ),
                        &[],
                    )
                    .await?;
                output::print_output(&data, format)?;
            } else if let (Some(ref space_id), Some(ref type_id)) = (&f.space_id, &f.type_id) {
                let data = client
                    .get(
                        &format!(
                            "/oapi/v1/projex/organizations/{oid}/projects/{}/workitemTypes/{}/workflows",
                            space_id, type_id
                        ),
                        &[],
                    )
                    .await?;
                output::print_output(&data, format)?;
            } else {
                return Err(crate::error::CliError::Config(
                    "Either --workitem-id or both --space-id and --type-id must be provided."
                        .into(),
                ));
            }
        }
    }
    Ok(())
}

/// Execute work-item search.
///
/// API docs: <https://help.aliyun.com/zh/yunxiao/developer-reference/searchworkitems>
async fn exec_workitems_search(
    s: &WiSearchArgs,
    oid: &str,
    client: &ApiClient,
    format: &OutputFormat,
) -> Result<()> {
    let conditions_str = ConditionBuilder::new()
        .opt_string_contains("subject", s.keyword.as_deref())
        .opt_string_contains("serialNumber", s.serial_number.as_deref())
        .opt_multi_list_contains("version", "version", s.version_id.as_deref())
        .opt_list_contains("sprint", "sprint", s.sprint_id.as_deref())
        .build();

    let mut body = json!({
        "category": s.category,
        "spaceId": s.space_id,
        "page": s.page,
        "perPage": s.page_size,
    });

    if let Some(conds) = conditions_str {
        body["conditions"] = json!(conds);
    }

    let resp = client
        .post_with_headers(
            &format!("/oapi/v1/projex/organizations/{oid}/workitems:search"),
            &body,
        )
        .await?;

    print_pagination_info(&resp.headers);
    output::print_output(&resp.body, format)?;
    Ok(())
}
