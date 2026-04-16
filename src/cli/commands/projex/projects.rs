//! Project sub-operations for `projex projects`.

use clap::{Args, Subcommand};
use serde_json::json;

use super::condition::ConditionBuilder;
use super::{get_header_int, print_pagination_info, require_org, OrderByField, SortOrder};
use crate::client::ApiClient;
use crate::config::types::OutputFormat;
use crate::error::Result;
use crate::output;

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

/// Arguments for `projex projects get`.
#[derive(Debug, Args)]
pub struct ProjectGetArgs {
    /// Project ID.
    pub id: String,
}

/// Execute project sub-operations.
pub(super) async fn exec_projects(
    args: &ProjectsArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        ProjectsCmds::Search(s) => {
            let conditions_str = ConditionBuilder::new()
                .opt_string_between("name", s.keyword.as_deref())
                .opt_list_contains("logicalStatus", "string", s.status.as_deref())
                .opt_user_contains("creator", s.creator.as_deref())
                .opt_multi_list_contains("project.admin", "user", s.admin.as_deref())
                .build();

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

            print_pagination_info(&resp.headers);
            output::print_output(&resp.body, format)?;
        }
        ProjectsCmds::List(l) => {
            let mut all_projects = Vec::new();
            let mut current_page = l.page;
            let per_page = l.per_page.min(200);

            let builder = ConditionBuilder::new()
                .opt_list_contains("logicalStatus", "string", l.status.as_deref())
                .opt_user_contains("creator", l.creator.as_deref())
                .opt_multi_list_contains("project.admin", "user", l.admin.as_deref());
            let conditions_str = builder.build();

            let extra_conditions_str = l
                .scope
                .as_ref()
                .map(|scope| json!({ "scope": scope }).to_string());

            loop {
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

                let headers = &resp.headers;
                let total_pages = get_header_int(headers, "x-total-pages");
                let current_resp_page = get_header_int(headers, "x-page").unwrap_or(current_page);

                if let Some(projects) = resp.body.as_array() {
                    if projects.is_empty() {
                        break;
                    }
                    all_projects.extend(projects.clone());

                    if let Some(total) = total_pages {
                        if current_resp_page >= total {
                            break;
                        }
                    } else if projects.len() < per_page as usize {
                        break;
                    }
                } else {
                    break;
                }

                current_page = current_resp_page + 1;

                if current_page > 100 {
                    log::warn!("Reached maximum page limit (100), stopping pagination");
                    break;
                }
            }

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
