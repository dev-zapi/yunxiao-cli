//! `flow` subcommand – pipeline management.
//!
//! Covers pipeline definitions and runs, job logs, service connections, and
//! organization-scoped Flow resource queries via the Yunxiao Flow API.

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
    /// Inspect deployment orders and machine logs.
    Deploy(DeployArgs),
    /// Manage private and public host groups.
    HostGroups(HostGroupsArgs),
    /// Manage pipeline groups.
    PipelineGroups(PipelineGroupsArgs),
    /// List resource members.
    ResourceMembers(ResourceMembersArgs),
    /// Manage Flow tag groups.
    Tags(TagsArgs),
    /// Manage Flow variable groups.
    VariableGroups(VariableGroupsArgs),
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
    /// Create a new pipeline.
    Create(PipelineCreateArgs),
    /// Update pipeline YAML definition.
    Update(PipelineUpdateArgs),
    /// Delete a pipeline.
    Delete(PipelineDeleteArgs),
    /// Generate pipeline YAML template.
    Template(PipelineTemplateArgs),
    /// Get a pipeline artifact download URL.
    ArtifactUrl(PipelineArtifactUrlArgs),
    /// Get an EMAS artifact download URL.
    EmasArtifactUrl(PipelineEmasArtifactUrlArgs),
    /// Get a pipeline scan report URL.
    ScanReportUrl(PipelineScanReportUrlArgs),
    /// Inspect pipeline relations.
    Relations(PipelineRelationsArgs),
}

/// Arguments for `flow pipelines list`.
#[derive(Debug, Args)]
pub struct PipelineListArgs {
    /// Filter by pipeline name.
    #[arg(long)]
    pub pipeline_name: Option<String>,
    /// Filter by creation start time (milliseconds timestamp).
    #[arg(long)]
    pub create_start_time: Option<i64>,
    /// Filter by creation end time (milliseconds timestamp).
    #[arg(long)]
    pub create_end_time: Option<i64>,
    /// Filter by execution start time (milliseconds timestamp).
    #[arg(long)]
    pub execute_start_time: Option<i64>,
    /// Filter by execution end time (milliseconds timestamp).
    #[arg(long)]
    pub execute_end_time: Option<i64>,
    /// Filter by status list (comma-separated: SUCCESS,RUNNING,FAIL,CANCELED,WAITING).
    #[arg(long)]
    pub status_list: Option<String>,
    /// Page number (1-based).
    #[arg(long, default_value = "1")]
    pub page: u32,
    /// Results per page (max 30).
    #[arg(long, default_value = "10")]
    pub per_page: u32,
}

/// Arguments for `flow pipelines get`.
#[derive(Debug, Args)]
pub struct PipelineGetArgs {
    /// Pipeline ID. Get via: yunxiao flow pipelines list
    #[arg(long)]
    pub pipeline_id: String,
}

/// Arguments for `flow pipelines artifact-url`.
#[derive(Debug, Args)]
pub struct PipelineArtifactUrlArgs {
    /// Artifact file path.
    #[arg(long)]
    pub file_path: String,
    /// Artifact file name.
    #[arg(long)]
    pub file_name: String,
}

/// Arguments for `flow pipelines emas-artifact-url`.
#[derive(Debug, Args)]
pub struct PipelineEmasArtifactUrlArgs {
    /// EMAS job instance ID.
    #[arg(long)]
    pub emas_job_instance_id: String,
    /// Artifact MD5 hash.
    #[arg(long)]
    pub md5: String,
    /// Pipeline ID.
    #[arg(long)]
    pub pipeline_id: String,
    /// Pipeline run ID.
    #[arg(long)]
    pub pipeline_run_id: String,
    /// Service connection ID.
    #[arg(long)]
    pub service_connection_id: String,
}

/// Arguments for `flow pipelines scan-report-url`.
#[derive(Debug, Args)]
pub struct PipelineScanReportUrlArgs {
    /// Scan report path.
    #[arg(long)]
    pub report_path: String,
}

/// Arguments for `flow pipelines relations`.
#[derive(Debug, Args)]
pub struct PipelineRelationsArgs {
    #[command(subcommand)]
    pub command: PipelineRelationsCmds,
}

/// Pipeline relation operations.
#[derive(Debug, Subcommand)]
pub enum PipelineRelationsCmds {
    /// List relations of a pipeline.
    List(PipelineRelationsListArgs),
}

/// Arguments for `flow pipelines relations list`.
#[derive(Debug, Args)]
pub struct PipelineRelationsListArgs {
    /// Pipeline ID.
    #[arg(long)]
    pub pipeline_id: String,
    /// Related object type, for example VARIABLE_GROUP.
    #[arg(long)]
    pub rel_object_type: String,
}

/// Arguments for `flow pipelines create`.
#[derive(Debug, Args)]
#[command(group(
    clap::ArgGroup::new("pipeline_content")
        .required(true)
        .multiple(false)
        .args(["content", "content_file"])
))]
pub struct PipelineCreateArgs {
    /// Pipeline name (max 60 characters).
    #[arg(long)]
    pub name: String,
    /// Pipeline YAML content (stages format). Use --content-file for long YAML.
    /// Reference: https://help.aliyun.com/zh/yunxiao/user-guide/yaml-pipeline-syntax
    #[arg(long)]
    pub content: Option<String>,
    /// Path to YAML file containing pipeline definition (stages format).
    /// Reference: https://help.aliyun.com/zh/yunxiao/user-guide/yaml-pipeline-syntax
    #[arg(long)]
    pub content_file: Option<String>,
}

/// Arguments for `flow pipelines update`.
#[derive(Debug, Args)]
#[command(group(
    clap::ArgGroup::new("pipeline_content")
        .required(true)
        .multiple(false)
        .args(["content", "content_file"])
))]
pub struct PipelineUpdateArgs {
    /// Pipeline ID. Get via: yunxiao flow pipelines list
    #[arg(long)]
    pub pipeline_id: String,
    /// Pipeline name (max 60 characters).
    #[arg(long)]
    pub name: String,
    /// Pipeline YAML content (stages format). Use --content-file for long YAML.
    /// Reference: https://help.aliyun.com/zh/yunxiao/user-guide/yaml-pipeline-syntax
    #[arg(long)]
    pub content: Option<String>,
    /// Path to YAML file containing pipeline definition (stages format).
    /// Reference: https://help.aliyun.com/zh/yunxiao/user-guide/yaml-pipeline-syntax
    #[arg(long)]
    pub content_file: Option<String>,
}

/// Arguments for `flow pipelines delete`.
#[derive(Debug, Args)]
pub struct PipelineDeleteArgs {
    /// Pipeline ID. Get via: yunxiao flow pipelines list
    #[arg(long)]
    pub pipeline_id: String,
}

/// Arguments for `flow pipelines template`.
#[derive(Debug, Args)]
pub struct PipelineTemplateArgs {
    /// Template type: simple, maven, docker, maven-docker, node, golang
    #[arg(long, default_value = "simple")]
    pub template_type: String,
    /// Output file path (optional). If not provided, prints to stdout.
    #[arg(long)]
    pub file: Option<String>,
    /// Codeup repository clone URL. Requires --service-connection-uuid.
    #[arg(long, requires = "service_connection_uuid")]
    pub codeup_repo: Option<String>,
    /// Codeup service connection UUID (from `flow connections list` response's `uuid` field).
    /// Requires --codeup-repo. Must NOT be a numeric ID; use the UUID string.
    #[arg(long, requires = "codeup_repo")]
    pub service_connection_uuid: Option<String>,
    /// YAML source ID (letters, digits, and underscores; defaults to repo).
    #[arg(long, requires = "codeup_repo")]
    pub source_id: Option<String>,
    /// Optional display name for the code source.
    #[arg(long, requires = "codeup_repo")]
    pub source_name: Option<String>,
    /// Source branch (defaults to master).
    #[arg(long, requires = "codeup_repo")]
    pub branch: Option<String>,
    /// Comma-separated source trigger events (defaults to push).
    #[arg(long, requires = "codeup_repo")]
    pub trigger_events: Option<String>,
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
    /// Pipeline ID. Get via: yunxiao flow pipelines list
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
    /// Pipeline ID. Get via: yunxiao flow pipelines list
    #[arg(long)]
    pub pipeline_id: String,
    /// Run ID. Get via: yunxiao flow runs list --pipeline-id <PIPELINE_ID>
    #[arg(long)]
    pub run_id: String,
}

/// Arguments for `flow runs create`.
#[derive(Debug, Args)]
pub struct RunCreateArgs {
    /// Pipeline ID. Get via: yunxiao flow pipelines list
    #[arg(long)]
    pub pipeline_id: String,
    /// Run parameters as a JSON string (optional).
    #[arg(long)]
    pub params: Option<String>,
}

/// Arguments for `flow runs latest`.
#[derive(Debug, Args)]
pub struct RunLatestArgs {
    /// Pipeline ID. Get via: yunxiao flow pipelines list
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
    /// List steps for a pipeline job run.
    Steps(JobStepsArgs),
    /// Get a step log page.
    StepLog(JobStepLogArgs),
    /// Get a step log URL.
    StepLogUrl(JobStepLogUrlArgs),
}

/// Arguments for `flow jobs list`.
#[derive(Debug, Args)]
pub struct JobListArgs {
    /// Pipeline ID. Get via: yunxiao flow pipelines list
    #[arg(long)]
    pub pipeline_id: String,
    /// Job category (e.g. BUILD, DEPLOY, TEST).
    #[arg(long)]
    pub category: String,
}

/// Arguments for `flow jobs history`.
#[derive(Debug, Args)]
pub struct JobHistoryArgs {
    /// Pipeline ID. Get via: yunxiao flow pipelines list
    #[arg(long)]
    pub pipeline_id: String,
    /// Job ID. Get via: yunxiao flow jobs list --pipeline-id <PIPELINE_ID> --run-id <RUN_ID>
    #[arg(long)]
    pub job_id: String,
}

/// Arguments for `flow jobs run`.
#[derive(Debug, Args)]
pub struct JobRunArgs {
    /// Pipeline ID. Get via: yunxiao flow pipelines list
    #[arg(long)]
    pub pipeline_id: String,
    /// Run ID. Get via: yunxiao flow runs list --pipeline-id <PIPELINE_ID>
    #[arg(long)]
    pub run_id: String,
    /// Job ID. Get via: yunxiao flow jobs list --pipeline-id <PIPELINE_ID> --run-id <RUN_ID>
    #[arg(long)]
    pub job_id: String,
}

/// Arguments for `flow jobs log`.
#[derive(Debug, Args)]
pub struct JobLogArgs {
    /// Pipeline ID. Get via: yunxiao flow pipelines list
    #[arg(long)]
    pub pipeline_id: String,
    /// Run ID. Get via: yunxiao flow runs list --pipeline-id <PIPELINE_ID>
    #[arg(long)]
    pub run_id: String,
    /// Job ID. Get via: yunxiao flow jobs list --pipeline-id <PIPELINE_ID> --run-id <RUN_ID>
    #[arg(long)]
    pub job_id: String,
}

/// Arguments for `flow jobs steps`.
#[derive(Debug, Args)]
pub struct JobStepsArgs {
    /// Pipeline ID.
    #[arg(long)]
    pub pipeline_id: String,
    /// Pipeline run ID.
    #[arg(long, visible_alias = "run-id")]
    pub pipeline_run_id: String,
    /// Job ID.
    #[arg(long)]
    pub job_id: String,
}

/// Arguments for `flow jobs step-log`.
#[derive(Debug, Args)]
pub struct JobStepLogArgs {
    /// Pipeline ID.
    #[arg(long)]
    pub pipeline_id: String,
    /// Pipeline run ID.
    #[arg(long, visible_alias = "run-id")]
    pub pipeline_run_id: String,
    /// Job ID.
    #[arg(long)]
    pub job_id: String,
    /// Zero-based step index.
    #[arg(long)]
    pub step_index: u32,
    /// Log offset.
    #[arg(long)]
    pub offset: u64,
    /// Maximum log bytes/lines to return, as defined by the API.
    #[arg(long)]
    pub limit: u64,
    /// Build ID.
    #[arg(long)]
    pub build_id: String,
}

/// Arguments for `flow jobs step-log-url`.
#[derive(Debug, Args)]
pub struct JobStepLogUrlArgs {
    /// Pipeline ID.
    #[arg(long)]
    pub pipeline_id: String,
    /// Pipeline run ID.
    #[arg(long, visible_alias = "run-id")]
    pub pipeline_run_id: String,
    /// Job ID.
    #[arg(long)]
    pub job_id: String,
    /// Zero-based step index.
    #[arg(long)]
    pub step_index: u32,
    /// Build ID.
    #[arg(long)]
    pub build_id: String,
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
    /// Manage service authentication definitions.
    Auths(ServiceAuthsArgs),
    /// Manage service credential definitions.
    Credentials(ServiceCredentialsArgs),
}

/// Arguments for `flow connections list`.
#[derive(Debug, Args)]
pub struct ConnectionsListArgs {
    /// Required service connection type, for example codeup.
    #[arg(long = "type", visible_alias = "conn-type")]
    pub conn_type: String,
}

/// Arguments for `flow connections auths`.
#[derive(Debug, Args)]
pub struct ServiceAuthsArgs {
    #[command(subcommand)]
    pub command: ServiceAuthsCmds,
}

/// Service authentication operations.
#[derive(Debug, Subcommand)]
pub enum ServiceAuthsCmds {
    /// List service authentication definitions.
    List(ServiceAuthsListArgs),
}

/// Arguments for `flow connections auths list`.
#[derive(Debug, Args)]
pub struct ServiceAuthsListArgs {
    /// Service authentication type, for example RAM.
    #[arg(long)]
    pub service_auth_type: String,
}

/// Arguments for `flow connections credentials`.
#[derive(Debug, Args)]
pub struct ServiceCredentialsArgs {
    #[command(subcommand)]
    pub command: ServiceCredentialsCmds,
}

/// Service credential operations.
#[derive(Debug, Subcommand)]
pub enum ServiceCredentialsCmds {
    /// List service credential definitions.
    List(ServiceCredentialsListArgs),
}

/// Arguments for `flow connections credentials list`.
#[derive(Debug, Args)]
pub struct ServiceCredentialsListArgs {
    /// Service credential type, for example username_password.
    #[arg(long)]
    pub service_credential_type: String,
}

/// Arguments for `flow deploy`.
#[derive(Debug, Args)]
pub struct DeployArgs {
    #[command(subcommand)]
    pub command: DeployCmds,
}

/// Deployment query operations.
#[derive(Debug, Subcommand)]
pub enum DeployCmds {
    /// Query a deployment order.
    Order(DeployOrderArgs),
    /// Query a deployment machine log.
    MachineLog(DeployMachineLogArgs),
}

/// Arguments for `flow deploy order`.
#[derive(Debug, Args)]
pub struct DeployOrderArgs {
    #[command(subcommand)]
    pub command: DeployOrderCmds,
}

/// Deployment order operations.
#[derive(Debug, Subcommand)]
pub enum DeployOrderCmds {
    /// Get deployment order details.
    Get(DeployOrderGetArgs),
}

/// Arguments for `flow deploy order get`.
#[derive(Debug, Args)]
pub struct DeployOrderGetArgs {
    /// Pipeline ID.
    #[arg(long)]
    pub pipeline_id: String,
    /// Deployment order ID.
    #[arg(long)]
    pub deploy_order_id: String,
}

/// Arguments for `flow deploy machine-log`.
#[derive(Debug, Args)]
pub struct DeployMachineLogArgs {
    /// Pipeline ID.
    #[arg(long)]
    pub pipeline_id: String,
    /// Deployment order ID.
    #[arg(long)]
    pub deploy_order_id: String,
    /// Deployment machine serial number.
    #[arg(long)]
    pub machine_sn: String,
}

/// Arguments for `flow host-groups`.
#[derive(Debug, Args)]
pub struct HostGroupsArgs {
    #[command(subcommand)]
    pub command: HostGroupsCmds,
}

/// Host group query operations.
#[derive(Debug, Subcommand)]
pub enum HostGroupsCmds {
    /// List host groups.
    List(HostGroupsListArgs),
    /// Get a host group.
    Get(HostGroupGetArgs),
}

/// Arguments for `flow host-groups list`.
#[derive(Debug, Args)]
pub struct HostGroupsListArgs {
    /// Host group IDs, comma-separated.
    #[arg(long)]
    pub ids: Option<String>,
    /// Filter by host group name.
    #[arg(long)]
    pub name: Option<String>,
    /// Creation start time (milliseconds timestamp).
    #[arg(long)]
    pub create_start_time: Option<i64>,
    /// Creation end time (milliseconds timestamp).
    #[arg(long)]
    pub create_end_time: Option<i64>,
    /// Creator account IDs, comma-separated.
    #[arg(long)]
    pub creator_account_ids: Option<String>,
    /// Page number.
    #[arg(long, default_value = "1")]
    pub page: u32,
    /// Results per page (max 30).
    #[arg(long, default_value = "10")]
    pub per_page: u32,
    /// Sort field.
    #[arg(long, default_value = "ID")]
    pub page_sort: String,
    /// Sort order.
    #[arg(long, default_value = "DESC")]
    pub page_order: String,
}

/// Arguments for `flow host-groups get`.
#[derive(Debug, Args)]
pub struct HostGroupGetArgs {
    /// Host group ID.
    #[arg(long)]
    pub id: String,
}

/// Arguments for `flow pipeline-groups`.
#[derive(Debug, Args)]
pub struct PipelineGroupsArgs {
    #[command(subcommand)]
    pub command: PipelineGroupsCmds,
}

/// Pipeline group query operations.
#[derive(Debug, Subcommand)]
pub enum PipelineGroupsCmds {
    /// List pipeline groups.
    List(PipelineGroupsListArgs),
    /// Get a pipeline group.
    Get(PipelineGroupGetArgs),
    /// List pipelines in a group.
    Pipelines(PipelineGroupPipelinesArgs),
}

/// Arguments for `flow pipeline-groups list`.
#[derive(Debug, Args)]
pub struct PipelineGroupsListArgs {
    /// Page number.
    #[arg(long, default_value = "1")]
    pub page: u32,
    /// Results per page (max 30).
    #[arg(long, default_value = "10")]
    pub per_page: u32,
}

/// Arguments for `flow pipeline-groups get`.
#[derive(Debug, Args)]
pub struct PipelineGroupGetArgs {
    /// Pipeline group ID.
    #[arg(long)]
    pub group_id: String,
}

/// Arguments for `flow pipeline-groups pipelines`.
#[derive(Debug, Args)]
pub struct PipelineGroupPipelinesArgs {
    /// Pipeline group ID. Use 0 for ungrouped pipelines.
    #[arg(long)]
    pub group_id: String,
    /// Filter by creation start time (milliseconds timestamp).
    #[arg(long)]
    pub create_start_time: Option<i64>,
    /// Filter by creation end time (milliseconds timestamp).
    #[arg(long)]
    pub create_end_time: Option<i64>,
    /// Filter by execution start time (milliseconds timestamp).
    #[arg(long)]
    pub execute_start_time: Option<i64>,
    /// Filter by execution end time (milliseconds timestamp).
    #[arg(long)]
    pub execute_end_time: Option<i64>,
    /// Filter by pipeline name.
    #[arg(long)]
    pub pipeline_name: Option<String>,
    /// Filter by status list (comma-separated).
    #[arg(long)]
    pub status_list: Option<String>,
    /// Page number.
    #[arg(long, default_value = "1")]
    pub page: u32,
    /// Results per page (max 30).
    #[arg(long, default_value = "10")]
    pub per_page: u32,
}

/// Arguments for `flow resource-members`.
#[derive(Debug, Args)]
pub struct ResourceMembersArgs {
    #[command(subcommand)]
    pub command: ResourceMembersCmds,
}

/// Resource member query operations.
#[derive(Debug, Subcommand)]
pub enum ResourceMembersCmds {
    /// List members of a resource.
    List(ResourceMembersListArgs),
}

/// Arguments for `flow resource-members list`.
#[derive(Debug, Args)]
pub struct ResourceMembersListArgs {
    /// Resource type, for example pipeline or hostGroup.
    #[arg(long)]
    pub resource_type: String,
    /// Resource ID.
    #[arg(long)]
    pub resource_id: String,
}

/// Arguments for `flow tags`.
#[derive(Debug, Args)]
pub struct TagsArgs {
    #[command(subcommand)]
    pub command: TagsCmds,
}

/// Flow tag query operations.
#[derive(Debug, Subcommand)]
pub enum TagsCmds {
    /// List tag groups.
    List,
    /// Get a tag group.
    Get(TagGroupGetArgs),
}

/// Arguments for `flow tags get`.
#[derive(Debug, Args)]
pub struct TagGroupGetArgs {
    /// Tag group ID.
    #[arg(long)]
    pub id: String,
}

/// Arguments for `flow variable-groups`.
#[derive(Debug, Args)]
pub struct VariableGroupsArgs {
    #[command(subcommand)]
    pub command: VariableGroupsCmds,
}

/// Variable group query operations.
#[derive(Debug, Subcommand)]
pub enum VariableGroupsCmds {
    /// List variable groups.
    List(VariableGroupsListArgs),
    /// Get a variable group.
    Get(VariableGroupGetArgs),
}

/// Arguments for `flow variable-groups list`.
#[derive(Debug, Args)]
pub struct VariableGroupsListArgs {
    /// Page number.
    #[arg(long, default_value = "1")]
    pub page: u32,
    /// Results per page (max 30).
    #[arg(long, default_value = "10")]
    pub per_page: u32,
    /// Sort field.
    #[arg(long, default_value = "ID")]
    pub page_sort: String,
    /// Sort order.
    #[arg(long, default_value = "DESC")]
    pub page_order: String,
}

/// Arguments for `flow variable-groups get`.
#[derive(Debug, Args)]
pub struct VariableGroupGetArgs {
    /// Variable group ID.
    #[arg(long)]
    pub id: String,
}

// ─────────────────────────── Execute ────────────────────────────────────

/// Execute the `flow` subcommand tree.
pub async fn execute(
    args: &FlowArgs,
    format: &OutputFormat,
    cli_token: Option<&str>,
    cli_endpoint: Option<&str>,
    cli_timeout: Option<u64>,
    cli_org_id: Option<&str>,
) -> Result<()> {
    // Templates are intentionally offline: they neither need credentials nor
    // instantiate an HTTP client.
    if let FlowCommands::Pipelines(PipelinesArgs {
        command: PipelinesCmds::Template(template),
    }) = &args.command
    {
        return write_pipeline_template(template);
    }

    let token = config::resolve_token(cli_token)?;
    let endpoint = config::resolve_endpoint(cli_endpoint);
    let timeout = config::resolve_timeout(cli_timeout);
    let org_id = config::resolve_org_id(cli_org_id);
    let client = ApiClient::new(&token, &endpoint, timeout)?;

    match &args.command {
        FlowCommands::Pipelines(p) => exec_pipelines(p, &client, &org_id, format).await,
        FlowCommands::Runs(r) => exec_runs(r, &client, &org_id, format).await,
        FlowCommands::Jobs(j) => exec_jobs(j, &client, &org_id, format).await,
        FlowCommands::Connections(c) => exec_connections(c, &client, &org_id, format).await,
        FlowCommands::Deploy(d) => exec_deploy(d, &client, &org_id, format).await,
        FlowCommands::HostGroups(h) => exec_host_groups(h, &client, &org_id, format).await,
        FlowCommands::PipelineGroups(g) => exec_pipeline_groups(g, &client, &org_id, format).await,
        FlowCommands::ResourceMembers(r) => {
            exec_resource_members(r, &client, &org_id, format).await
        }
        FlowCommands::Tags(t) => exec_tags(t, &client, &org_id, format).await,
        FlowCommands::VariableGroups(v) => exec_variable_groups(v, &client, &org_id, format).await,
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

/// Convert owned query values into the borrowed form expected by `ApiClient`.
fn query_refs(params: &[(String, String)]) -> Vec<(&str, &str)> {
    params
        .iter()
        .map(|(key, value)| (key.as_str(), value.as_str()))
        .collect()
}

/// Add an optional query parameter when the caller supplied it.
fn push_optional_param(params: &mut Vec<(String, String)>, key: &str, value: Option<String>) {
    if let Some(value) = value {
        params.push((key.to_string(), value));
    }
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
            let mut params: Vec<(&str, String)> =
                vec![("page", page.clone()), ("perPage", per_page.clone())];

            if let Some(ref name) = l.pipeline_name {
                params.push(("pipelineName", name.clone()));
            }
            if let Some(start) = l.create_start_time {
                params.push(("createStartTime", start.to_string()));
            }
            if let Some(end) = l.create_end_time {
                params.push(("createEndTime", end.to_string()));
            }
            if let Some(start) = l.execute_start_time {
                params.push(("executeStartTime", start.to_string()));
            }
            if let Some(end) = l.execute_end_time {
                params.push(("executeEndTime", end.to_string()));
            }
            if let Some(ref status) = l.status_list {
                params.push(("statusList", status.clone()));
            }

            let params_str: Vec<(&str, &str)> =
                params.iter().map(|(k, v)| (*k, v.as_str())).collect();

            let data = client
                .get(
                    &format!("/oapi/v1/flow/organizations/{oid}/pipelines"),
                    &params_str,
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
        PipelinesCmds::ArtifactUrl(a) => {
            let params = vec![
                ("filePath".to_string(), a.file_path.clone()),
                ("fileName".to_string(), a.file_name.clone()),
            ];
            let data = client
                .get(
                    &format!("/oapi/v1/flow/organizations/{oid}/pipelines/getArtifactDownloadUrl"),
                    &query_refs(&params),
                )
                .await?;
            output::print_output(&data, format)?;
        }
        PipelinesCmds::EmasArtifactUrl(a) => {
            let params = vec![
                (
                    "emasJobInstanceId".to_string(),
                    a.emas_job_instance_id.clone(),
                ),
                ("md5".to_string(), a.md5.clone()),
                ("pipelineId".to_string(), a.pipeline_id.clone()),
                ("pipelineRunId".to_string(), a.pipeline_run_id.clone()),
                (
                    "serviceConnectionId".to_string(),
                    a.service_connection_id.clone(),
                ),
            ];
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/flow/organizations/{oid}/pipelines/getEmasArtifactDownloadUrl"
                    ),
                    &query_refs(&params),
                )
                .await?;
            output::print_output(&data, format)?;
        }
        PipelinesCmds::ScanReportUrl(a) => {
            let params = vec![("reportPath".to_string(), a.report_path.clone())];
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/flow/organizations/{oid}/pipelines/getPipelineScanReportUrl"
                    ),
                    &query_refs(&params),
                )
                .await?;
            output::print_output(&data, format)?;
        }
        PipelinesCmds::Relations(r) => {
            let PipelineRelationsCmds::List(l) = &r.command;
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/flow/organizations/{oid}/pipelines/{}/pipelineObjRel/{}/list",
                        l.pipeline_id, l.rel_object_type
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        PipelinesCmds::Create(c) => {
            let content = read_pipeline_content(c.content.as_deref(), c.content_file.as_deref())?;

            let body = json!({
                "name": c.name,
                "content": content,
            });
            let data = client
                .post(
                    &format!("/oapi/v1/flow/organizations/{oid}/pipelines"),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        PipelinesCmds::Update(u) => {
            let content = read_pipeline_content(u.content.as_deref(), u.content_file.as_deref())?;

            let body = json!({
                "name": u.name,
                "content": content,
            });
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
        PipelinesCmds::Delete(d) => {
            let data = client
                .delete(
                    &format!(
                        "/oapi/v1/flow/organizations/{oid}/pipelines/{}",
                        d.pipeline_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        PipelinesCmds::Template(t) => {
            write_pipeline_template(t)?;
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
                serde_json::from_str(p)
                    .map_err(|e| crate::error::CliError::Api(format!("Invalid params JSON: {e}")))?
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
        JobsCmds::Steps(s) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/flow/organizations/{oid}/pipelines/{}/pipelineRuns/{}/jobs/{}/steps",
                        s.pipeline_id, s.pipeline_run_id, s.job_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        JobsCmds::StepLog(s) => {
            let params = vec![
                ("stepIndex".to_string(), s.step_index.to_string()),
                ("offset".to_string(), s.offset.to_string()),
                ("limit".to_string(), s.limit.to_string()),
                ("buildId".to_string(), s.build_id.clone()),
            ];
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/flow/organizations/{oid}/pipelines/{}/pipelineRuns/{}/jobs/{}/step/log",
                        s.pipeline_id, s.pipeline_run_id, s.job_id
                    ),
                    &query_refs(&params),
                )
                .await?;
            output::print_output(&data, format)?;
        }
        JobsCmds::StepLogUrl(s) => {
            let params = vec![
                ("stepIndex".to_string(), s.step_index.to_string()),
                ("buildId".to_string(), s.build_id.clone()),
            ];
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/flow/organizations/{oid}/pipelines/{}/pipelineRuns/{}/jobs/{}/step/log/url",
                        s.pipeline_id, s.pipeline_run_id, s.job_id
                    ),
                    &query_refs(&params),
                )
                .await?;
            output::print_output(&data, format)?;
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
            let data = client
                .get(
                    &format!("/oapi/v1/flow/organizations/{oid}/serviceConnections"),
                    &[("serviceConnectionType", l.conn_type.as_str())],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        ConnectionsCmds::Auths(a) => {
            let ServiceAuthsCmds::List(l) = &a.command;
            let params = vec![("serviceAuthType".to_string(), l.service_auth_type.clone())];
            let data = client
                .get(
                    &format!("/oapi/v1/flow/organizations/{oid}/serviceAuths"),
                    &query_refs(&params),
                )
                .await?;
            output::print_output(&data, format)?;
        }
        ConnectionsCmds::Credentials(c) => {
            let ServiceCredentialsCmds::List(l) = &c.command;
            let params = vec![(
                "serviceCredentialType".to_string(),
                l.service_credential_type.clone(),
            )];
            let data = client
                .get(
                    &format!("/oapi/v1/flow/organizations/{oid}/serviceCredentials"),
                    &query_refs(&params),
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

// ─────────────────────────── Deployment ─────────────────────────────────

/// Execute deployment query operations.
async fn exec_deploy(
    args: &DeployArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        DeployCmds::Order(order) => {
            let DeployOrderCmds::Get(g) = &order.command;
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/flow/organizations/{oid}/pipelines/{}/deploy/{}",
                        g.pipeline_id, g.deploy_order_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        DeployCmds::MachineLog(l) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/flow/organizations/{oid}/pipelines/{}/deploy/{}/machine/{}/log",
                        l.pipeline_id, l.deploy_order_id, l.machine_sn
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

// ─────────────────────────── Host Groups ────────────────────────────────

/// Execute host group query operations.
async fn exec_host_groups(
    args: &HostGroupsArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        HostGroupsCmds::List(l) => {
            let mut params = vec![
                ("page".to_string(), l.page.to_string()),
                ("perPage".to_string(), l.per_page.to_string()),
                ("pageSort".to_string(), l.page_sort.clone()),
                ("pageOrder".to_string(), l.page_order.clone()),
            ];
            push_optional_param(&mut params, "ids", l.ids.clone());
            push_optional_param(&mut params, "name", l.name.clone());
            push_optional_param(
                &mut params,
                "createStartTime",
                l.create_start_time.map(|value| value.to_string()),
            );
            push_optional_param(
                &mut params,
                "createEndTime",
                l.create_end_time.map(|value| value.to_string()),
            );
            push_optional_param(
                &mut params,
                "creatorAccountIds",
                l.creator_account_ids.clone(),
            );
            let data = client
                .get(
                    &format!("/oapi/v1/flow/organizations/{oid}/hostGroups"),
                    &query_refs(&params),
                )
                .await?;
            output::print_output(&data, format)?;
        }
        HostGroupsCmds::Get(g) => {
            let data = client
                .get(
                    &format!("/oapi/v1/flow/organizations/{oid}/hostGroups/{}", g.id),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

// ─────────────────────────── Pipeline Groups ────────────────────────────

/// Execute pipeline group query operations.
async fn exec_pipeline_groups(
    args: &PipelineGroupsArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        PipelineGroupsCmds::List(l) => {
            let params = vec![
                ("page".to_string(), l.page.to_string()),
                ("perPage".to_string(), l.per_page.to_string()),
            ];
            let data = client
                .get(
                    &format!("/oapi/v1/flow/organizations/{oid}/pipelineGroups"),
                    &query_refs(&params),
                )
                .await?;
            output::print_output(&data, format)?;
        }
        PipelineGroupsCmds::Get(g) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/flow/organizations/{oid}/pipelineGroups/{}",
                        g.group_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        PipelineGroupsCmds::Pipelines(l) => {
            let mut params = vec![
                ("groupId".to_string(), l.group_id.clone()),
                ("page".to_string(), l.page.to_string()),
                ("perPage".to_string(), l.per_page.to_string()),
            ];
            push_optional_param(
                &mut params,
                "createStartTime",
                l.create_start_time.map(|value| value.to_string()),
            );
            push_optional_param(
                &mut params,
                "createEndTime",
                l.create_end_time.map(|value| value.to_string()),
            );
            push_optional_param(
                &mut params,
                "executeStartTime",
                l.execute_start_time.map(|value| value.to_string()),
            );
            push_optional_param(
                &mut params,
                "executeEndTime",
                l.execute_end_time.map(|value| value.to_string()),
            );
            push_optional_param(&mut params, "pipelineName", l.pipeline_name.clone());
            push_optional_param(&mut params, "statusList", l.status_list.clone());
            let data = client
                .get(
                    &format!("/oapi/v1/flow/organizations/{oid}/pipelineGroups/pipelines"),
                    &query_refs(&params),
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

// ─────────────────────────── Resource Members ──────────────────────────

/// Execute resource member query operations.
async fn exec_resource_members(
    args: &ResourceMembersArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    let ResourceMembersCmds::List(l) = &args.command;
    let data = client
        .get(
            &format!(
                "/oapi/v1/flow/organizations/{oid}/resourceMembers/resourceTypes/{}/resourceIds/{}",
                l.resource_type, l.resource_id
            ),
            &[],
        )
        .await?;
    output::print_output(&data, format)?;
    Ok(())
}

// ─────────────────────────── Tags ──────────────────────────────────────

/// Execute Flow tag group query operations.
async fn exec_tags(
    args: &TagsArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        TagsCmds::List => {
            let data = client
                .get(&format!("/oapi/v1/flow/organizations/{oid}/tagGroups"), &[])
                .await?;
            output::print_output(&data, format)?;
        }
        TagsCmds::Get(g) => {
            let data = client
                .get(
                    &format!("/oapi/v1/flow/organizations/{oid}/tagGroups/{}", g.id),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

// ─────────────────────────── Variable Groups ────────────────────────────

/// Execute Flow variable group query operations.
async fn exec_variable_groups(
    args: &VariableGroupsArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        VariableGroupsCmds::List(l) => {
            let params = vec![
                ("page".to_string(), l.page.to_string()),
                ("perPage".to_string(), l.per_page.to_string()),
                ("pageSort".to_string(), l.page_sort.clone()),
                ("pageOrder".to_string(), l.page_order.clone()),
            ];
            let data = client
                .get(
                    &format!("/oapi/v1/flow/organizations/{oid}/variableGroups"),
                    &query_refs(&params),
                )
                .await?;
            output::print_output(&data, format)?;
        }
        VariableGroupsCmds::Get(g) => {
            let data = client
                .get(
                    &format!("/oapi/v1/flow/organizations/{oid}/variableGroups/{}", g.id),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

// ─────────────────────────── Template Generator ─────────────────────────────

/// Read YAML supplied directly or from a file after clap has validated the input mode.
fn read_pipeline_content(content: Option<&str>, content_file: Option<&str>) -> Result<String> {
    match (content, content_file) {
        (Some(content), None) => Ok(content.to_string()),
        (None, Some(file_path)) => std::fs::read_to_string(file_path).map_err(|error| {
            crate::error::CliError::Config(format!(
                "Failed to read content file '{file_path}': {error}"
            ))
        }),
        _ => Err(crate::error::CliError::Config(
            "Exactly one of --content or --content-file must be provided".into(),
        )),
    }
}

/// Write a locally generated template to its requested destination.
fn write_pipeline_template(args: &PipelineTemplateArgs) -> Result<()> {
    let template = generate_pipeline_template(args)?;
    if let Some(output_path) = &args.file {
        std::fs::write(output_path, &template).map_err(|error| {
            crate::error::CliError::Config(format!(
                "Failed to write template to '{output_path}': {error}"
            ))
        })?;
        println!("Template written to: {output_path}");
    } else {
        println!("{template}");
    }
    Ok(())
}

/// Generate pipeline YAML template based on type.
fn generate_pipeline_template(args: &PipelineTemplateArgs) -> Result<String> {
    let template = match args.template_type.as_str() {
        "simple" => {
            r#"# Simple Command Pipeline Template
# Reference: https://help.aliyun.com/zh/yunxiao/user-guide/yaml-pipeline-syntax

stages:
  build_stage:
    name: "构建阶段"
    jobs:
      command_job:
        name: "执行命令"
        runsOn:
          group: public/cn-beijing
          container: build-steps-public-registry.cn-beijing.cr.aliyuncs.com/build-steps/alinux3:latest
        steps:
          command_step:
            name: "执行命令"
            step: "Command"
            with:
              run: |
                echo "Hello from Yunxiao Pipeline"
                # Add your commands here
"#
        }
        "maven" => {
            r#"# Maven Build Pipeline Template
# Reference: https://help.aliyun.com/zh/yunxiao/user-guide/yaml-pipeline-syntax

stages:
  build_stage:
    name: "Maven 构建"
    jobs:
      maven_build:
        name: "Maven 编译打包"
        runsOn:
          group: public/cn-beijing
          container: maven:3.8-openjdk-17
        steps:
          maven_compile:
            name: "Maven 打包"
            step: "Command"
            with:
              run: |
                mvn clean package -DskipTests

          maven_test:
            name: "运行测试"
            step: "Command"
            with:
              run: |
                mvn test
"#
        }
        "docker" => {
            r#"# Docker Build and Push Pipeline Template
# Reference: https://help.aliyun.com/zh/yunxiao/user-guide/yaml-pipeline-syntax

stages:
  docker_stage:
    name: "Docker 镜像构建推送"
    jobs:
      docker_build:
        name: "构建推送 Docker 镜像"
        runsOn:
          group: public/cn-beijing
          container: docker:20.10
        steps:
          build_and_push:
            name: "构建推送镜像"
            step: "Command"
            with:
              run: |
                # Set image tag with timestamp
                DATETIME=$(date +%Y%m%d%H%M%S)
                IMAGE_TAG="your-registry.com/your-repo/your-app:${DATETIME}"

                # Build Docker image
                docker build -f Dockerfile -t ${IMAGE_TAG} .

                # Push to registry
                docker push ${IMAGE_TAG}

                echo "Image pushed: ${IMAGE_TAG}"
"#
        }
        "maven-docker" => {
            r#"# Maven + Docker Pipeline Template
# Reference: https://help.aliyun.com/zh/yunxiao/user-guide/yaml-pipeline-syntax

stages:
  build_stage:
    name: "构建阶段"
    jobs:
      maven_build:
        name: "Maven 编译"
        runsOn:
          group: public/cn-beijing
          container: maven:3.8-openjdk-17
        steps:
          maven_compile:
            name: "Maven 打包"
            step: "Command"
            with:
              run: |
                mvn clean package -DskipTests

      docker_build:
        name: "Docker 镜像构建推送"
        runsOn:
          group: public/cn-beijing
          container: docker:20.10
        steps:
          build_and_push:
            name: "构建推送镜像"
            step: "Command"
            with:
              run: |
                DATETIME=$(date +%Y%m%d%H%M%S)
                IMAGE_TAG="harbor.example.com/your-repo/your-app:${DATETIME}"

                docker build -f docker/Dockerfile -t ${IMAGE_TAG} .
                docker push ${IMAGE_TAG}

                echo "Image pushed: ${IMAGE_TAG}"
"#
        }
        "node" => {
            r#"# Node.js Build Pipeline Template
# Reference: https://help.aliyun.com/zh/yunxiao/user-guide/yaml-pipeline-syntax

stages:
  build_stage:
    name: "Node.js 构建"
    jobs:
      node_build:
        name: "Node.js 构建"
        runsOn:
          group: public/cn-beijing
          container: node:18-alpine
        steps:
          install_deps:
            name: "安装依赖"
            step: "Command"
            with:
              run: |
                npm ci

          build_app:
            name: "构建应用"
            step: "Command"
            with:
              run: |
                npm run build

          run_tests:
            name: "运行测试"
            step: "Command"
            with:
              run: |
                npm test
"#
        }
        "golang" => {
            r#"# Go Build Pipeline Template
# Reference: https://help.aliyun.com/zh/yunxiao/user-guide/yaml-pipeline-syntax

stages:
  build_stage:
    name: "Go 构建"
    jobs:
      go_build:
        name: "Go 编译"
        runsOn:
          group: public/cn-beijing
          container: golang:1.21-alpine
        steps:
          go_mod:
            name: "下载依赖"
            step: "Command"
            with:
              run: |
                go mod download

          go_build:
            name: "编译二进制"
            step: "Command"
            with:
              run: |
                CGO_ENABLED=0 GOOS=linux go build -o app .

          go_test:
            name: "运行测试"
            step: "Command"
            with:
              run: |
                go test -v ./...
"#
        }
        _ => {
            return Err(crate::error::CliError::Config(format!(
                "Unknown template type: {}. Available: simple, maven, docker, maven-docker, node, golang",
                args.template_type
            )));
        }
    };

    match (&args.codeup_repo, &args.service_connection_uuid) {
        (None, None) => Ok(template.to_string()),
        (Some(repo), Some(connection_uuid)) => {
            let trimmed_uuid = connection_uuid.trim();
            if trimmed_uuid.is_empty() {
                return Err(crate::error::CliError::Config(
                    "--service-connection-uuid cannot be blank".into(),
                ));
            }
            if trimmed_uuid.chars().all(|c| c.is_ascii_digit()) {
                return Err(crate::error::CliError::Config(
                    "--service-connection-uuid must be a UUID string, not a numeric ID. Get the UUID from `flow connections list` response's `uuid` field".into(),
                ));
            }

            let source_id = args.source_id.as_deref().unwrap_or("repo");
            let branch = args.branch.as_deref().unwrap_or("master");
            let trigger_events = args.trigger_events.as_deref().unwrap_or("push");
            validate_source_id(source_id)?;
            Ok(format!(
                "{}{}",
                generate_codeup_source(
                    source_id,
                    args.source_name.as_deref(),
                    repo,
                    branch,
                    trigger_events,
                    trimmed_uuid,
                ),
                template
            ))
        }
        _ => Err(crate::error::CliError::Config(
            "--codeup-repo and --service-connection-uuid must be provided together".into(),
        )),
    }
}

/// Validate the identifier constraints imposed by Flow's YAML source schema.
fn validate_source_id(source_id: &str) -> Result<()> {
    let valid = !source_id.is_empty()
        && source_id.len() <= 30
        && source_id.as_bytes()[0].is_ascii_alphabetic()
        && source_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_');
    if valid {
        Ok(())
    } else {
        Err(crate::error::CliError::Config(
            "--source-id must begin with a letter, contain only letters, digits, or underscores, and be at most 30 characters".into(),
        ))
    }
}

/// Quote a scalar so arbitrary user input cannot alter the generated YAML structure.
fn quote_yaml(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            character if character.is_control() => {
                escaped.push_str(&format!("\\u{:04X}", character as u32));
            }
            character => escaped.push(character),
        }
    }
    format!("\"{escaped}\"")
}

/// Build the documented top-level `sources` entry for one Codeup repository.
fn generate_codeup_source(
    source_id: &str,
    source_name: Option<&str>,
    repo: &str,
    branch: &str,
    trigger_events: &str,
    service_connection_uuid: &str,
) -> String {
    let events = trigger_events
        .split(',')
        .map(str::trim)
        .filter(|event| !event.is_empty())
        .map(quote_yaml)
        .collect::<Vec<_>>();
    let trigger_events = if events.is_empty() {
        "    triggerEvents: []".to_string()
    } else {
        format!(
            "    triggerEvents:\n{}",
            events
                .into_iter()
                .map(|event| format!("      - {event}"))
                .collect::<Vec<_>>()
                .join("\n")
        )
    };
    let source_name = source_name
        .map(|name| format!("    name: {}\n", quote_yaml(name)))
        .unwrap_or_default();

    format!(
        "sources:\n  {source_id}:\n    type: codeup\n{source_name}    endpoint: {}\n    branch: {}\n{trigger_events}\n    certificate:\n      type: serviceConnection\n      serviceConnection: {}\n\n",
        quote_yaml(repo),
        quote_yaml(branch),
        quote_yaml(service_connection_uuid),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use mockito::Server;

    #[derive(Debug, Parser)]
    struct TestCli {
        #[command(subcommand)]
        command: ConnectionsCmds,
    }

    #[test]
    fn connections_list_parses_type_and_legacy_alias() {
        for flag in ["--type", "--conn-type"] {
            let cli = TestCli::try_parse_from(["test", "list", flag, "codeup"])
                .expect("connection type should parse");
            let ConnectionsCmds::List(args) = cli.command else {
                panic!("expected service connection list command");
            };
            assert_eq!(args.conn_type, "codeup");
        }
        assert!(TestCli::try_parse_from(["test", "list"]).is_err());
    }

    #[tokio::test]
    async fn connections_list_uses_only_documented_query_key() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock(
                "GET",
                "/oapi/v1/flow/organizations/org-1/serviceConnections?serviceConnectionType=codeup",
            )
            .with_body("[]")
            .create_async()
            .await;
        let client = ApiClient::new("token", &server.url(), 5).unwrap();
        let args = ConnectionsArgs {
            command: ConnectionsCmds::List(ConnectionsListArgs {
                conn_type: "codeup".into(),
            }),
        };

        exec_connections(&args, &client, &Some("org-1".into()), &OutputFormat::Json)
            .await
            .unwrap();
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn pipeline_artifact_url_uses_documented_query_keys() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock(
                "GET",
                "/oapi/v1/flow/organizations/org-1/pipelines/getArtifactDownloadUrl?filePath=target%2Fapp.tgz&fileName=app.tgz",
            )
            .with_body(r#"{"url":"https://example.test/app.tgz"}"#)
            .create_async()
            .await;
        let client = ApiClient::new("token", &server.url(), 5).unwrap();
        let args = PipelinesArgs {
            command: PipelinesCmds::ArtifactUrl(PipelineArtifactUrlArgs {
                file_path: "target/app.tgz".into(),
                file_name: "app.tgz".into(),
            }),
        };

        exec_pipelines(&args, &client, &Some("org-1".into()), &OutputFormat::Json)
            .await
            .unwrap();
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn job_step_log_uses_path_and_pagination_parameters() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock(
                "GET",
                "/oapi/v1/flow/organizations/org-1/pipelines/p-1/pipelineRuns/r-1/jobs/j-1/step/log?stepIndex=2&offset=10&limit=100&buildId=b-1",
            )
            .with_body(r#"{"log":"ok"}"#)
            .create_async()
            .await;
        let client = ApiClient::new("token", &server.url(), 5).unwrap();
        let args = JobsArgs {
            command: JobsCmds::StepLog(JobStepLogArgs {
                pipeline_id: "p-1".into(),
                pipeline_run_id: "r-1".into(),
                job_id: "j-1".into(),
                step_index: 2,
                offset: 10,
                limit: 100,
                build_id: "b-1".into(),
            }),
        };

        exec_jobs(&args, &client, &Some("org-1".into()), &OutputFormat::Json)
            .await
            .unwrap();
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn host_groups_list_sends_filters_and_sorting() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock(
                "GET",
                "/oapi/v1/flow/organizations/org-1/hostGroups?page=2&perPage=5&pageSort=NAME&pageOrder=ASC&ids=1%2C2&name=build&createStartTime=100&createEndTime=200&creatorAccountIds=u-1%2Cu-2",
            )
            .with_body("[]")
            .create_async()
            .await;
        let client = ApiClient::new("token", &server.url(), 5).unwrap();
        let args = HostGroupsArgs {
            command: HostGroupsCmds::List(HostGroupsListArgs {
                ids: Some("1,2".into()),
                name: Some("build".into()),
                create_start_time: Some(100),
                create_end_time: Some(200),
                creator_account_ids: Some("u-1,u-2".into()),
                page: 2,
                per_page: 5,
                page_sort: "NAME".into(),
                page_order: "ASC".into(),
            }),
        };

        exec_host_groups(&args, &client, &Some("org-1".into()), &OutputFormat::Json)
            .await
            .unwrap();
        mock.assert_async().await;
    }

    #[test]
    fn new_flow_query_commands_parse() {
        #[derive(Debug, Parser)]
        struct Cli {
            #[command(subcommand)]
            command: FlowCommands,
        }

        for argv in [
            vec![
                "test",
                "pipelines",
                "artifact-url",
                "--file-path",
                "a",
                "--file-name",
                "b",
            ],
            vec![
                "test",
                "jobs",
                "steps",
                "--pipeline-id",
                "p",
                "--pipeline-run-id",
                "r",
                "--job-id",
                "j",
            ],
            vec!["test", "host-groups", "get", "--id", "1"],
            vec!["test", "pipeline-groups", "pipelines", "--group-id", "0"],
            vec![
                "test",
                "connections",
                "auths",
                "list",
                "--service-auth-type",
                "RAM",
            ],
            vec!["test", "tags", "list"],
            vec!["test", "variable-groups", "get", "--id", "1"],
        ] {
            assert!(Cli::try_parse_from(argv).is_ok());
        }
    }

    fn template_args() -> PipelineTemplateArgs {
        PipelineTemplateArgs {
            template_type: "simple".into(),
            file: None,
            codeup_repo: None,
            service_connection_uuid: None,
            source_id: None,
            source_name: None,
            branch: None,
            trigger_events: None,
        }
    }

    #[test]
    fn template_without_source_remains_a_valid_build_template() {
        let template = generate_pipeline_template(&template_args()).unwrap();
        assert!(template.contains("stages:"));
        assert!(!template.contains("sources:"));
    }

    #[test]
    fn template_generates_codeup_source_with_quoted_uuid() {
        let mut args = template_args();
        args.codeup_repo = Some("https://codeup.example/repo.git?x=\"quoted\"".into());
        args.service_connection_uuid = Some("  abc-123-uuid  ".into());
        args.source_name = Some("app: production".into());
        args.branch = Some("feature/new\nbranch".into());
        args.trigger_events = Some("push, tagPush".into());

        let template = generate_pipeline_template(&args).unwrap();
        assert!(template.contains("sources:\n  repo:"));
        assert!(template.contains("endpoint: \"https://codeup.example/repo.git?x=\\\"quoted\\\"\""));
        assert!(template.contains("branch: \"feature/new\\nbranch\""));
        assert!(template.contains("      - \"push\""));
        assert!(template.contains("serviceConnection: \"abc-123-uuid\""));
    }

    #[test]
    fn template_rejects_invalid_source_id() {
        let mut args = template_args();
        args.codeup_repo = Some("https://codeup.example/repo.git".into());
        args.service_connection_uuid = Some("abc-uuid".into());
        args.source_id = Some("not-valid".into());
        assert!(generate_pipeline_template(&args).is_err());
    }

    #[test]
    fn template_rejects_blank_service_connection_uuid() {
        let mut args = template_args();
        args.codeup_repo = Some("https://codeup.example/repo.git".into());
        args.service_connection_uuid = Some("   ".into());
        assert!(generate_pipeline_template(&args).is_err());
    }

    #[test]
    fn template_rejects_trimmed_numeric_service_connection_id() {
        let mut args = template_args();
        args.codeup_repo = Some("https://codeup.example/repo.git".into());
        args.service_connection_uuid = Some(" 12345 ".into());
        let error = generate_pipeline_template(&args).unwrap_err();
        let message = format!("{error}");
        assert!(
            message.contains("UUID"),
            "error should mention UUID: {message}"
        );
    }

    #[derive(Debug, Parser)]
    struct TemplateCli {
        #[command(subcommand)]
        command: PipelinesCmds,
    }

    #[test]
    fn template_requires_codeup_arguments_together() {
        assert!(TemplateCli::try_parse_from([
            "test",
            "template",
            "--codeup-repo",
            "https://codeup.example/repo.git",
        ])
        .is_err());
        assert!(TemplateCli::try_parse_from([
            "test",
            "template",
            "--service-connection-uuid",
            "abc-uuid",
        ])
        .is_err());
    }

    #[test]
    fn template_accepts_codeup_repo_with_service_connection_uuid() {
        let result = TemplateCli::try_parse_from([
            "test",
            "template",
            "--codeup-repo",
            "https://codeup.example/repo.git",
            "--service-connection-uuid",
            "abc-123-uuid",
        ]);
        assert!(result.is_ok());
        if let Ok(TemplateCli {
            command: PipelinesCmds::Template(args),
        }) = result
        {
            assert_eq!(
                args.codeup_repo,
                Some("https://codeup.example/repo.git".to_string())
            );
            assert_eq!(
                args.service_connection_uuid,
                Some("abc-123-uuid".to_string())
            );
        } else {
            panic!("Expected Template command");
        }
    }

    #[test]
    fn template_rejects_source_options_without_a_codeup_repository() {
        for args in [
            vec!["test", "template", "--source-id", "application"],
            vec!["test", "template", "--source-name", "Application"],
            vec!["test", "template", "--branch", "main"],
            vec!["test", "template", "--trigger-events", "tagPush"],
        ] {
            assert!(TemplateCli::try_parse_from(args).is_err());
        }
    }

    #[derive(Debug, Parser)]
    struct ContentCli {
        #[command(subcommand)]
        command: PipelinesCmds,
    }

    #[test]
    fn create_and_update_require_exactly_one_content_input() {
        for command in ["create", "update"] {
            let mut args = vec!["test", command];
            if command == "update" {
                args.extend(["--pipeline-id", "pipeline-1"]);
            }
            args.extend(["--name", "pipeline"]);
            let error = ContentCli::try_parse_from(&args).unwrap_err();
            let rendered = error.render().to_string();
            assert!(rendered.contains("<--content <CONTENT>|--content-file <CONTENT_FILE>>"));

            let mut with_content = args.clone();
            with_content.extend(["--content", "stages: {}"]);
            assert!(ContentCli::try_parse_from(&with_content).is_ok());

            let mut with_content_file = args.clone();
            with_content_file.extend(["--content-file", "pipeline.yaml"]);
            assert!(ContentCli::try_parse_from(&with_content_file).is_ok());

            args.extend(["--content", "stages: {}", "--content-file", "pipeline.yaml"]);
            assert!(ContentCli::try_parse_from(&args).is_err());
        }
    }
}
