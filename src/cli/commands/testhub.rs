//! `testhub` subcommand – test management.
//!
//! Covers test cases (including directories and custom fields), test plans,
//! and test results via the Yunxiao TestHub API.

use clap::{Args, Subcommand};
use serde_json::json;

use crate::client::ApiClient;
use crate::config;
use crate::config::types::OutputFormat;
use crate::error::Result;
use crate::output;

/// Arguments for the `testhub` subcommand.
#[derive(Debug, Args)]
pub struct TesthubArgs {
    #[command(subcommand)]
    pub command: TesthubCommands,
}

/// Top-level testhub operations.
#[derive(Debug, Subcommand)]
pub enum TesthubCommands {
    /// Manage test cases.
    Cases(CasesArgs),
    /// Manage test plans and results.
    Plans(PlansArgs),
}

// ─────────────────────────── Cases ──────────────────────────────────────

/// Arguments for `testhub cases`.
#[derive(Debug, Args)]
pub struct CasesArgs {
    #[command(subcommand)]
    pub command: CasesCmds,
}

/// Test-case operations.
#[derive(Debug, Subcommand)]
pub enum CasesCmds {
    /// Search test cases.
    Search(CaseSearchArgs),
    /// Get test case details.
    Get(CaseGetArgs),
    /// Create a new test case.
    Create(CaseCreateArgs),
    /// Delete a test case.
    Delete(CaseDeleteArgs),
    /// Manage test case directories.
    Directories(CaseDirsArgs),
    /// List custom fields for test cases in a space.
    Fields(CaseFieldsArgs),
}

/// Arguments for `testhub cases search`.
#[derive(Debug, Args)]
pub struct CaseSearchArgs {
    /// Project space ID.
    #[arg(long)]
    pub space_id: String,
    /// Search keyword (optional).
    #[arg(long)]
    pub keyword: Option<String>,
    /// Page number.
    #[arg(long, default_value = "1")]
    pub page: u32,
    /// Results per page.
    #[arg(long, default_value = "20")]
    pub per_page: u32,
}

/// Arguments for `testhub cases get`.
#[derive(Debug, Args)]
pub struct CaseGetArgs {
    /// Project space ID.
    #[arg(long)]
    pub space_id: String,
    /// Test case ID.
    #[arg(long)]
    pub case_id: String,
}

/// Arguments for `testhub cases create`.
#[derive(Debug, Args)]
pub struct CaseCreateArgs {
    /// Project space ID.
    #[arg(long)]
    pub space_id: String,
    /// Test case subject / title.
    #[arg(long)]
    pub subject: String,
    /// Priority (e.g. P0, P1, P2, P3, optional).
    #[arg(long)]
    pub priority: Option<String>,
    /// Directory ID to place the case in (optional).
    #[arg(long)]
    pub directory_id: Option<String>,
    /// Pre-conditions (optional).
    #[arg(long)]
    pub precondition: Option<String>,
}

/// Arguments for `testhub cases delete`.
#[derive(Debug, Args)]
pub struct CaseDeleteArgs {
    /// Project space ID.
    #[arg(long)]
    pub space_id: String,
    /// Test case ID.
    #[arg(long)]
    pub case_id: String,
}

// ── Case directories ────────────────────────────────────────────────────

/// Arguments for `testhub cases directories`.
#[derive(Debug, Args)]
pub struct CaseDirsArgs {
    #[command(subcommand)]
    pub command: CaseDirsCmds,
}

/// Test-case directory operations.
#[derive(Debug, Subcommand)]
pub enum CaseDirsCmds {
    /// List test case directories.
    List(CaseDirsListArgs),
    /// Create a new test case directory.
    Create(CaseDirsCreateArgs),
}

/// Arguments for `testhub cases directories list`.
#[derive(Debug, Args)]
pub struct CaseDirsListArgs {
    /// Project space ID.
    #[arg(long)]
    pub space_id: String,
}

/// Arguments for `testhub cases directories create`.
#[derive(Debug, Args)]
pub struct CaseDirsCreateArgs {
    /// Project space ID.
    #[arg(long)]
    pub space_id: String,
    /// Directory name.
    #[arg(long)]
    pub name: String,
    /// Parent directory ID (optional, omit for root).
    #[arg(long)]
    pub parent_id: Option<String>,
}

/// Arguments for `testhub cases fields`.
#[derive(Debug, Args)]
pub struct CaseFieldsArgs {
    /// Project space ID.
    #[arg(long)]
    pub space_id: String,
}

// ─────────────────────────── Plans ──────────────────────────────────────

/// Arguments for `testhub plans`.
#[derive(Debug, Args)]
pub struct PlansArgs {
    #[command(subcommand)]
    pub command: PlansCmds,
}

/// Test-plan operations.
#[derive(Debug, Subcommand)]
pub enum PlansCmds {
    /// List test plans.
    List(PlansListArgs),
    /// Manage test plan results.
    Results(PlansResultsArgs),
}

/// Arguments for `testhub plans list`.
#[derive(Debug, Args)]
pub struct PlansListArgs {
    /// Project space ID.
    #[arg(long)]
    pub space_id: String,
}

/// Arguments for `testhub plans results`.
#[derive(Debug, Args)]
pub struct PlansResultsArgs {
    #[command(subcommand)]
    pub command: PlansResultsCmds,
}

/// Test-plan result operations.
#[derive(Debug, Subcommand)]
pub enum PlansResultsCmds {
    /// List results for a test plan.
    List(PlansResultsListArgs),
    /// Update a test result for a specific case within a plan.
    Update(PlansResultsUpdateArgs),
}

/// Arguments for `testhub plans results list`.
#[derive(Debug, Args)]
pub struct PlansResultsListArgs {
    /// Project space ID.
    #[arg(long)]
    pub space_id: String,
    /// Test plan ID.
    #[arg(long)]
    pub plan_id: String,
}

/// Arguments for `testhub plans results update`.
#[derive(Debug, Args)]
pub struct PlansResultsUpdateArgs {
    /// Project space ID.
    #[arg(long)]
    pub space_id: String,
    /// Test plan ID.
    #[arg(long)]
    pub plan_id: String,
    /// Test case ID.
    #[arg(long)]
    pub case_id: String,
    /// Result status (pass, fail, block, skip, etc.).
    #[arg(long)]
    pub status: String,
}

// ─────────────────────────── Execute ────────────────────────────────────

/// Execute the `testhub` subcommand tree.
pub async fn execute(
    args: &TesthubArgs,
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
        TesthubCommands::Cases(c) => exec_cases(c, &client, &org_id, format).await,
        TesthubCommands::Plans(p) => exec_plans(p, &client, &org_id, format).await,
    }
}

/// Helper: require org ID.
fn require_org(org_id: &Option<String>) -> Result<&str> {
    org_id.as_deref().ok_or_else(|| {
        crate::error::CliError::Config(
            "Organization ID required. Set via --org-id, YUNXIAO_CLI_ORG_ID, or config.".into(),
        )
    })
}

// ─────────────────────────── Cases ──────────────────────────────────────

/// Execute test-case sub-operations.
async fn exec_cases(
    args: &CasesArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        CasesCmds::Search(s) => {
            let mut body = json!({
                "page": s.page,
                "perPage": s.per_page,
            });
            if let Some(ref kw) = s.keyword {
                body["keyword"] = json!(kw);
            }
            let data = client
                .post(
                    &format!(
                        "/oapi/v1/testhub/organizations/{oid}/testRepos/{}/testcases:search",
                        s.space_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        CasesCmds::Get(g) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/testhub/organizations/{oid}/testRepos/{}/testcases/{}",
                        g.space_id, g.case_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        CasesCmds::Create(c) => {
            let mut body = json!({"subject": c.subject});
            if let Some(ref p) = c.priority {
                body["priority"] = json!(p);
            }
            if let Some(ref d) = c.directory_id {
                body["directoryId"] = json!(d);
            }
            if let Some(ref pc) = c.precondition {
                body["precondition"] = json!(pc);
            }
            let data = client
                .post(
                    &format!(
                        "/oapi/v1/testhub/organizations/{oid}/testRepos/{}/testcases",
                        c.space_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        CasesCmds::Delete(d) => {
            let data = client
                .delete(
                    &format!(
                        "/oapi/v1/testhub/organizations/{oid}/testRepos/{}/testcases/{}",
                        d.space_id, d.case_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        CasesCmds::Directories(dirs) => match &dirs.command {
            CaseDirsCmds::List(l) => {
                let data = client
                    .get(
                    &format!(
                        "/oapi/v1/testhub/organizations/{oid}/testRepos/{}/directories",
                        l.space_id
                    ),
                        &[],
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
            CaseDirsCmds::Create(c) => {
                let mut body = json!({"name": c.name});
                if let Some(ref pid) = c.parent_id {
                    body["parentId"] = json!(pid);
                }
                let data = client
                    .post(
                    &format!(
                        "/oapi/v1/testhub/organizations/{oid}/testRepos/{}/directories",
                        c.space_id
                    ),
                        &body,
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
        },
        CasesCmds::Fields(f) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/testhub/organizations/{oid}/testRepos/{}/testcases/fields",
                        f.space_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

// ─────────────────────────── Plans ──────────────────────────────────────

/// Execute test-plan sub-operations.
async fn exec_plans(
    args: &PlansArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        PlansCmds::List(_l) => {
            let data = client
                .post(
                    &format!(
                        "/oapi/v1/projex/organizations/{oid}/testPlan/list",
                    ),
                    &json!({}),
                )
                .await?;
            output::print_output(&data, format)?;
        }
        PlansCmds::Results(r) => match &r.command {
            PlansResultsCmds::List(l) => {
                let data = client
                    .get(
                        &format!(
                            "/oapi/v1/testhub/organizations/{oid}/testPlans/{}/results",
                            l.plan_id
                        ),
                        &[],
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
            PlansResultsCmds::Update(u) => {
                let body = json!({
                    "status": u.status,
                });
                let data = client
                    .put(
                        &format!(
                            "/oapi/v1/testhub/organizations/{oid}/testPlans/{}/testcases/{}",
                            u.plan_id, u.case_id
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
