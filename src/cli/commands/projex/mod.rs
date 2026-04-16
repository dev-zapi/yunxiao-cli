//! `projex` subcommand – project collaboration management.
//!
//! Covers projects, programs, work items (including comments & attachments),
//! sprints, versions, and effort records.

pub mod condition;
mod efforts;
mod labels;
mod programs;
mod projects;
mod sprints;
mod versions;
mod workitems;

use clap::{Args, Subcommand};
use reqwest::header::HeaderMap;

use crate::client::ApiClient;
use crate::config;
use crate::config::types::OutputFormat;
use crate::error::Result;

pub use efforts::EffortsArgs;
pub use labels::LabelsArgs;
pub use programs::ProgramsArgs;
pub use projects::ProjectsArgs;
pub use sprints::SprintsArgs;
pub use versions::VersionsArgs;
pub use workitems::WorkitemsArgs;

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
        ProjexCommands::Projects(p) => projects::exec_projects(p, &client, &org_id, format).await,
        ProjexCommands::Programs(p) => programs::exec_programs(p, &client, &org_id, format).await,
        ProjexCommands::Workitems(w) => {
            workitems::exec_workitems(w, &client, &org_id, format).await
        }
        ProjexCommands::Sprints(s) => sprints::exec_sprints(s, &client, &org_id, format).await,
        ProjexCommands::Versions(v) => versions::exec_versions(v, &client, &org_id, format).await,
        ProjexCommands::Efforts(e) => efforts::exec_efforts(e, &client, &org_id, format).await,
        ProjexCommands::Labels(l) => labels::exec_labels(l, &client, &org_id, format).await,
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
        let processed = content.replace("\\n", "\n");
        return Ok(Some(processed));
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
