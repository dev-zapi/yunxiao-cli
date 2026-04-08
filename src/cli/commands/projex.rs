//! `projex` subcommand – project collaboration management.
//!
//! Covers projects, programs, work items (including comments & attachments),
//! sprints, versions, and effort records.

use clap::{Args, Subcommand};
use reqwest::header::HeaderMap;
use serde_json::json;

use crate::client::ApiClient;
use crate::config;
use crate::config::types::OutputFormat;
use crate::error::Result;
use crate::output;

/// Arguments for the `projex` subcommand.
#[derive(Debug, Args)]
pub struct ProjexArgs {
    /// Projex sub-operation.
    #[command(subcommand)]
    pub command: ProjexCommands,
}

/// Top-level projex operations.
#[derive(Debug, Subcommand)]
pub enum ProjexCommands {
    /// Manage projects.
    Projects(ProjectsArgs),
    /// Manage programs.
    Programs(ProgramsArgs),
    /// Manage work items (requirements, tasks, bugs, etc.).
    Workitems(WorkitemsArgs),
    /// Manage sprints / iterations.
    Sprints(SprintsArgs),
    /// Manage versions / releases.
    Versions(VersionsArgs),
    /// Manage effort / work-hour records.
    Efforts(EffortsArgs),
    /// Manage labels in a project.
    Labels(LabelsArgs),
}

// ───────────────────────── Projects ─────────────────────────────────────

/// Arguments for `projex projects`.
#[derive(Debug, Args)]
pub struct ProjectsArgs {
    #[command(subcommand)]
    pub command: ProjectsCmds,
}

/// Project operations.
#[derive(Debug, Subcommand)]
pub enum ProjectsCmds {
    /// Search projects by keyword.
    Search(ProjectSearchArgs),
    /// List all projects (alternative to search).
    List(ProjectListArgs),
    /// Get project details by ID.
    Get(ProjectGetArgs),
}

/// Arguments for `projex projects list`.
#[derive(Debug, Args)]
pub struct ProjectListArgs {
    /// Page size (1-200).
    #[arg(long, default_value = "20")]
    pub per_page: u32,
    /// Page number (1-based).
    #[arg(long, default_value = "1")]
    pub page: u32,
    /// Sort field: gmtCreate (default) or name.
    #[arg(long, value_enum, default_value = "gmtCreate")]
    pub order_by: OrderByField,
    /// Sort order: desc (default) or asc.
    #[arg(long, value_enum, default_value = "desc")]
    pub sort: SortOrder,
    /// Filter by logical status (NORMAL, ARCHIVED, DELETED).
    #[arg(long)]
    pub status: Option<String>,
    /// Filter by creator user ID.
    #[arg(long)]
    pub creator: Option<String>,
    /// Filter by admin user ID.
    #[arg(long)]
    pub admin: Option<String>,
    /// Extra conditions: managed, joined, or starred.
    #[arg(long)]
    pub scope: Option<String>,
}

/// Arguments for `projex projects search`.
#[derive(Debug, Args)]
pub struct ProjectSearchArgs {
    /// Search keyword (searches in project name).
    #[arg(long)]
    pub keyword: Option<String>,
    /// Page size (1-200).
    #[arg(long, default_value = "20")]
    pub per_page: u32,
    /// Page number (1-based).
    #[arg(long, default_value = "1")]
    pub page: u32,
    /// Sort field: gmtCreate (default) or name.
    #[arg(long, value_enum, default_value = "gmtCreate")]
    pub order_by: OrderByField,
    /// Sort order: desc (default) or asc.
    #[arg(long, value_enum, default_value = "desc")]
    pub sort: SortOrder,
    /// Filter by logical status (NORMAL, ARCHIVED, DELETED).
    #[arg(long)]
    pub status: Option<String>,
    /// Filter by creator user ID.
    #[arg(long)]
    pub creator: Option<String>,
    /// Filter by admin user ID.
    #[arg(long)]
    pub admin: Option<String>,
}

/// Sort field for project listing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum OrderByField {
    /// Sort by creation time.
    #[value(name = "gmtCreate")]
    GmtCreate,
    /// Sort by project name.
    #[value(name = "name")]
    Name,
}

/// Sort order for project listing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum SortOrder {
    /// Descending order.
    #[value(name = "desc")]
    Desc,
    /// Ascending order.
    #[value(name = "asc")]
    Asc,
}

/// Description format for workitem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum DescriptionFormat {
    /// Rich text format (富文本).
    #[value(name = "text")]
    Text,
    /// Markdown format (Markdown格式).
    #[value(name = "markdown")]
    Markdown,
}

/// Arguments for `projex projects get`.
#[derive(Debug, Args)]
pub struct ProjectGetArgs {
    /// Project ID.
    pub id: String,
}

// ───────────────────────── Programs ─────────────────────────────────────

/// Arguments for `projex programs`.
#[derive(Debug, Args)]
pub struct ProgramsArgs {
    #[command(subcommand)]
    pub command: ProgramsCmds,
}

/// Program operations.
#[derive(Debug, Subcommand)]
pub enum ProgramsCmds {
    /// Search programs by keyword.
    Search(ProgramSearchArgs),
}

/// Arguments for `projex programs search`.
#[derive(Debug, Args)]
pub struct ProgramSearchArgs {
    /// Search keyword.
    #[arg(long)]
    pub keyword: Option<String>,
}

// ───────────────────────── Work Items ───────────────────────────────────

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

// ───── Work-item comments ───────────────────────────────────────────────

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

// ───── Work-item attachments ────────────────────────────────────────────

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

// ───────────────────────── Sprints ──────────────────────────────────────

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

// ───────────────────────── Versions ─────────────────────────────────────

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

// ───────────────────────── Efforts ──────────────────────────────────────

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

// ───────────────────────── Labels ────────────────────────────────────────

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

// ─────────────────────────── Execute ────────────────────────────────────

/// Execute the `projex` subcommand tree.
pub async fn execute(
    args: &ProjexArgs,
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
        ProjexCommands::Projects(p) => exec_projects(p, &client, &org_id, format).await,
        ProjexCommands::Programs(p) => exec_programs(p, &client, &org_id, format).await,
        ProjexCommands::Workitems(w) => exec_workitems(w, &client, &org_id, format).await,
        ProjexCommands::Sprints(s) => exec_sprints(s, &client, &org_id, format).await,
        ProjexCommands::Versions(v) => exec_versions(v, &client, &org_id, format).await,
        ProjexCommands::Efforts(e) => exec_efforts(e, &client, &org_id, format).await,
        ProjexCommands::Labels(l) => exec_labels(l, &client, &org_id, format).await,
    }
}

/// Helper: require org ID for org-scoped endpoints.
fn require_org(org_id: &Option<String>) -> Result<&str> {
    org_id.as_deref().ok_or_else(|| {
        crate::error::CliError::Config(
            "Organization ID required. Set via --org-id, YUNXIAO_CLI_ORG_ID, or config.".into(),
        )
    })
}

/// Parse dynamic fields from "key=value" format.
fn parse_dynamic_fields(fields: &[String]) -> Vec<(String, String)> {
    fields
        .iter()
        .filter_map(|f| {
            let parts: Vec<&str> = f.splitn(2, '=').collect();
            if parts.len() == 2 {
                Some((parts[0].to_string(), parts[1].to_string()))
            } else {
                log::warn!("Invalid field format: {}, expected \"key=value\"", f);
                None
            }
        })
        .collect()
}

/// Resolve description content from args.
fn resolve_description(
    description: Option<&String>,
    description_file: Option<&String>,
) -> Result<Option<String>> {
    if let Some(content) = description {
        return Ok(Some(content.to_string()));
    }
    if let Some(path) = description_file {
        let content = std::fs::read_to_string(path)?;
        return Ok(Some(content));
    }
    Ok(None)
}

/// Convert DescriptionFormat to API formatType string.
fn format_type_to_api(format: DescriptionFormat) -> &'static str {
    match format {
        DescriptionFormat::Text => "RICHTEXT",
        DescriptionFormat::Markdown => "MARKDOWN",
    }
}

/// Print pagination information from response headers.
fn print_pagination_info(headers: &HeaderMap) {
    if let Some(total) = get_header_int(headers, "x-total") {
        eprintln!("Total: {}", total);
    }
    if let Some(page) = get_header_int(headers, "x-page") {
        eprintln!("Page: {}", page);
    }
    if let Some(per_page) = get_header_int(headers, "x-per-page") {
        eprintln!("Per Page: {}", per_page);
    }
    if let Some(total_pages) = get_header_int(headers, "x-total-pages") {
        eprintln!("Total Pages: {}", total_pages);
    }
}

/// Get integer value from response header.
fn get_header_int(headers: &HeaderMap, key: &str) -> Option<u32> {
    headers
        .get(key)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u32>().ok())
}

// ─────────────────────────── Projects ───────────────────────────────────

/// Execute project sub-operations.
async fn exec_projects(
    args: &ProjectsArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        ProjectsCmds::Search(s) => {
            // API docs: https://help.aliyun.com/zh/yunxiao/developer-reference/searchprojects?spm=a2c4g.11186623.help-menu-150040.d_5_0_7_0_4.23dc1b82xEGVGs
            // Build conditions according to API spec
            let mut conditions = Vec::new();

            // Add keyword filter if provided
            if let Some(ref kw) = s.keyword {
                conditions.push(json!({
                    "className": "string",
                    "fieldIdentifier": "name",
                    "format": "input",
                    "operator": "BETWEEN",
                    "toValue": null,
                    "value": [kw]
                }));
            }

            // Add status filter if provided
            if let Some(ref status) = s.status {
                conditions.push(json!({
                    "className": "string",
                    "fieldIdentifier": "logicalStatus",
                    "format": "list",
                    "operator": "CONTAINS",
                    "toValue": null,
                    "value": [status]
                }));
            }

            // Add creator filter if provided
            if let Some(ref creator) = s.creator {
                conditions.push(json!({
                    "className": "user",
                    "fieldIdentifier": "creator",
                    "format": "list",
                    "operator": "CONTAINS",
                    "toValue": null,
                    "value": [creator]
                }));
            }

            // Add admin filter if provided
            if let Some(ref admin) = s.admin {
                conditions.push(json!({
                    "className": "user",
                    "fieldIdentifier": "project.admin",
                    "format": "multiList",
                    "operator": "CONTAINS",
                    "toValue": null,
                    "value": [admin]
                }));
            }

            let conditions_str = if conditions.is_empty() {
                None
            } else {
                Some(json!({ "conditionGroups": [conditions] }).to_string())
            };

            let body = json!({
                "page": s.page,
                "perPage": s.per_page,
                "orderBy": match s.order_by {
                    OrderByField::GmtCreate => "gmtCreate",
                    OrderByField::Name => "name",
                },
                "sort": match s.sort {
                    SortOrder::Desc => "desc",
                    SortOrder::Asc => "asc",
                },
                "conditions": conditions_str,
            });

            let resp = client
                .post_with_headers(
                    &format!("/oapi/v1/projex/organizations/{oid}/projects:search"),
                    &body,
                )
                .await?;

            // Print pagination info if available
            print_pagination_info(&resp.headers);

            output::print_output(&resp.body, format)?;
        }
        ProjectsCmds::List(l) => {
            let mut all_projects = Vec::new();
            let mut current_page = l.page;
            let per_page = l.per_page.min(200); // API限制最大200

            // Build conditions according to API spec
            let mut conditions = Vec::new();

            // Add status filter if provided
            if let Some(ref status) = l.status {
                conditions.push(json!({
                    "className": "string",
                    "fieldIdentifier": "logicalStatus",
                    "format": "list",
                    "operator": "CONTAINS",
                    "toValue": null,
                    "value": [status]
                }));
            }

            // Add creator filter if provided
            if let Some(ref creator) = l.creator {
                conditions.push(json!({
                    "className": "user",
                    "fieldIdentifier": "creator",
                    "format": "list",
                    "operator": "CONTAINS",
                    "toValue": null,
                    "value": [creator]
                }));
            }

            // Add admin filter if provided
            if let Some(ref admin) = l.admin {
                conditions.push(json!({
                    "className": "user",
                    "fieldIdentifier": "project.admin",
                    "format": "multiList",
                    "operator": "CONTAINS",
                    "toValue": null,
                    "value": [admin]
                }));
            }

            // Build extraConditions if scope is provided
            let extra_conditions_str = l
                .scope
                .as_ref()
                .map(|scope| json!({ "scope": scope }).to_string());

            loop {
                let conditions_str = if conditions.is_empty() {
                    None
                } else {
                    Some(json!({ "conditionGroups": [conditions.clone()] }).to_string())
                };

                let body = json!({
                    "page": current_page,
                    "perPage": per_page,
                    "orderBy": match l.order_by {
                        OrderByField::GmtCreate => "gmtCreate",
                        OrderByField::Name => "name",
                    },
                    "sort": match l.sort {
                        SortOrder::Desc => "desc",
                        SortOrder::Asc => "asc",
                    },
                    "conditions": conditions_str,
                    "extraConditions": extra_conditions_str,
                });

                let resp = client
                    .post_with_headers(
                        &format!("/oapi/v1/projex/organizations/{oid}/projects:search"),
                        &body,
                    )
                    .await?;

                // Parse response headers for pagination info
                let headers = &resp.headers;
                let total_pages = get_header_int(headers, "x-total-pages");
                let current_resp_page = get_header_int(headers, "x-page").unwrap_or(current_page);

                // 解析响应中的项目列表
                if let Some(projects) = resp.body.as_array() {
                    if projects.is_empty() {
                        break;
                    }
                    all_projects.extend(projects.clone());

                    // Check if we've reached the last page using response headers
                    if let Some(total) = total_pages {
                        if current_resp_page >= total {
                            break;
                        }
                    } else if projects.len() < per_page as usize {
                        // Fallback: check if we got fewer results than requested
                        break;
                    }
                } else {
                    break;
                }

                current_page = current_resp_page + 1;

                // 安全检查：防止无限循环
                if current_page > 100 {
                    log::warn!("Reached maximum page limit (100), stopping pagination");
                    break;
                }
            }

            // 输出合并后的结果
            let combined = serde_json::Value::Array(all_projects);
            output::print_output(&combined, format)?;
        }
        ProjectsCmds::Get(g) => {
            let data = client
                .get(
                    &format!("/oapi/v1/projex/organizations/{oid}/projects/{}", g.id),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

// ─────────────────────────── Programs ───────────────────────────────────

/// Execute program sub-operations.
async fn exec_programs(
    args: &ProgramsArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        ProgramsCmds::Search(s) => {
            let mut body = json!({});
            if let Some(ref kw) = s.keyword {
                body["keyword"] = json!(kw);
            }
            let data = client
                .post(
                    &format!("/oapi/v1/projex/organizations/{oid}/programs:search"),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

// ─────────────────────────── Work Items ─────────────────────────────────

/// Execute work-item sub-operations.
async fn exec_workitems(
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
                if let Some(format) = u.description_format {
                    body["formatType"] = json!(format_type_to_api(format));
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
            // When category is provided, use the project-scoped endpoint.
            // API docs: https://help.aliyun.com/zh/yunxiao/developer-reference/listworkitemtypes?spm=a2c4g.11186623.help-menu-150040.d_5_0_7_13_1.3d65224fUbi08m&scm=20140722.H_2870624._.OR_help-T_cn~zh-V_1
            // When category is not provided, use the organization-wide endpoint.
            // API docs: https://help.aliyun.com/zh/yunxiao/developer-reference/listallworkitemtypes?spm=a2c4g.11186623.help-menu-150040.d_5_0_7_13_0.5c4c6a3beGaLvw&scm=20140722.H_2870623._.OR_help-T_cn~zh-V_1
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

            // Local keyword filter by name
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
            // API docs: https://help.aliyun.com/zh/yunxiao/developer-reference/getworkitemtypefieldconfig
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
    }
    Ok(())
}

/// Execute work-item search.
///
/// API docs: <https://help.aliyun.com/zh/yunxiao/developer-reference/searchworkitems?spm=a2c4g.11186623.help-menu-150040.d_5_0_7_9_0.59282316XIqioe&scm=20140722.H_2870366._.OR_help-T_cn~zh-V_1>
async fn exec_workitems_search(
    s: &WiSearchArgs,
    oid: &str,
    client: &ApiClient,
    format: &OutputFormat,
) -> Result<()> {
    // Build conditions according to API spec
    // The API uses conditions parameter with conditionGroups for filtering
    let mut conditions = Vec::new();

    // Add keyword filter as subject CONTAINS condition if provided
    if let Some(ref kw) = s.keyword {
        conditions.push(json!({
            "fieldIdentifier": "subject",
            "operator": "CONTAINS",
            "value": [kw],
            "toValue": null,
            "className": "string",
            "format": "input"
        }));
    }

    let conditions_str = if conditions.is_empty() {
        None
    } else {
        Some(json!({ "conditionGroups": [conditions] }).to_string())
    };

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

    // Print pagination info if available
    print_pagination_info(&resp.headers);

    output::print_output(&resp.body, format)?;
    Ok(())
}

// ─────────────────────────── Sprints ────────────────────────────────────

/// Execute sprint sub-operations.
async fn exec_sprints(
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

// ─────────────────────────── Versions ───────────────────────────────────

/// Execute version sub-operations.
async fn exec_versions(
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

// ─────────────────────────── Efforts ────────────────────────────────────

/// Execute effort sub-operations.
async fn exec_efforts(
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

// ─────────────────────────── Labels ─────────────────────────────────────

/// Execute label sub-operations.
async fn exec_labels(
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
