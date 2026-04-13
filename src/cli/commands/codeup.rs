//! `codeup` subcommand – code management.
//!
//! Manages repositories, branches, commits, files, comparisons, and
//! change requests via the Yunxiao Codeup API.

use clap::{Args, Subcommand};
use serde_json::json;

use crate::client::ApiClient;
use crate::config;
use crate::config::types::OutputFormat;
use crate::error::Result;
use crate::output;

/// Arguments for the `codeup` subcommand.
#[derive(Debug, Args)]
pub struct CodeupArgs {
    #[command(subcommand)]
    pub command: CodeupCommands,
}

/// Top-level codeup operations.
#[derive(Debug, Subcommand)]
pub enum CodeupCommands {
    /// Manage repositories.
    Repos(ReposArgs),
    /// Manage branches.
    Branches(BranchesArgs),
    /// Browse commits.
    Commits(CommitsArgs),
    /// Manage files in a repository.
    Files(FilesArgs),
    /// Compare two refs (branches, tags, commits).
    Compare(CompareArgs),
    /// Manage merge requests.
    Mr(MrArgs),
}

// ─────────────────────────── Repos ──────────────────────────────────────

/// Arguments for `codeup repos`.
#[derive(Debug, Args)]
pub struct ReposArgs {
    #[command(subcommand)]
    pub command: ReposCmds,
}

/// Repository operations.
#[derive(Debug, Subcommand)]
pub enum ReposCmds {
    /// List repositories.
    List(ReposListArgs),
    /// Get repository details by ID.
    Get(ReposGetArgs),
}

/// Arguments for `codeup repos list`.
#[derive(Debug, Args)]
pub struct ReposListArgs {
    /// Page number (1-based).
    #[arg(long, default_value = "1")]
    pub page: u32,
    /// Number of results per page.
    #[arg(long, default_value = "20")]
    pub per_page: u32,
}

/// Arguments for `codeup repos get`.
#[derive(Debug, Args)]
pub struct ReposGetArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
}

// ─────────────────────────── Branches ───────────────────────────────────

/// Arguments for `codeup branches`.
#[derive(Debug, Args)]
pub struct BranchesArgs {
    #[command(subcommand)]
    pub command: BranchesCmds,
}

/// Branch operations.
#[derive(Debug, Subcommand)]
pub enum BranchesCmds {
    /// List branches in a repository.
    List(BranchListArgs),
    /// Get branch details.
    Get(BranchGetArgs),
    /// Create a new branch.
    Create(BranchCreateArgs),
    /// Delete a branch.
    Delete(BranchDeleteArgs),
}

/// Arguments for `codeup branches list`.
#[derive(Debug, Args)]
pub struct BranchListArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Page number.
    #[arg(long, default_value = "1")]
    pub page: u32,
    /// Results per page.
    #[arg(long, default_value = "20")]
    pub per_page: u32,
}

/// Arguments for `codeup branches get`.
#[derive(Debug, Args)]
pub struct BranchGetArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Branch name.
    #[arg(long)]
    pub branch: String,
}

/// Arguments for `codeup branches create`.
#[derive(Debug, Args)]
pub struct BranchCreateArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// New branch name.
    #[arg(long)]
    pub branch: String,
    /// Source ref (branch, tag, or commit SHA) to branch from.
    #[arg(long, name = "ref")]
    pub ref_name: String,
}

/// Arguments for `codeup branches delete`.
#[derive(Debug, Args)]
pub struct BranchDeleteArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Branch name to delete.
    #[arg(long)]
    pub branch: String,
}

// ─────────────────────────── Commits ────────────────────────────────────

/// Arguments for `codeup commits`.
#[derive(Debug, Args)]
pub struct CommitsArgs {
    #[command(subcommand)]
    pub command: CommitsCmds,
}

/// Commit operations.
#[derive(Debug, Subcommand)]
pub enum CommitsCmds {
    /// List commits on a ref.
    List(CommitListArgs),
    /// Get a single commit by SHA.
    Get(CommitGetArgs),
}

/// Arguments for `codeup commits list`.
#[derive(Debug, Args)]
pub struct CommitListArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Branch or tag to list commits for (defaults to default branch).
    #[arg(long, name = "ref")]
    pub ref_name: Option<String>,
    /// Page number.
    #[arg(long, default_value = "1")]
    pub page: u32,
    /// Results per page.
    #[arg(long, default_value = "20")]
    pub per_page: u32,
}

/// Arguments for `codeup commits get`.
#[derive(Debug, Args)]
pub struct CommitGetArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Commit SHA.
    #[arg(long)]
    pub sha: String,
}

// ─────────────────────────── Files ──────────────────────────────────────

/// Arguments for `codeup files`.
#[derive(Debug, Args)]
pub struct FilesArgs {
    #[command(subcommand)]
    pub command: FilesCmds,
}

/// File operations.
#[derive(Debug, Subcommand)]
pub enum FilesCmds {
    /// List files / directory contents.
    List(FileListArgs),
    /// Get file content.
    Get(FileGetArgs),
    /// Create a new file.
    Create(FileCreateArgs),
    /// Update an existing file.
    Update(FileUpdateArgs),
    /// Delete a file.
    Delete(FileDeleteArgs),
}

/// Arguments for `codeup files list`.
#[derive(Debug, Args)]
pub struct FileListArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Path within the repository (defaults to root).
    #[arg(long, default_value = "/")]
    pub path: String,
    /// Branch or tag ref.
    #[arg(long, name = "ref")]
    pub ref_name: Option<String>,
}

/// Arguments for `codeup files get`.
#[derive(Debug, Args)]
pub struct FileGetArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// File path within the repository.
    #[arg(long)]
    pub path: String,
    /// Branch or tag ref.
    #[arg(long, name = "ref")]
    pub ref_name: Option<String>,
}

/// Arguments for `codeup files create`.
#[derive(Debug, Args)]
pub struct FileCreateArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// File path within the repository.
    #[arg(long)]
    pub path: String,
    /// File content.
    #[arg(long)]
    pub content: String,
    /// Target branch.
    #[arg(long)]
    pub branch: String,
    /// Commit message.
    #[arg(long)]
    pub message: String,
}

/// Arguments for `codeup files update`.
#[derive(Debug, Args)]
pub struct FileUpdateArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// File path within the repository.
    #[arg(long)]
    pub path: String,
    /// New file content.
    #[arg(long)]
    pub content: String,
    /// Target branch.
    #[arg(long)]
    pub branch: String,
    /// Commit message.
    #[arg(long)]
    pub message: String,
}

/// Arguments for `codeup files delete`.
#[derive(Debug, Args)]
pub struct FileDeleteArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// File path within the repository.
    #[arg(long)]
    pub path: String,
    /// Target branch.
    #[arg(long)]
    pub branch: String,
    /// Commit message.
    #[arg(long)]
    pub message: String,
}

// ─────────────────────────── Compare ────────────────────────────────────

/// Arguments for `codeup compare`.
#[derive(Debug, Args)]
pub struct CompareArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Source ref (branch, tag, or SHA).
    #[arg(long)]
    pub from: String,
    /// Target ref (branch, tag, or SHA).
    #[arg(long)]
    pub to: String,
}

// ──────────────────────── Merge Requests ────────────────────────────────

/// Arguments for `codeup mr`.
#[derive(Debug, Args)]
pub struct MrArgs {
    #[command(subcommand)]
    pub command: MrCmds,
}

/// Merge-request operations.
#[derive(Debug, Subcommand)]
pub enum MrCmds {
    /// List merge requests.
    List(MrListArgs),
    /// Get merge request details.
    Get(MrGetArgs),
    /// Create a new merge request.
    Create(MrCreateArgs),
    /// Manage merge-request comments.
    Comments(MrCommentsArgs),
    /// List merge-request patchsets.
    Patchsets(MrPatchsetsArgs),
}

/// Arguments for `codeup mr list`.
#[derive(Debug, Args)]
pub struct MrListArgs {
    /// Repository ID (optional filter). Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: Option<String>,
    /// Filter by state (opened, closed, merged, all).
    #[arg(long)]
    pub state: Option<String>,
    /// Page number.
    #[arg(long, default_value = "1")]
    pub page: u32,
    /// Results per page.
    #[arg(long, default_value = "20")]
    pub per_page: u32,
}

/// Arguments for `codeup mr get`.
#[derive(Debug, Args)]
pub struct MrGetArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Merge request ID. Get via: yunxiao codeup mr list --repo-id <REPO_ID>
    #[arg(long)]
    pub mr_id: String,
}

/// Arguments for `codeup mr create`.
#[derive(Debug, Args)]
pub struct MrCreateArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Source branch.
    #[arg(long)]
    pub source: String,
    /// Target branch.
    #[arg(long)]
    pub target: String,
    /// Merge request title.
    #[arg(long)]
    pub title: String,
    /// Description body (optional).
    #[arg(long)]
    pub description: Option<String>,
    /// Source project ID (optional, defaults to repo_id).
    #[arg(long)]
    pub source_project_id: Option<i64>,
    /// Target project ID (optional, defaults to repo_id).
    #[arg(long)]
    pub target_project_id: Option<i64>,
}

// ── MR Comments ─────────────────────────────────────────────────────────

/// Arguments for `codeup mr comments`.
#[derive(Debug, Args)]
pub struct MrCommentsArgs {
    #[command(subcommand)]
    pub command: MrCommentsCmds,
}

/// MR comment operations.
#[derive(Debug, Subcommand)]
pub enum MrCommentsCmds {
    /// List comments on a merge request.
    List(MrCommentsListArgs),
    /// Add a comment to a merge request.
    Create(MrCommentsCreateArgs),
}

/// Arguments for `codeup mr comments list`.
#[derive(Debug, Args)]
pub struct MrCommentsListArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Merge request ID. Get via: yunxiao codeup mr list --repo-id <REPO_ID>
    #[arg(long)]
    pub mr_id: String,
}

/// Arguments for `codeup mr comments create`.
#[derive(Debug, Args)]
pub struct MrCommentsCreateArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Merge request ID. Get via: yunxiao codeup mr list --repo-id <REPO_ID>
    #[arg(long)]
    pub mr_id: String,
    /// Comment content.
    #[arg(long)]
    pub content: String,
}

// ── MR Patchsets ────────────────────────────────────────────────────────

/// Arguments for `codeup mr patchsets`.
#[derive(Debug, Args)]
pub struct MrPatchsetsArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Merge request ID. Get via: yunxiao codeup mr list --repo-id <REPO_ID>
    #[arg(long)]
    pub mr_id: String,
}

// ─────────────────────────── Execute ────────────────────────────────────

/// Execute the `codeup` subcommand tree.
pub async fn execute(
    args: &CodeupArgs,
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
        CodeupCommands::Repos(r) => exec_repos(r, &client, &org_id, format).await,
        CodeupCommands::Branches(b) => exec_branches(b, &client, &org_id, format).await,
        CodeupCommands::Commits(c) => exec_commits(c, &client, &org_id, format).await,
        CodeupCommands::Files(f) => exec_files(f, &client, &org_id, format).await,
        CodeupCommands::Compare(c) => exec_compare(c, &client, &org_id, format).await,
        CodeupCommands::Mr(m) => exec_mr(m, &client, &org_id, format).await,
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

// ─────────────────────────── Repos ──────────────────────────────────────

/// Execute repository sub-operations.
async fn exec_repos(
    args: &ReposArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        ReposCmds::List(l) => {
            let page = l.page.to_string();
            let per_page = l.per_page.to_string();
            let data = client
                .get(
                    &format!("/oapi/v1/codeup/organizations/{oid}/repositories"),
                    &[("page", page.as_str()), ("perPage", per_page.as_str())],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        ReposCmds::Get(g) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}",
                        g.repo_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

// ─────────────────────────── Branches ───────────────────────────────────

/// Execute branch sub-operations.
async fn exec_branches(
    args: &BranchesArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        BranchesCmds::List(l) => {
            let page = l.page.to_string();
            let per_page = l.per_page.to_string();
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}/branches",
                        l.repo_id
                    ),
                    &[("page", page.as_str()), ("perPage", per_page.as_str())],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        BranchesCmds::Get(g) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}/branches/detail",
                        g.repo_id
                    ),
                    &[("branchName", g.branch.as_str())],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        BranchesCmds::Create(c) => {
            let body = json!({
                "branchName": c.branch,
                "ref": c.ref_name,
            });
            let data = client
                .post(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}/branches",
                        c.repo_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        BranchesCmds::Delete(d) => {
            let data = client
                .delete(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}/branches",
                        d.repo_id
                    ),
                    &[("branchName", d.branch.as_str())],
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

// ─────────────────────────── Commits ────────────────────────────────────

/// Execute commit sub-operations.
async fn exec_commits(
    args: &CommitsArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        CommitsCmds::List(l) => {
            let page = l.page.to_string();
            let per_page = l.per_page.to_string();
            let mut params: Vec<(&str, &str)> =
                vec![("page", page.as_str()), ("perPage", per_page.as_str())];
            if let Some(ref r) = l.ref_name {
                params.push(("refName", r.as_str()));
            }
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}/commits",
                        l.repo_id
                    ),
                    &params,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        CommitsCmds::Get(g) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}/commits/{}",
                        g.repo_id, g.sha
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

// ─────────────────────────── Files ──────────────────────────────────────

/// Execute file sub-operations.
async fn exec_files(
    args: &FilesArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        FilesCmds::List(l) => {
            let mut params: Vec<(&str, &str)> = vec![("path", l.path.as_str())];
            if let Some(ref r) = l.ref_name {
                params.push(("ref", r.as_str()));
            }
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}/files/tree",
                        l.repo_id
                    ),
                    &params,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        FilesCmds::Get(g) => {
            let encoded_path = urlencoding::encode(&g.path);
            let mut params: Vec<(&str, &str)> = vec![];
            if let Some(ref r) = g.ref_name {
                params.push(("ref", r.as_str()));
            }
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}/files/{}",
                        g.repo_id, encoded_path
                    ),
                    &params,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        FilesCmds::Create(c) => {
            let body = json!({
                "filePath": c.path,
                "content": c.content,
                "branch": c.branch,
                "commitMessage": c.message,
            });
            let data = client
                .post(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}/files",
                        c.repo_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        FilesCmds::Update(u) => {
            let body = json!({
                "filePath": u.path,
                "content": u.content,
                "branch": u.branch,
                "commitMessage": u.message,
            });
            let data = client
                .put(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}/files",
                        u.repo_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        FilesCmds::Delete(d) => {
            let data = client
                .delete(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}/files",
                        d.repo_id
                    ),
                    &[
                        ("filePath", d.path.as_str()),
                        ("branch", d.branch.as_str()),
                        ("commitMessage", d.message.as_str()),
                    ],
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

// ─────────────────────────── Compare ────────────────────────────────────

/// Execute the compare operation.
async fn exec_compare(
    args: &CompareArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    let data = client
        .get(
            &format!(
                "/oapi/v1/codeup/organizations/{oid}/repositories/{}/compares",
                args.repo_id
            ),
            &[("from", args.from.as_str()), ("to", args.to.as_str())],
        )
        .await?;
    output::print_output(&data, format)?;
    Ok(())
}

// ─────────────────────── Merge Requests ─────────────────────────────────

/// Execute merge-request sub-operations.
async fn exec_mr(
    args: &MrArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        MrCmds::List(l) => {
            let page = l.page.to_string();
            let per_page = l.per_page.to_string();
            let repo_id_str;
            let mut params: Vec<(&str, &str)> =
                vec![("page", page.as_str()), ("perPage", per_page.as_str())];
            if let Some(ref s) = l.state {
                params.push(("state", s.as_str()));
            }
            if let Some(ref r) = l.repo_id {
                repo_id_str = r.clone();
                params.push(("repositoryId", repo_id_str.as_str()));
            }
            let data = client
                .get(
                    &format!("/oapi/v1/codeup/organizations/{oid}/changeRequests"),
                    &params,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        MrCmds::Get(g) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}/changeRequests/{}",
                        g.repo_id, g.mr_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        MrCmds::Create(c) => {
            let source_project_id = c
                .source_project_id
                .unwrap_or_else(|| c.repo_id.parse().unwrap_or(0));
            let target_project_id = c
                .target_project_id
                .unwrap_or_else(|| c.repo_id.parse().unwrap_or(0));
            let mut body = json!({
                "sourceBranch": c.source,
                "sourceProjectId": source_project_id,
                "targetBranch": c.target,
                "targetProjectId": target_project_id,
                "title": c.title,
            });
            if let Some(ref d) = c.description {
                body["description"] = json!(d);
            }
            let data = client
                .post(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}/changeRequests",
                        c.repo_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        MrCmds::Comments(c) => match &c.command {
            MrCommentsCmds::List(l) => {
                let data = client
                    .get(
                        &format!(
                            "/oapi/v1/codeup/organizations/{oid}/repositories/{}/changeRequests/{}/comments",
                            l.repo_id, l.mr_id
                        ),
                        &[],
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
            MrCommentsCmds::Create(cr) => {
                let body = json!({"content": cr.content});
                let data = client
                    .post(
                        &format!(
                            "/oapi/v1/codeup/organizations/{oid}/repositories/{}/changeRequests/{}/comments",
                            cr.repo_id, cr.mr_id
                        ),
                        &body,
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
        },
        MrCmds::Patchsets(p) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}/changeRequests/{}/patchsets",
                        p.repo_id, p.mr_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}
