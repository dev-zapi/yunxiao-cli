//! Work-item sub-operations for `projex workitems`.

use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use tokio::time::{sleep, Instant};

use super::condition::ConditionBuilder;
use super::labels::{
    invalidate_label_cache, parse_label_references, resolve_label_references, ResolvedLabel,
};
use super::{
    format_type_to_api, parse_dynamic_fields, print_pagination_info, require_org,
    resolve_description, DescriptionFormat,
};
use crate::cache::{read_cache_with_ttl, write_cache_with_ttl};
use crate::client::ApiClient;
use crate::config::types::OutputFormat;
use crate::error::{CliError, Result};
use crate::output;

/// Standard fields that should be placed in the body top-level.
const STANDARD_FIELDS: &[&str] = &[
    "subject",
    "description",
    "assignedTo",
    "sprint",
    "spaceId",
    "workitemTypeId",
    "formatType",
    "status",
    "labels",
    "participants",
    "trackers",
    "verifier",
    "versions",
    "parentId",
];

const FIELD_CONFIG_CACHE_TTL_SECONDS: u64 = 3600;
const LABEL_RETRY_ERROR_CODES: &[&str] = &["InvaildData.Failed", "InvalidData.Failed"];
const POLL_DELAYS: &[Duration] = &[
    Duration::from_millis(250),
    Duration::from_millis(500),
    Duration::from_secs(1),
    Duration::from_secs(2),
    Duration::from_secs(4),
    Duration::from_secs(5),
];

/// Field configuration from the GetWorkitemTypeFieldConfig API.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FieldConfig {
    field_id: String,
    #[serde(default)]
    field_identifier: String,
    field_name: String,
    field_format: String,
    required: bool,
    #[serde(default)]
    default_value: Option<serde_json::Value>,
}

/// Cache key for field configurations.
fn field_config_cache_key(org_id: &str, space_id: &str, type_id: &str) -> String {
    format!("field_config_v2_{}_{}_{}", org_id, space_id, type_id)
}

/// Fetch field configurations from API or cache.
///
/// Cache TTL: 1 hour (3600 seconds).
async fn get_field_configs(
    client: &ApiClient,
    org_id: &str,
    space_id: &str,
    type_id: &str,
) -> Result<HashMap<String, FieldConfig>> {
    let cache_key = field_config_cache_key(org_id, space_id, type_id);

    // Try to read from cache first
    if let Some(cached) = read_cache_with_ttl::<HashMap<String, FieldConfig>>(&cache_key)? {
        return Ok(cached);
    }

    // Fetch from API
    let data = client
        .get(
            &format!(
                "/oapi/v1/projex/organizations/{org_id}/projects/{space_id}/workitemTypes/{type_id}/fields"
            ),
            &[],
        )
        .await?;

    log::debug!(
        "Field configs API response: {}",
        serde_json::to_string_pretty(&data).unwrap()
    );
    let mut configs = std::collections::HashMap::new();

    if let Some(fields) = data.as_array() {
        for field in fields {
            let field_id = field.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let field_identifier = field
                .get("fieldIdentifier")
                .and_then(|v| v.as_str())
                .unwrap_or(field_id);

            let config = FieldConfig {
                field_id: field_id.to_string(),
                field_identifier: field_identifier.to_string(),
                field_name: field
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                field_format: field
                    .get("format")
                    .and_then(|v| v.as_str())
                    .unwrap_or("string")
                    .to_string(),
                required: field
                    .get("required")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
                default_value: field
                    .get("defaultValue")
                    .filter(|value| !value.is_null())
                    .cloned(),
            };

            configs.insert(field_identifier.to_string(), config);
        }
    }

    // Write to cache with 1 hour TTL
    write_cache_with_ttl(&cache_key, &configs, Some(FIELD_CONFIG_CACHE_TTL_SECONDS))?;

    Ok(configs)
}

/// Parse comma-separated string into a JSON array.
fn parse_array_field(value: &str) -> serde_json::Value {
    let items: Vec<&str> = value.split(',').map(|s| s.trim()).collect();
    serde_json::Value::Array(items.iter().map(|s| json!(s)).collect())
}

fn label_ids(labels: &[ResolvedLabel]) -> Vec<String> {
    labels.iter().map(|label| label.id.clone()).collect()
}

fn label_details(labels: &[ResolvedLabel]) -> String {
    labels
        .iter()
        .map(|label| format!("{} ({})", label.name, label.id))
        .collect::<Vec<_>>()
        .join(", ")
}

fn validate_dynamic_field_overrides(
    fields: &[(String, String)],
    reserved_fields: &HashSet<&str>,
) -> Result<()> {
    for (field_id, _) in fields {
        if field_id == "labels" {
            return Err(CliError::Config(
                "Use --labels for labels; --field labels=... bypasses label validation and is not supported."
                    .into(),
            ));
        }
        if reserved_fields.contains(field_id.as_str()) {
            return Err(CliError::Config(format!(
                "Field '{field_id}' was supplied by both a dedicated option and --field. Use only one input source."
            )));
        }
    }
    Ok(())
}

fn required_field_hint(field_id: &str) -> String {
    match field_id {
        "assignedTo" => "--assignee <member.userId>".into(),
        "description" => "--description <text> or --description-file <path>".into(),
        "labels" => "--labels <label ID or exact label name>".into(),
        "priority" => "--priority <priority ID>".into(),
        "sprint" => "--sprint-id <sprint ID>".into(),
        _ => format!("--field {field_id}=<value>"),
    }
}

fn validate_required_create_fields(
    field_configs: &HashMap<String, FieldConfig>,
    provided_fields: &HashSet<String>,
) -> Result<()> {
    let mut missing: Vec<&FieldConfig> = field_configs
        .values()
        .filter(|field| {
            field.required
                && field.default_value.is_none()
                && !provided_fields.contains(&field.field_identifier)
        })
        .collect();
    missing.sort_by(|left, right| left.field_identifier.cmp(&right.field_identifier));

    if missing.is_empty() {
        return Ok(());
    }

    let details = missing
        .iter()
        .map(|field| {
            format!(
                "{} (fieldIdentifier: {}, id: {}, format: {}): {}",
                field.field_name,
                field.field_identifier,
                field.field_id,
                field.field_format,
                required_field_hint(&field.field_identifier)
            )
        })
        .collect::<Vec<_>>()
        .join("; ");
    Err(CliError::Config(format!(
        "Missing required work-item fields: {details}"
    )))
}

fn create_provided_fields(
    args: &WiCreateArgs,
    description: &Option<String>,
    dynamic_fields: &[(String, String)],
) -> HashSet<String> {
    let mut provided = dynamic_fields
        .iter()
        .map(|(field_id, _)| field_id.clone())
        .collect::<HashSet<_>>();
    provided.extend([
        "subject".to_string(),
        "spaceId".to_string(),
        "workitemTypeId".to_string(),
    ]);
    if args.assignee.is_some() {
        provided.insert("assignedTo".into());
    }
    if args.sprint_id.is_some() {
        provided.insert("sprint".into());
    }
    if args.labels.is_some() {
        provided.insert("labels".into());
    }
    if description.is_some() {
        provided.extend(["description".to_string(), "formatType".to_string()]);
    }
    if args.priority.is_some() {
        provided.insert("priority".into());
    }
    provided
}

fn create_reserved_fields(
    args: &WiCreateArgs,
    description: &Option<String>,
) -> HashSet<&'static str> {
    let mut reserved = HashSet::from(["subject", "spaceId", "workitemTypeId"]);
    if args.assignee.is_some() {
        reserved.insert("assignedTo");
    }
    if args.sprint_id.is_some() {
        reserved.insert("sprint");
    }
    if description.is_some() {
        reserved.extend(["description", "formatType"]);
    }
    if args.priority.is_some() {
        reserved.insert("priority");
    }
    reserved
}

fn update_reserved_fields(
    args: &WiUpdateArgs,
    description: &Option<String>,
) -> HashSet<&'static str> {
    let mut reserved = HashSet::new();
    if args.subject.is_some() {
        reserved.insert("subject");
    }
    if args.assignee.is_some() {
        reserved.insert("assignedTo");
    }
    if args.status.is_some() {
        reserved.insert("status");
    }
    if description.is_some() {
        reserved.insert("description");
        if args.description_format.is_some() {
            reserved.insert("formatType");
        }
    }
    if args.priority.is_some() {
        reserved.insert("priority");
    }
    reserved
}

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
    /// Manage work-item relations (parent, sub, associated, depend_on, depended_by).
    Relations(WiRelationsArgs),
}

/// Arguments for `projex workitems search`.
#[derive(Debug, Args)]
pub struct WiSearchArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Work-item category, can be specified multiple times (e.g. -c Req -c Task).
    /// Defaults to Req, Task, and Bug when omitted.
    /// Get available categories via: yunxiao projex workitems types --space-id <SPACE_ID>
    #[arg(short = 'c', long = "category")]
    pub category: Vec<String>,
    /// Optional keyword filter.
    #[arg(short = 'k', long)]
    pub keyword: Option<String>,
    /// Filter by serial number (e.g. PROJ-123).
    #[arg(short = 'n', long)]
    pub serial_number: Option<String>,
    /// Filter by version ID. Get via: yunxiao projex versions list --space-id <SPACE_ID>
    #[arg(short = 'v', long)]
    pub version_id: Option<String>,
    /// Filter by sprint ID. Get via: yunxiao projex sprints list --space-id <SPACE_ID>
    #[arg(short = 'S', long)]
    pub sprint_id: Option<String>,
    /// Page size.
    #[arg(short = 'P', long, default_value = "20")]
    pub page_size: u32,
    /// Page number.
    #[arg(short = 'p', long, default_value = "1")]
    pub page: u32,
}

/// Arguments for `projex workitems get`.
#[derive(Debug, Args)]
pub struct WiGetArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Work item ID. Get via: yunxiao projex workitems search --space-id <SPACE_ID>
    #[arg(long)]
    pub workitem_id: String,
}

/// Arguments for `projex workitems create`.
#[derive(Debug, Args)]
pub struct WiCreateArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Work-item type ID (required). Get via: yunxiao projex workitems types --space-id <SPACE_ID>
    #[arg(long)]
    pub type_id: String,
    /// Work-item subject / title.
    #[arg(long)]
    pub subject: String,
    /// Assignee account user ID. Use the `userId` from `yunxiao org members search`, not the membership `id`.
    #[arg(long)]
    pub assignee: Option<String>,
    /// Sprint ID. Get via: yunxiao projex sprints list --space-id <SPACE_ID>
    #[arg(long)]
    pub sprint_id: Option<String>,
    /// Priority. Get via: yunxiao projex workitems fields --space-id <SPACE_ID> --type-id <TYPE_ID>
    #[arg(long)]
    pub priority: Option<String>,
    /// Labels (comma-separated IDs or exact names). Each value is resolved and validated in the project space.
    #[arg(long)]
    pub labels: Option<String>,
    /// Work item description (optional, directly input).
    #[arg(long)]
    pub description: Option<String>,
    /// Work item description file path (optional, read from file).
    #[arg(long)]
    pub description_file: Option<String>,
    /// Description format: text (richtext) or markdown (default: markdown).
    #[arg(long, value_enum, default_value = "markdown")]
    pub description_format: DescriptionFormat,
    /// Seconds to wait for the created work item to reach a stable readable state. Set to 0 to return the raw create response.
    #[arg(long, default_value_t = 15)]
    pub wait_timeout: u64,
    /// Dynamic field in format "fieldId=value", can be used multiple times.
    /// Use "yunxiao projex workitems fields --space-id <SPACE_ID> --type-id <TYPE_ID>" to get available field IDs. Labels must use --labels.
    #[arg(long = "field")]
    pub fields: Vec<String>,
}

/// Arguments for `projex workitems update`.
#[derive(Debug, Args)]
pub struct WiUpdateArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Work item ID. Get via: yunxiao projex workitems search --space-id <SPACE_ID>
    #[arg(long)]
    pub workitem_id: String,
    /// Work-item type ID (optional, for field validation). Get via: yunxiao projex workitems get --space-id <SPACE_ID> --workitem-id <ID>
    #[arg(long)]
    pub type_id: Option<String>,
    /// New subject (optional).
    #[arg(long)]
    pub subject: Option<String>,
    /// New assignee account user ID. Use the `userId` from `yunxiao org members search`, not the membership `id`.
    #[arg(long)]
    pub assignee: Option<String>,
    /// New status. Get via: yunxiao projex workitems fields --space-id <SPACE_ID> --type-id <TYPE_ID>
    #[arg(long)]
    pub status: Option<String>,
    /// New priority. Get via: yunxiao projex workitems fields --space-id <SPACE_ID> --type-id <TYPE_ID>
    #[arg(long)]
    pub priority: Option<String>,
    /// New labels (comma-separated IDs or exact names). This replaces the work item's label set.
    #[arg(long)]
    pub labels: Option<String>,
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
    /// Use "yunxiao projex workitems fields --space-id <SPACE_ID> --type-id <TYPE_ID>" to get available field IDs. Labels must use --labels.
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
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long, required_unless_present = "project_id")]
    pub space_id: Option<String>,
    /// Compatibility alias for --space-id.
    #[arg(long, hide = true, required_unless_present = "space_id")]
    pub project_id: Option<String>,
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
    /// Work item ID. Get via: yunxiao projex workitems search --space-id <SPACE_ID>
    #[arg(long)]
    pub workitem_id: String,
}

/// Arguments for comment creation.
#[derive(Debug, Args)]
pub struct WiCommentsCreateArgs {
    /// Project space ID. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: String,
    /// Work item ID. Get via: yunxiao projex workitems search --space-id <SPACE_ID>
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
    /// Work item ID. Get via: yunxiao projex workitems search --space-id <SPACE_ID>
    #[arg(long)]
    pub workitem_id: String,
}

/// Arguments for `projex workitems flow`.
#[derive(Debug, Args)]
pub struct WiFlowArgs {
    /// Work item ID. Get via: yunxiao projex workitems search --space-id <SPACE_ID>
    #[arg(long)]
    pub workitem_id: Option<String>,
    /// Project space ID. Required when using --type-id. Get via: yunxiao projex projects search
    #[arg(long)]
    pub space_id: Option<String>,
    /// Work-item type ID. Required when using --space-id. Get via: yunxiao projex workitems types --space-id <SPACE_ID>
    #[arg(long)]
    pub type_id: Option<String>,
}

/// Arguments for `projex workitems relations`.
#[derive(Debug, Args)]
pub struct WiRelationsArgs {
    #[command(subcommand)]
    pub command: WiRelationsCmds,
}

/// Relation operations.
#[derive(Debug, Subcommand)]
pub enum WiRelationsCmds {
    /// List relation records for a work item.
    List(WiRelationsListArgs),
    /// Create a relation between two work items.
    Create(WiRelationsCreateArgs),
    /// Delete a relation between two work items.
    Delete(WiRelationsDeleteArgs),
}

/// Arguments for `projex workitems relations list`.
#[derive(Debug, Args)]
pub struct WiRelationsListArgs {
    /// Work item ID. Get via: yunxiao projex workitems search --space-id <SPACE_ID>
    #[arg(long)]
    pub workitem_id: String,
    /// Relation type: PARENT, SUB, ASSOCIATED, DEPEND_ON, DEPENDED_BY.
    #[arg(long)]
    pub relation_type: String,
}

/// Arguments for `projex workitems relations create`.
#[derive(Debug, Args)]
pub struct WiRelationsCreateArgs {
    /// Source work item ID. Get via: yunxiao projex workitems search --space-id <SPACE_ID>
    #[arg(long)]
    pub workitem_id: String,
    /// Target work item ID to relate to.
    #[arg(long)]
    pub target_workitem_id: String,
    /// Relation type: PARENT, SUB, ASSOCIATED, DEPEND_ON, DEPENDED_BY.
    #[arg(long)]
    pub relation_type: String,
}

/// Arguments for `projex workitems relations delete`.
#[derive(Debug, Args)]
pub struct WiRelationsDeleteArgs {
    /// Source work item ID. Get via: yunxiao projex workitems search --space-id <SPACE_ID>
    #[arg(long)]
    pub workitem_id: String,
    /// Target work item ID to remove relation from.
    #[arg(long)]
    pub target_workitem_id: String,
    /// Relation type: PARENT, SUB, ASSOCIATED, DEPEND_ON, DEPENDED_BY.
    #[arg(long)]
    pub relation_type: String,
}

fn build_create_body(
    args: &WiCreateArgs,
    description: &Option<String>,
    dynamic_fields: &[(String, String)],
    labels: &[ResolvedLabel],
) -> serde_json::Value {
    let mut body = json!({
        "subject": args.subject,
        "spaceId": args.space_id,
        "workitemTypeId": args.type_id,
    });

    if let Some(assignee) = &args.assignee {
        body["assignedTo"] = json!(assignee);
    }
    if let Some(sprint_id) = &args.sprint_id {
        body["sprint"] = json!(sprint_id);
    }
    if !labels.is_empty() {
        body["labels"] = json!(label_ids(labels));
    }
    if let Some(description) = description {
        body["description"] = json!(description);
        body["formatType"] = json!(format_type_to_api(args.description_format));
    }

    let mut custom_field_values = serde_json::Map::new();
    if let Some(priority) = &args.priority {
        custom_field_values.insert("priority".into(), json!(priority));
    }

    for (key, value) in dynamic_fields {
        if STANDARD_FIELDS.contains(&key.as_str()) {
            if ["participants", "trackers", "versions"].contains(&key.as_str()) {
                body[key] = parse_array_field(value);
            } else {
                body[key] = json!(value);
            }
        } else {
            custom_field_values.insert(key.clone(), json!(value));
        }
    }

    if !custom_field_values.is_empty() {
        body["customFieldValues"] = json!(custom_field_values);
    }
    body
}

fn build_update_body(
    args: &WiUpdateArgs,
    description: &Option<String>,
    dynamic_fields: &[(String, String)],
    labels: &[ResolvedLabel],
) -> serde_json::Value {
    let mut body = json!({});
    if let Some(subject) = &args.subject {
        body["subject"] = json!(subject);
    }
    if let Some(assignee) = &args.assignee {
        body["assignedTo"] = json!(assignee);
    }
    if let Some(status) = &args.status {
        body["status"] = json!(status);
    }
    if args.labels.is_some() {
        body["labels"] = json!(label_ids(labels));
    }
    if let Some(description) = description {
        body["description"] = json!(description);
        if let Some(format) = args.description_format {
            body["formatType"] = json!(format_type_to_api(format));
        }
    }
    if let Some(priority) = &args.priority {
        body["priority"] = json!(priority);
    }

    for (key, value) in dynamic_fields {
        if ["participants", "trackers", "versions"].contains(&key.as_str()) {
            body[key] = parse_array_field(value);
        } else {
            body[key] = json!(value);
        }
    }
    body
}

fn create_response_id(data: &serde_json::Value) -> Result<&str> {
    data.get("id")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| {
            CliError::Api("Create work-item response did not include a usable 'id'.".into())
        })
}

fn detail_label_ids(detail: &serde_json::Value) -> HashSet<String> {
    detail
        .get("labels")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|label| {
            label.as_str().map(ToOwned::to_owned).or_else(|| {
                label
                    .get("id")
                    .and_then(serde_json::Value::as_str)
                    .map(ToOwned::to_owned)
            })
        })
        .collect()
}

fn workitem_detail_mismatches(
    detail: &serde_json::Value,
    workitem_id: &str,
    subject: &str,
    assignee: Option<&str>,
    labels: &[ResolvedLabel],
) -> Vec<String> {
    let mut mismatches = Vec::new();
    if detail.get("id").and_then(serde_json::Value::as_str) != Some(workitem_id) {
        mismatches.push("work item ID does not match the create response".into());
    }
    if detail.get("subject").and_then(serde_json::Value::as_str) != Some(subject) {
        mismatches.push("subject has not reached its requested value".into());
    }
    if let Some(assignee) = assignee {
        let actual = detail
            .get("assignedTo")
            .and_then(|value| value.get("id"))
            .and_then(serde_json::Value::as_str);
        if actual != Some(assignee) {
            mismatches.push("assignee has not reached its requested value".into());
        }
    }

    let actual_labels = detail_label_ids(detail);
    let missing_labels: Vec<String> = labels
        .iter()
        .filter(|label| !actual_labels.contains(&label.id))
        .map(|label| format!("{} ({})", label.name, label.id))
        .collect();
    if !missing_labels.is_empty() {
        mismatches.push(format!("missing labels: {}", missing_labels.join(", ")));
    }
    mismatches
}

fn is_not_found(error: &CliError) -> bool {
    matches!(error, CliError::ApiResponse(api_error) if api_error.status == 404)
}

fn is_label_validation_error(error: &CliError) -> bool {
    matches!(
        error,
        CliError::ApiResponse(api_error)
            if api_error
                .code
                .as_deref()
                .is_some_and(|code| LABEL_RETRY_ERROR_CODES.contains(&code))
    )
}

fn label_update_error(workitem_id: &str, labels: &[ResolvedLabel], error: CliError) -> CliError {
    CliError::Api(format!(
        "Failed to set labels [{}] on work item '{workitem_id}': {error}",
        label_details(labels)
    ))
}

async fn replace_workitem_labels(
    client: &ApiClient,
    org_id: &str,
    workitem_id: &str,
    labels: &[ResolvedLabel],
) -> Result<()> {
    client
        .put(
            &format!("/oapi/v1/projex/organizations/{org_id}/workitems/{workitem_id}"),
            &json!({"labels": label_ids(labels)}),
        )
        .await
        .map(|_| ())
}

async fn repair_created_labels(
    client: &ApiClient,
    org_id: &str,
    space_id: &str,
    workitem_id: &str,
    original_references: &[String],
    labels: Vec<ResolvedLabel>,
) -> Result<Vec<ResolvedLabel>> {
    match replace_workitem_labels(client, org_id, workitem_id, &labels).await {
        Ok(()) => Ok(labels),
        Err(error) if is_label_validation_error(&error) => {
            invalidate_label_cache(org_id, space_id)?;
            let refreshed_labels =
                resolve_label_references(client, org_id, space_id, original_references).await?;
            replace_workitem_labels(client, org_id, workitem_id, &refreshed_labels)
                .await
                .map_err(|error| label_update_error(workitem_id, &refreshed_labels, error))?;
            Ok(refreshed_labels)
        }
        Err(error) => Err(label_update_error(workitem_id, &labels, error)),
    }
}

async fn wait_for_stable_workitem(
    client: &ApiClient,
    org_id: &str,
    args: &WiCreateArgs,
    workitem_id: &str,
    original_label_references: &[String],
    mut labels: Vec<ResolvedLabel>,
) -> Result<serde_json::Value> {
    let deadline = Instant::now() + Duration::from_secs(args.wait_timeout);
    let mut delay_index = 0;
    let mut labels_repaired = false;
    let mut last_observation: String;

    loop {
        match client
            .get(
                &format!("/oapi/v1/projex/organizations/{org_id}/workitems/{workitem_id}"),
                &[],
            )
            .await
        {
            Ok(detail) => {
                let mismatches = workitem_detail_mismatches(
                    &detail,
                    workitem_id,
                    &args.subject,
                    args.assignee.as_deref(),
                    &labels,
                );
                if mismatches.is_empty() {
                    return Ok(detail);
                }

                let labels_missing = !labels.is_empty()
                    && !labels
                        .iter()
                        .all(|label| detail_label_ids(&detail).contains(&label.id));
                last_observation = mismatches.join("; ");
                if labels_missing && !labels_repaired {
                    labels = repair_created_labels(
                        client,
                        org_id,
                        &args.space_id,
                        workitem_id,
                        original_label_references,
                        labels,
                    )
                    .await?;
                    labels_repaired = true;
                    continue;
                }
            }
            Err(error) if is_not_found(&error) => {
                last_observation = "work item returned HTTP 404".into();
            }
            Err(error) => return Err(error),
        }

        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(CliError::Api(format!(
                "Work item '{workitem_id}' was created but did not become stable within {} seconds: {}",
                args.wait_timeout,
                last_observation
            )));
        }
        let delay = POLL_DELAYS[delay_index.min(POLL_DELAYS.len() - 1)].min(remaining);
        delay_index += 1;
        sleep(delay).await;
    }
}

async fn create_workitem(
    args: &WiCreateArgs,
    client: &ApiClient,
    org_id: &str,
) -> Result<serde_json::Value> {
    let description =
        resolve_description(args.description.as_ref(), args.description_file.as_ref())?;
    let dynamic_fields = parse_dynamic_fields(&args.fields);
    validate_dynamic_field_overrides(&dynamic_fields, &create_reserved_fields(args, &description))?;

    let field_configs = get_field_configs(client, org_id, &args.space_id, &args.type_id).await?;
    validate_required_create_fields(
        &field_configs,
        &create_provided_fields(args, &description, &dynamic_fields),
    )?;

    let label_references = args
        .labels
        .as_deref()
        .map(parse_label_references)
        .transpose()?;
    let labels = match &label_references {
        Some(references) => {
            resolve_label_references(client, org_id, &args.space_id, references).await?
        }
        None => Vec::new(),
    };
    let body = build_create_body(args, &description, &dynamic_fields, &labels);

    log::debug!(
        "Creating workitem with body: {}",
        serde_json::to_string_pretty(&body).unwrap()
    );
    let data = client
        .post(
            &format!("/oapi/v1/projex/organizations/{org_id}/workitems"),
            &body,
        )
        .await
        .map_err(|error| {
            if label_references.is_some() && is_label_validation_error(&error) {
                CliError::Api(format!(
                    "Work-item creation failed while applying labels. YunXiao may have created a work item without returning its ID; do not retry automatically. Original error: {error}"
                ))
            } else {
                error
            }
        })?;

    if args.wait_timeout == 0 {
        return Ok(data);
    }

    let workitem_id = create_response_id(&data)?.to_string();
    wait_for_stable_workitem(
        client,
        org_id,
        args,
        &workitem_id,
        label_references.as_deref().unwrap_or_default(),
        labels,
    )
    .await
}

async fn update_workitem(
    args: &WiUpdateArgs,
    client: &ApiClient,
    org_id: &str,
) -> Result<serde_json::Value> {
    if let Some(type_id) = &args.type_id {
        let _field_configs = get_field_configs(client, org_id, &args.space_id, type_id).await?;
    }

    let description =
        resolve_description(args.description.as_ref(), args.description_file.as_ref())?;
    let dynamic_fields = parse_dynamic_fields(&args.fields);
    validate_dynamic_field_overrides(&dynamic_fields, &update_reserved_fields(args, &description))?;

    let label_references = args
        .labels
        .as_deref()
        .map(parse_label_references)
        .transpose()?;
    let mut labels = match &label_references {
        Some(references) => {
            resolve_label_references(client, org_id, &args.space_id, references).await?
        }
        None => Vec::new(),
    };

    let path = format!(
        "/oapi/v1/projex/organizations/{org_id}/workitems/{}",
        args.workitem_id
    );
    let mut body = build_update_body(args, &description, &dynamic_fields, &labels);
    match client.put(&path, &body).await {
        Ok(data) => Ok(data),
        Err(error) if label_references.is_some() && is_label_validation_error(&error) => {
            invalidate_label_cache(org_id, &args.space_id)?;
            labels = resolve_label_references(
                client,
                org_id,
                &args.space_id,
                label_references.as_deref().unwrap_or_default(),
            )
            .await?;
            body = build_update_body(args, &description, &dynamic_fields, &labels);
            client
                .put(&path, &body)
                .await
                .map_err(|error| label_update_error(&args.workitem_id, &labels, error))
        }
        Err(error) => Err(error),
    }
}

fn resolve_fields_space_id(args: &WiFieldsArgs) -> Result<&str> {
    match (args.space_id.as_deref(), args.project_id.as_deref()) {
        (Some(space_id), Some(project_id)) if space_id != project_id => Err(CliError::Config(
            format!(
                "--space-id ('{space_id}') and --project-id ('{project_id}') must match when both are supplied."
            ),
        )),
        (Some(space_id), _) => Ok(space_id),
        (_, Some(project_id)) => Ok(project_id),
        (None, None) => Err(CliError::Config(
            "Either --space-id or --project-id must be provided.".into(),
        )),
    }
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
            let data = create_workitem(c, client, oid).await?;
            output::print_output(&data, format)?;
        }
        WorkitemsCmds::Update(u) => {
            let data = update_workitem(u, client, oid).await?;
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
            let space_id = resolve_fields_space_id(f)?;
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/projex/organizations/{oid}/projects/{}/workitemTypes/{}/fields",
                        space_id, f.type_id
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
        WorkitemsCmds::Relations(r) => match &r.command {
            WiRelationsCmds::List(l) => {
                let data = client
                    .get(
                        &format!(
                            "/oapi/v1/projex/organizations/{oid}/workitems/{}/relationRecords",
                            l.workitem_id
                        ),
                        &[("relationType", l.relation_type.as_str())],
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
            WiRelationsCmds::Create(c) => {
                let body = json!({
                    "relationType": c.relation_type,
                    "workitemId": c.target_workitem_id,
                });
                let data = client
                    .post(
                        &format!(
                            "/oapi/v1/projex/organizations/{oid}/workitems/{}/relationRecords",
                            c.workitem_id
                        ),
                        &body,
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
            WiRelationsCmds::Delete(d) => {
                let body = json!({
                    "relationType": d.relation_type,
                    "workitemId": d.target_workitem_id,
                });
                let data = client
                    .delete_with_body(
                        &format!(
                            "/oapi/v1/projex/organizations/{oid}/workitems/{}/relationRecords",
                            d.workitem_id
                        ),
                        &body,
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
        },
    }
    Ok(())
}

fn resolve_search_categories(categories: &[String]) -> String {
    if categories.is_empty() {
        "Req,Task,Bug".to_string()
    } else {
        categories.join(",")
    }
}

fn build_workitems_search_body(s: &WiSearchArgs) -> serde_json::Value {
    let categories = resolve_search_categories(&s.category);
    let conditions_str = ConditionBuilder::new()
        .opt_string_contains("subject", s.keyword.as_deref())
        .opt_string_contains("serialNumber", s.serial_number.as_deref())
        .opt_multi_list_contains("version", "version", s.version_id.as_deref())
        .opt_list_contains("sprint", "sprint", s.sprint_id.as_deref())
        .build();

    let mut body = json!({
        "category": categories,
        "spaceId": s.space_id,
        "page": s.page,
        "perPage": s.page_size,
    });

    if let Some(conds) = conditions_str {
        body["conditions"] = json!(conds);
    }

    body
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
    let body = build_workitems_search_body(s);

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

#[cfg(test)]
#[path = "workitems_tests.rs"]
mod tests;
