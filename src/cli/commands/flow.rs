//! `flow` subcommand – pipeline management.
//!
//! Covers pipelines, pipeline runs, jobs (including logs), and service
//! connections via the Yunxiao Flow API.

use clap::{Args, Subcommand};
use serde_json::json;

use crate::client::ApiClient;
use crate::config;
use crate::config::types::OutputFormat;
use crate::error::Result;
use crate::output;

/// Arguments for the `flow` subcommand.
#[derive(Debug, Args)]
pub struct FlowArgs {
    #[command(subcommand)]
    pub command: FlowCommands,
}

/// Top-level flow operations.
#[derive(Debug, Subcommand)]
pub enum FlowCommands {
    /// Manage pipelines.
    Pipelines(PipelinesArgs),
    /// Manage pipeline runs.
    Runs(RunsArgs),
    /// Manage pipeline jobs.
    Jobs(JobsArgs),
    /// Manage service connections.
    Connections(ConnectionsArgs),
}

// ─────────────────────────── Pipelines ──────────────────────────────────

/// Arguments for `flow pipelines`.
#[derive(Debug, Args)]
pub struct PipelinesArgs {
    #[command(subcommand)]
    pub command: PipelinesCmds,
}

/// Pipeline operations.
#[derive(Debug, Subcommand)]
pub enum PipelinesCmds {
    /// List pipelines.
    List(PipelineListArgs),
    /// Get pipeline details.
    Get(PipelineGetArgs),
    /// Update pipeline YAML definition.
    Update(PipelineUpdateArgs),
}

/// Arguments for `flow pipelines list`.
#[derive(Debug, Args)]
pub struct PipelineListArgs {
    /// Search keyword.
    #[arg(long)]
    pub keyword: Option<String>,
    /// Page number (1-based).
    #[arg(long, default_value = "1")]
    pub page: u32,
    /// Results per page.
    #[arg(long, default_value = "20")]
    pub per_page: u32,
}

/// Arguments for `flow pipelines get`.
#[derive(Debug, Args)]
pub struct PipelineGetArgs {
    /// Pipeline ID.
    #[arg(long)]
    pub pipeline_id: String,
}

/// Arguments for `flow pipelines update`.
#[derive(Debug, Args)]
pub struct PipelineUpdateArgs {
    /// Pipeline ID.
    #[arg(long)]
    pub pipeline_id: String,
    /// YAML content for the pipeline definition.
    #[arg(long)]
    pub yaml: String,
}

// ─────────────────────────── Runs ───────────────────────────────────────

/// Arguments for `flow runs`.
#[derive(Debug, Args)]
pub struct RunsArgs {
    #[command(subcommand)]
    pub command: RunsCmds,
}

/// Pipeline run operations.
#[derive(Debug, Subcommand)]
pub enum RunsCmds {
    /// List runs for a pipeline.
    List(RunListArgs),
    /// Get details of a specific run.
    Get(RunGetArgs),
    /// Trigger a new pipeline run.
    Create(RunCreateArgs),
    /// Get the latest run for a pipeline.
    Latest(RunLatestArgs),
}

/// Arguments for `flow runs list`.
#[derive(Debug, Args)]
pub struct RunListArgs {
    /// Pipeline ID.
    #[arg(long)]
    pub pipeline_id: String,
    /// Page number.
    #[arg(long, default_value = "1")]
    pub page: u32,
    /// Results per page.
    #[arg(long, default_value = "20")]
    pub per_page: u32,
}

/// Arguments for `flow runs get`.
#[derive(Debug, Args)]
pub struct RunGetArgs {
    /// Pipeline ID.
    #[arg(long)]
    pub pipeline_id: String,
    /// Run ID.
    #[arg(long)]
    pub run_id: String,
}

/// Arguments for `flow runs create`.
#[derive(Debug, Args)]
pub struct RunCreateArgs {
    /// Pipeline ID.
    #[arg(long)]
    pub pipeline_id: String,
    /// Run parameters as a JSON string (optional).
    #[arg(long)]
    pub params: Option<String>,
}

/// Arguments for `flow runs latest`.
#[derive(Debug, Args)]
pub struct RunLatestArgs {
    /// Pipeline ID.
    #[arg(long)]
    pub pipeline_id: String,
}

// ─────────────────────────── Jobs ───────────────────────────────────────

/// Arguments for `flow jobs`.
#[derive(Debug, Args)]
pub struct JobsArgs {
    #[command(subcommand)]
    pub command: JobsCmds,
}

/// Job operations.
#[derive(Debug, Subcommand)]
pub enum JobsCmds {
    /// List jobs in a pipeline by category.
    List(JobListArgs),
    /// Get job run history.
    History(JobHistoryArgs),
    /// Trigger a specific job within a run.
    Run(JobRunArgs),
    /// Get job execution logs.
    Log(JobLogArgs),
}

/// Arguments for `flow jobs list`.
#[derive(Debug, Args)]
pub struct JobListArgs {
    /// Pipeline ID.
    #[arg(long)]
    pub pipeline_id: String,
    /// Job category (e.g. BUILD, DEPLOY, TEST).
    #[arg(long)]
    pub category: String,
}

/// Arguments for `flow jobs history`.
#[derive(Debug, Args)]
pub struct JobHistoryArgs {
    /// Pipeline ID.
    #[arg(long)]
    pub pipeline_id: String,
    /// Job ID.
    #[arg(long)]
    pub job_id: String,
}

/// Arguments for `flow jobs run`.
#[derive(Debug, Args)]
pub struct JobRunArgs {
    /// Pipeline ID.
    #[arg(long)]
    pub pipeline_id: String,
    /// Run ID.
    #[arg(long)]
    pub run_id: String,
    /// Job ID.
    #[arg(long)]
    pub job_id: String,
}

/// Arguments for `flow jobs log`.
#[derive(Debug, Args)]
pub struct JobLogArgs {
    /// Pipeline ID.
    #[arg(long)]
    pub pipeline_id: String,
    /// Run ID.
    #[arg(long)]
    pub run_id: String,
    /// Job ID.
    #[arg(long)]
    pub job_id: String,
}

// ─────────────────────────── Connections ────────────────────────────────

/// Arguments for `flow connections`.
#[derive(Debug, Args)]
pub struct ConnectionsArgs {
    #[command(subcommand)]
    pub command: ConnectionsCmds,
}

/// Service connection operations.
#[derive(Debug, Subcommand)]
pub enum ConnectionsCmds {
    /// List service connections.
    List(ConnectionsListArgs),
}

/// Arguments for `flow connections list`.
#[derive(Debug, Args)]
pub struct ConnectionsListArgs {
    /// Filter by connection type (optional).
    #[arg(long, name = "type")]
    pub conn_type: Option<String>,
}

// ─────────────────────────── Execute ────────────────────────────────────

/// Execute the `flow` subcommand tree.
pub async fn execute(
    args: &FlowArgs,
    format: &OutputFormat,
    cli_token: Option<&str>,
    cli_domain: Option<&str>,
    cli_timeout: Option<u64>,
    cli_org_id: Option<&str>,
) -> Result<()> {
    let token = config::resolve_token(cli_token)?;
    let domain = config::resolve_domain(cli_domain);
    let timeout = config::resolve_timeout(cli_timeout);
    let org_id = config::resolve_org_id(cli_org_id);
    let client = ApiClient::new(&token, &domain, timeout)?;

    match &args.command {
        FlowCommands::Pipelines(p) => exec_pipelines(p, &client, &org_id, format).await,
        FlowCommands::Runs(r) => exec_runs(r, &client, &org_id, format).await,
        FlowCommands::Jobs(j) => exec_jobs(j, &client, &org_id, format).await,
        FlowCommands::Connections(c) => exec_connections(c, &client, &org_id, format).await,
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

// ─────────────────────────── Pipelines ──────────────────────────────────

/// Execute pipeline sub-operations.
async fn exec_pipelines(
    args: &PipelinesArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        PipelinesCmds::List(l) => {
            let page = l.page.to_string();
            let per_page = l.per_page.to_string();
            let mut params: Vec<(&str, &str)> = vec![
                ("page", page.as_str()),
                ("perPage", per_page.as_str()),
            ];
            if let Some(ref kw) = l.keyword {
                params.push(("keyword", kw.as_str()));
            }
            let data = client
                .get(
                    &format!("/oapi/v1/flow/organizations/{oid}/pipelines"),
                    &params,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        PipelinesCmds::Get(g) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/flow/organizations/{oid}/pipelines/{}",
                        g.pipeline_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        PipelinesCmds::Update(u) => {
            let body = json!({"yaml": u.yaml});
            let data = client
                .put(
                    &format!(
                        "/oapi/v1/flow/organizations/{oid}/pipelines/{}",
                        u.pipeline_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

// ─────────────────────────── Runs ───────────────────────────────────────

/// Execute run sub-operations.
async fn exec_runs(
    args: &RunsArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        RunsCmds::List(l) => {
            let page = l.page.to_string();
            let per_page = l.per_page.to_string();
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/flow/organizations/{oid}/pipelines/{}/runs",
                        l.pipeline_id
                    ),
                    &[("page", page.as_str()), ("perPage", per_page.as_str())],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        RunsCmds::Get(g) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/flow/organizations/{oid}/pipelines/{}/runs/{}",
                        g.pipeline_id, g.run_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        RunsCmds::Create(c) => {
            let body = if let Some(ref p) = c.params {
                // Parse user-supplied JSON parameters
                serde_json::from_str(p).map_err(|e| {
                    crate::error::CliError::Api(format!("Invalid params JSON: {e}"))
                })?
            } else {
                json!({})
            };
            let data = client
                .post(
                    &format!(
                        "/oapi/v1/flow/organizations/{oid}/pipelines/{}/runs",
                        c.pipeline_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        RunsCmds::Latest(l) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/flow/organizations/{oid}/pipelines/{}/runs/latestPipelineRun",
                        l.pipeline_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

// ─────────────────────────── Jobs ───────────────────────────────────────

/// Execute job sub-operations.
async fn exec_jobs(
    args: &JobsArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        JobsCmds::List(l) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/flow/organizations/{oid}/pipelines/{}/jobs",
                        l.pipeline_id
                    ),
                    &[("category", l.category.as_str())],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        JobsCmds::History(h) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/flow/organizations/{oid}/pipelines/{}/jobs/{}/history",
                        h.pipeline_id, h.job_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        JobsCmds::Run(r) => {
            let body = json!({});
            let data = client
                .post(
                    &format!(
                        "/oapi/v1/flow/organizations/{oid}/pipelines/{}/runs/{}/jobs/{}/run",
                        r.pipeline_id, r.run_id, r.job_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        JobsCmds::Log(l) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/flow/organizations/{oid}/pipelines/{}/runs/{}/jobs/{}/log",
                        l.pipeline_id, l.run_id, l.job_id
                    ),
                    &[],
                )
                .await?;
            // For logs, print as text regardless of format setting to preserve readability.
            if let Some(log_content) = data.get("log").and_then(|l| l.as_str()) {
                println!("{log_content}");
            } else {
                output::print_output(&data, format)?;
            }
        }
    }
    Ok(())
}

// ─────────────────────────── Connections ────────────────────────────────

/// Execute connection sub-operations.
async fn exec_connections(
    args: &ConnectionsArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        ConnectionsCmds::List(l) => {
            let mut params: Vec<(&str, &str)> = vec![];
            if let Some(ref t) = l.conn_type {
                params.push(("type", t.as_str()));
            }
            let data = client
                .get(
                    &format!("/oapi/v1/flow/organizations/{oid}/serviceConnections"),
                    &params,
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}
