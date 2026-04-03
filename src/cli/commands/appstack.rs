//! `appstack` subcommand – application delivery management.
//!
//! Covers applications, tags, variable groups, orchestrations, change orders,
//! deployments, and release workflows.

use clap::{Args, Subcommand};
use serde_json::json;

use crate::client::ApiClient;
use crate::config;
use crate::config::types::OutputFormat;
use crate::error::Result;
use crate::output;

/// Arguments for the `appstack` subcommand.
#[derive(Debug, Args)]
pub struct AppstackArgs {
    #[command(subcommand)]
    pub command: AppstackCommands,
}

/// Top-level appstack operations.
#[derive(Debug, Subcommand)]
pub enum AppstackCommands {
    /// Manage applications.
    Apps(AppsArgs),
    /// Manage application tags.
    Tags(TagsArgs),
    /// Manage variable groups.
    Vars(VarsArgs),
    /// Manage deployment orchestrations.
    Orchestrations(OrchArgs),
    /// Manage change orders.
    Changes(ChangesArgs),
    /// Manage deployments.
    Deploy(DeployArgs),
    /// Manage release workflows and stages.
    Releases(ReleasesArgs),
}

// ─────────────────────────── Apps ───────────────────────────────────────

/// Arguments for `appstack apps`.
#[derive(Debug, Args)]
pub struct AppsArgs {
    #[command(subcommand)]
    pub command: AppsCmds,
}

/// Application operations.
#[derive(Debug, Subcommand)]
pub enum AppsCmds {
    /// List applications.
    List(AppsListArgs),
    /// Get application details.
    Get(AppsGetArgs),
    /// Create a new application.
    Create(AppsCreateArgs),
    /// Update an existing application.
    Update(AppsUpdateArgs),
}

/// Arguments for `appstack apps list`.
#[derive(Debug, Args)]
pub struct AppsListArgs {
    /// Page number.
    #[arg(long, default_value = "1")]
    pub page: u32,
    /// Results per page.
    #[arg(long, default_value = "20")]
    pub per_page: u32,
}

/// Arguments for `appstack apps get`.
#[derive(Debug, Args)]
pub struct AppsGetArgs {
    /// Application name.
    #[arg(long)]
    pub app_name: String,
}

/// Arguments for `appstack apps create`.
#[derive(Debug, Args)]
pub struct AppsCreateArgs {
    /// Application name.
    #[arg(long)]
    pub app_name: String,
    /// Description (optional).
    #[arg(long)]
    pub description: Option<String>,
    /// Application type (optional).
    #[arg(long)]
    pub app_type: Option<String>,
}

/// Arguments for `appstack apps update`.
#[derive(Debug, Args)]
pub struct AppsUpdateArgs {
    /// Application name.
    #[arg(long)]
    pub app_name: String,
    /// New description (optional).
    #[arg(long)]
    pub description: Option<String>,
}

// ─────────────────────────── Tags ───────────────────────────────────────

/// Arguments for `appstack tags`.
#[derive(Debug, Args)]
pub struct TagsArgs {
    #[command(subcommand)]
    pub command: TagsCmds,
}

/// Tag operations.
#[derive(Debug, Subcommand)]
pub enum TagsCmds {
    /// Search tags by keyword.
    Search(TagSearchArgs),
    /// Create a new tag.
    Create(TagCreateArgs),
}

/// Arguments for `appstack tags search`.
#[derive(Debug, Args)]
pub struct TagSearchArgs {
    /// Search keyword.
    #[arg(long)]
    pub keyword: Option<String>,
}

/// Arguments for `appstack tags create`.
#[derive(Debug, Args)]
pub struct TagCreateArgs {
    /// Tag name.
    #[arg(long)]
    pub tag_name: String,
}

// ─────────────────────────── Vars ───────────────────────────────────────

/// Arguments for `appstack vars`.
#[derive(Debug, Args)]
pub struct VarsArgs {
    #[command(subcommand)]
    pub command: VarsCmds,
}

/// Variable group operations.
#[derive(Debug, Subcommand)]
pub enum VarsCmds {
    /// List variable groups.
    List,
    /// Get a variable group by ID.
    Get(VarsGetArgs),
    /// Create a new variable group.
    Create(VarsCreateArgs),
    /// Update a variable group.
    Update(VarsUpdateArgs),
    /// Delete a variable group.
    Delete(VarsDeleteArgs),
}

/// Arguments for `appstack vars get`.
#[derive(Debug, Args)]
pub struct VarsGetArgs {
    /// Variable group ID.
    #[arg(long)]
    pub var_id: String,
}

/// Arguments for `appstack vars create`.
#[derive(Debug, Args)]
pub struct VarsCreateArgs {
    /// Variable group name.
    #[arg(long)]
    pub name: String,
    /// Variables as a JSON string (e.g. '{"key":"value"}').
    #[arg(long)]
    pub variables: String,
}

/// Arguments for `appstack vars update`.
#[derive(Debug, Args)]
pub struct VarsUpdateArgs {
    /// Variable group ID.
    #[arg(long)]
    pub var_id: String,
    /// Variables as a JSON string.
    #[arg(long)]
    pub variables: String,
}

/// Arguments for `appstack vars delete`.
#[derive(Debug, Args)]
pub struct VarsDeleteArgs {
    /// Variable group ID.
    #[arg(long)]
    pub var_id: String,
}

// ─────────────────────────── Orchestrations ─────────────────────────────

/// Arguments for `appstack orchestrations`.
#[derive(Debug, Args)]
pub struct OrchArgs {
    #[command(subcommand)]
    pub command: OrchCmds,
}

/// Orchestration operations.
#[derive(Debug, Subcommand)]
pub enum OrchCmds {
    /// List orchestrations for an application.
    List(OrchListArgs),
    /// Get orchestration details.
    Get(OrchGetArgs),
    /// Create a new orchestration.
    Create(OrchCreateArgs),
    /// Update an orchestration.
    Update(OrchUpdateArgs),
    /// Delete an orchestration.
    Delete(OrchDeleteArgs),
}

/// Arguments for `appstack orchestrations list`.
#[derive(Debug, Args)]
pub struct OrchListArgs {
    /// Application name.
    #[arg(long)]
    pub app_name: String,
}

/// Arguments for `appstack orchestrations get`.
#[derive(Debug, Args)]
pub struct OrchGetArgs {
    /// Application name.
    #[arg(long)]
    pub app_name: String,
    /// Orchestration ID.
    #[arg(long)]
    pub orch_id: String,
}

/// Arguments for `appstack orchestrations create`.
#[derive(Debug, Args)]
pub struct OrchCreateArgs {
    /// Application name.
    #[arg(long)]
    pub app_name: String,
    /// Orchestration template content (JSON or YAML).
    #[arg(long)]
    pub template: String,
}

/// Arguments for `appstack orchestrations update`.
#[derive(Debug, Args)]
pub struct OrchUpdateArgs {
    /// Application name.
    #[arg(long)]
    pub app_name: String,
    /// Orchestration ID.
    #[arg(long)]
    pub orch_id: String,
    /// Updated template content (optional).
    #[arg(long)]
    pub template: Option<String>,
}

/// Arguments for `appstack orchestrations delete`.
#[derive(Debug, Args)]
pub struct OrchDeleteArgs {
    /// Application name.
    #[arg(long)]
    pub app_name: String,
    /// Orchestration ID.
    #[arg(long)]
    pub orch_id: String,
}

// ─────────────────────────── Changes ────────────────────────────────────

/// Arguments for `appstack changes`.
#[derive(Debug, Args)]
pub struct ChangesArgs {
    #[command(subcommand)]
    pub command: ChangesCmds,
}

/// Change-order operations.
#[derive(Debug, Subcommand)]
pub enum ChangesCmds {
    /// Create a new change order.
    Create(ChangeCreateArgs),
    /// Cancel a change order.
    Cancel(ChangeCancelArgs),
    /// Close a change order.
    Close(ChangeCloseArgs),
}

/// Arguments for `appstack changes create`.
#[derive(Debug, Args)]
pub struct ChangeCreateArgs {
    /// Application name.
    #[arg(long)]
    pub app_name: Option<String>,
    /// Orchestration ID (optional).
    #[arg(long)]
    pub orch_id: Option<String>,
    /// Description (optional).
    #[arg(long)]
    pub description: Option<String>,
}

/// Arguments for `appstack changes cancel`.
#[derive(Debug, Args)]
pub struct ChangeCancelArgs {
    /// Change order ID.
    #[arg(long)]
    pub change_id: String,
}

/// Arguments for `appstack changes close`.
#[derive(Debug, Args)]
pub struct ChangeCloseArgs {
    /// Change order ID.
    #[arg(long)]
    pub change_id: String,
}

// ─────────────────────────── Deploy ─────────────────────────────────────

/// Arguments for `appstack deploy`.
#[derive(Debug, Args)]
pub struct DeployArgs {
    #[command(subcommand)]
    pub command: DeployCmds,
}

/// Deployment operations.
#[derive(Debug, Subcommand)]
pub enum DeployCmds {
    /// Create a new deployment.
    Create(DeployCreateArgs),
    /// Get deployment details.
    Get(DeployGetArgs),
    /// Get deployment logs.
    Log(DeployLogArgs),
    /// Perform an action on a deployment (confirm, rollback, etc.).
    Action(DeployActionArgs),
}

/// Arguments for `appstack deploy create`.
#[derive(Debug, Args)]
pub struct DeployCreateArgs {
    /// Application name.
    #[arg(long)]
    pub app_name: String,
    /// Environment name (optional).
    #[arg(long)]
    pub env: Option<String>,
    /// Image or package version (optional).
    #[arg(long)]
    pub version: Option<String>,
}

/// Arguments for `appstack deploy get`.
#[derive(Debug, Args)]
pub struct DeployGetArgs {
    /// Change order ID.
    #[arg(long)]
    pub change_order_id: String,
}

/// Arguments for `appstack deploy log`.
#[derive(Debug, Args)]
pub struct DeployLogArgs {
    /// Change order ID.
    #[arg(long)]
    pub change_order_id: String,
}

/// Arguments for `appstack deploy action`.
#[derive(Debug, Args)]
pub struct DeployActionArgs {
    /// Change order ID.
    #[arg(long)]
    pub change_order_id: String,
    /// Action to perform (confirm, rollback, retry, abort).
    #[arg(long)]
    pub action: String,
}

// ─────────────────────────── Releases ───────────────────────────────────

/// Arguments for `appstack releases`.
#[derive(Debug, Args)]
pub struct ReleasesArgs {
    #[command(subcommand)]
    pub command: ReleasesCmds,
}

/// Release workflow operations.
#[derive(Debug, Subcommand)]
pub enum ReleasesCmds {
    /// List release workflows for an application.
    List(ReleasesListArgs),
    /// Manage release workflow stages.
    Stages(StagesArgs),
}

/// Arguments for `appstack releases list`.
#[derive(Debug, Args)]
pub struct ReleasesListArgs {
    /// Application name.
    #[arg(long)]
    pub app_name: String,
}

/// Arguments for `appstack releases stages`.
#[derive(Debug, Args)]
pub struct StagesArgs {
    #[command(subcommand)]
    pub command: StagesCmds,
}

/// Stage operations.
#[derive(Debug, Subcommand)]
pub enum StagesCmds {
    /// List stages in a release workflow.
    List(StagesListArgs),
    /// Execute a stage.
    Execute(StagesExecArgs),
    /// Cancel a stage execution.
    Cancel(StagesCancelArgs),
}

/// Arguments for `appstack releases stages list`.
#[derive(Debug, Args)]
pub struct StagesListArgs {
    /// Application name.
    #[arg(long)]
    pub app_name: String,
    /// Workflow ID.
    #[arg(long)]
    pub workflow_id: String,
}

/// Arguments for `appstack releases stages execute`.
#[derive(Debug, Args)]
pub struct StagesExecArgs {
    /// Change order ID.
    #[arg(long)]
    pub change_id: String,
    /// Stage ID.
    #[arg(long)]
    pub stage_id: String,
}

/// Arguments for `appstack releases stages cancel`.
#[derive(Debug, Args)]
pub struct StagesCancelArgs {
    /// Change order ID.
    #[arg(long)]
    pub change_id: String,
    /// Stage ID.
    #[arg(long)]
    pub stage_id: String,
}

// ─────────────────────────── Execute ────────────────────────────────────

/// Execute the `appstack` subcommand tree.
pub async fn execute(
    args: &AppstackArgs,
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
        AppstackCommands::Apps(a) => exec_apps(a, &client, &org_id, format).await,
        AppstackCommands::Tags(t) => exec_tags(t, &client, &org_id, format).await,
        AppstackCommands::Vars(v) => exec_vars(v, &client, &org_id, format).await,
        AppstackCommands::Orchestrations(o) => exec_orch(o, &client, &org_id, format).await,
        AppstackCommands::Changes(c) => exec_changes(c, &client, &org_id, format).await,
        AppstackCommands::Deploy(d) => exec_deploy(d, &client, &org_id, format).await,
        AppstackCommands::Releases(r) => exec_releases(r, &client, &org_id, format).await,
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

// ─────────────────────────── Apps ───────────────────────────────────────

/// Execute application sub-operations.
async fn exec_apps(
    args: &AppsArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        AppsCmds::List(l) => {
            let page = l.page.to_string();
            let per_page = l.per_page.to_string();
            let data = client
                .get(
                    &format!("/oapi/v1/organizations/{oid}/apps"),
                    &[("page", page.as_str()), ("perPage", per_page.as_str())],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        AppsCmds::Get(g) => {
            let data = client
                .get(
                    &format!("/oapi/v1/organizations/{oid}/apps/{}", g.app_name),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        AppsCmds::Create(c) => {
            let mut body = json!({"appName": c.app_name});
            if let Some(ref d) = c.description {
                body["description"] = json!(d);
            }
            if let Some(ref t) = c.app_type {
                body["appType"] = json!(t);
            }
            let data = client
                .post(
                    &format!("/oapi/v1/organizations/{oid}/apps"),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        AppsCmds::Update(u) => {
            let mut body = json!({});
            if let Some(ref d) = u.description {
                body["description"] = json!(d);
            }
            let data = client
                .put(
                    &format!("/oapi/v1/organizations/{oid}/apps/{}", u.app_name),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

// ─────────────────────────── Tags ───────────────────────────────────────

/// Execute tag sub-operations.
async fn exec_tags(
    args: &TagsArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        TagsCmds::Search(s) => {
            let mut params: Vec<(&str, &str)> = vec![];
            if let Some(ref kw) = s.keyword {
                params.push(("keyword", kw.as_str()));
            }
            let data = client
                .get(
                    &format!("/oapi/v1/organizations/{oid}/apps/tags"),
                    &params,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        TagsCmds::Create(c) => {
            let body = json!({"tagName": c.tag_name});
            let data = client
                .post(
                    &format!("/oapi/v1/organizations/{oid}/apps/tags"),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

// ─────────────────────────── Vars ───────────────────────────────────────

/// Execute variable-group sub-operations.
async fn exec_vars(
    args: &VarsArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        VarsCmds::List => {
            let data = client
                .get(
                    &format!("/oapi/v1/organizations/{oid}/variableGroups"),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        VarsCmds::Get(g) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/organizations/{oid}/variableGroups/{}",
                        g.var_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        VarsCmds::Create(c) => {
            let variables: serde_json::Value =
                serde_json::from_str(&c.variables).map_err(|e| {
                    crate::error::CliError::Api(format!("Invalid variables JSON: {e}"))
                })?;
            let body = json!({"name": c.name, "variables": variables});
            let data = client
                .post(
                    &format!("/oapi/v1/organizations/{oid}/variableGroups"),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        VarsCmds::Update(u) => {
            let variables: serde_json::Value =
                serde_json::from_str(&u.variables).map_err(|e| {
                    crate::error::CliError::Api(format!("Invalid variables JSON: {e}"))
                })?;
            let body = json!({"variables": variables});
            let data = client
                .put(
                    &format!(
                        "/oapi/v1/organizations/{oid}/variableGroups/{}",
                        u.var_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        VarsCmds::Delete(d) => {
            let data = client
                .delete(
                    &format!(
                        "/oapi/v1/organizations/{oid}/variableGroups/{}",
                        d.var_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

// ─────────────────────────── Orchestrations ─────────────────────────────

/// Execute orchestration sub-operations.
async fn exec_orch(
    args: &OrchArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        OrchCmds::List(l) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/organizations/{oid}/apps/{}/orchestrations",
                        l.app_name
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        OrchCmds::Get(g) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/organizations/{oid}/apps/{}/orchestrations/{}",
                        g.app_name, g.orch_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        OrchCmds::Create(c) => {
            let template: serde_json::Value =
                serde_json::from_str(&c.template).unwrap_or_else(|_| json!(c.template));
            let body = json!({"template": template});
            let data = client
                .post(
                    &format!(
                        "/oapi/v1/organizations/{oid}/apps/{}/orchestrations",
                        c.app_name
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        OrchCmds::Update(u) => {
            let mut body = json!({});
            if let Some(ref t) = u.template {
                let template: serde_json::Value =
                    serde_json::from_str(t).unwrap_or_else(|_| json!(t));
                body["template"] = template;
            }
            let data = client
                .put(
                    &format!(
                        "/oapi/v1/organizations/{oid}/apps/{}/orchestrations/{}",
                        u.app_name, u.orch_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        OrchCmds::Delete(d) => {
            let data = client
                .delete(
                    &format!(
                        "/oapi/v1/organizations/{oid}/apps/{}/orchestrations/{}",
                        d.app_name, d.orch_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

// ─────────────────────────── Changes ────────────────────────────────────

/// Execute change-order sub-operations.
async fn exec_changes(
    args: &ChangesArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        ChangesCmds::Create(c) => {
            let mut body = json!({});
            if let Some(ref a) = c.app_name {
                body["appName"] = json!(a);
            }
            if let Some(ref o) = c.orch_id {
                body["orchestrationId"] = json!(o);
            }
            if let Some(ref d) = c.description {
                body["description"] = json!(d);
            }
            let data = client
                .post(
                    &format!("/oapi/v1/organizations/{oid}/changeOrders"),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        ChangesCmds::Cancel(c) => {
            let body = json!({});
            let data = client
                .post(
                    &format!(
                        "/oapi/v1/organizations/{oid}/changeOrders/{}/cancel",
                        c.change_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        ChangesCmds::Close(c) => {
            let body = json!({});
            let data = client
                .post(
                    &format!(
                        "/oapi/v1/organizations/{oid}/changeOrders/{}/close",
                        c.change_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

// ─────────────────────────── Deploy ─────────────────────────────────────

/// Execute deployment sub-operations.
async fn exec_deploy(
    args: &DeployArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        DeployCmds::Create(c) => {
            let mut body = json!({"appName": c.app_name});
            if let Some(ref e) = c.env {
                body["envName"] = json!(e);
            }
            if let Some(ref v) = c.version {
                body["version"] = json!(v);
            }
            let data = client
                .post(
                    &format!("/oapi/v1/organizations/{oid}/deployments"),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        DeployCmds::Get(g) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/organizations/{oid}/changeOrders/{}",
                        g.change_order_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        DeployCmds::Log(l) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/organizations/{oid}/changeOrders/{}/log",
                        l.change_order_id
                    ),
                    &[],
                )
                .await?;
            // Print log content as text when available
            if let Some(log_content) = data.get("log").and_then(|l| l.as_str()) {
                println!("{log_content}");
            } else {
                output::print_output(&data, format)?;
            }
        }
        DeployCmds::Action(a) => {
            let body = json!({"action": a.action});
            let data = client
                .post(
                    &format!(
                        "/oapi/v1/organizations/{oid}/changeOrders/{}/action",
                        a.change_order_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

// ─────────────────────────── Releases ───────────────────────────────────

/// Execute release sub-operations.
async fn exec_releases(
    args: &ReleasesArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        ReleasesCmds::List(l) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/organizations/{oid}/apps/{}/releaseWorkflows",
                        l.app_name
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        ReleasesCmds::Stages(s) => match &s.command {
            StagesCmds::List(l) => {
                let data = client
                    .get(
                        &format!(
                            "/oapi/v1/organizations/{oid}/apps/{}/releaseWorkflows/{}/stages",
                            l.app_name, l.workflow_id
                        ),
                        &[],
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
            StagesCmds::Execute(e) => {
                let body = json!({});
                let data = client
                    .post(
                        &format!(
                            "/oapi/v1/organizations/{oid}/changeOrders/{}/stages/{}/execute",
                            e.change_id, e.stage_id
                        ),
                        &body,
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
            StagesCmds::Cancel(c) => {
                let body = json!({});
                let data = client
                    .post(
                        &format!(
                            "/oapi/v1/organizations/{oid}/changeOrders/{}/stages/{}/cancel",
                            c.change_id, c.stage_id
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
