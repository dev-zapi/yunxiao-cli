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
    /// Merge a merge request.
    Merge(MrMergeArgs),
    /// Submit a review decision or review comment.
    Review(MrReviewArgs),
    /// Close a merge request.
    Close(MrRefArgs),
    /// Reopen a closed merge request.
    Reopen(MrRefArgs),
    /// Update merge request title or description.
    Update(MrUpdateArgs),
    /// Update merge request reviewers or subscribers.
    Person(MrPersonArgs),
    /// Manage merge-request labels.
    Labels(MrLabelsArgs),
    /// Query changed file tree for a merge request.
    Tree(MrTreeArgs),
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
    /// Related work-item ID. Repeat to associate multiple work items.
    #[arg(long = "workitem-id", value_name = "WORKITEM_ID")]
    pub workitem_ids: Vec<String>,
    /// Source project ID (optional, defaults to repo_id).
    #[arg(long)]
    pub source_project_id: Option<i64>,
    /// Target project ID (optional, defaults to repo_id).
    #[arg(long)]
    pub target_project_id: Option<i64>,
}

/// Common arguments for commands that operate on one merge request.
#[derive(Debug, Args)]
pub struct MrRefArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Merge request ID. Get via: yunxiao codeup mr list --repo-id <REPO_ID>
    #[arg(long)]
    pub mr_id: String,
}

/// Arguments for `codeup mr merge`.
#[derive(Debug, Args)]
pub struct MrMergeArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Merge request ID. Get via: yunxiao codeup mr list --repo-id <REPO_ID>
    #[arg(long)]
    pub mr_id: String,
    /// Merge strategy: ff-only, no-fast-forward, squash, or rebase.
    #[arg(
        long,
        value_parser = ["ff-only", "no-fast-forward", "squash", "rebase"]
    )]
    pub merge_type: String,
    /// Merge commit message.
    #[arg(long)]
    pub merge_message: Option<String>,
    /// Delete source branch after merge.
    #[arg(long, default_value_t = false)]
    pub remove_source_branch: bool,
}

/// Arguments for `codeup mr review`.
#[derive(Debug, Args)]
pub struct MrReviewArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Merge request ID. Get via: yunxiao codeup mr list --repo-id <REPO_ID>
    #[arg(long)]
    pub mr_id: String,
    /// Review opinion: PASS or NOT_PASS.
    #[arg(long, value_parser = ["PASS", "NOT_PASS"])]
    pub opinion: Option<String>,
    /// Review comment content.
    #[arg(long)]
    pub comment: Option<String>,
    /// Draft comment ID to submit. Repeat or use comma-separated values.
    #[arg(
        long = "draft-comment-id",
        value_name = "DRAFT_COMMENT_ID",
        value_delimiter = ','
    )]
    pub submit_draft_comment_ids: Vec<String>,
}

/// Arguments for `codeup mr update`.
#[derive(Debug, Args)]
pub struct MrUpdateArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Merge request ID. Get via: yunxiao codeup mr list --repo-id <REPO_ID>
    #[arg(long)]
    pub mr_id: String,
    /// New merge request title.
    #[arg(long)]
    pub title: Option<String>,
    /// New merge request description.
    #[arg(long)]
    pub description: Option<String>,
}

/// Arguments for `codeup mr person`.
#[derive(Debug, Args)]
pub struct MrPersonArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Merge request ID. Get via: yunxiao codeup mr list --repo-id <REPO_ID>
    #[arg(long)]
    pub mr_id: String,
    /// Related person type: REVIEWER or SUBSCRIBER.
    #[arg(long = "type", value_parser = ["REVIEWER", "SUBSCRIBER"])]
    pub person_type: String,
    /// User ID. Repeat or use comma-separated values.
    #[arg(
        long = "user-id",
        visible_alias = "user-ids",
        value_name = "USER_ID",
        value_delimiter = ','
    )]
    pub user_ids: Vec<String>,
}

/// Arguments for `codeup mr labels`.
#[derive(Debug, Args)]
pub struct MrLabelsArgs {
    #[command(subcommand)]
    pub command: MrLabelsCmds,
}

/// MR label operations.
#[derive(Debug, Subcommand)]
pub enum MrLabelsCmds {
    /// List labels attached to a merge request.
    List(MrLabelsListArgs),
    /// Attach labels to a merge request.
    Attach(MrLabelsAttachArgs),
}

/// Arguments for `codeup mr labels list`.
#[derive(Debug, Args)]
pub struct MrLabelsListArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Merge request ID. Get via: yunxiao codeup mr list --repo-id <REPO_ID>
    #[arg(long)]
    pub mr_id: String,
}

/// Arguments for `codeup mr labels attach`.
#[derive(Debug, Args)]
pub struct MrLabelsAttachArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Merge request ID. Get via: yunxiao codeup mr list --repo-id <REPO_ID>
    #[arg(long)]
    pub mr_id: String,
    /// Label ID. Repeat or use comma-separated values.
    #[arg(
        long = "label-id",
        visible_alias = "label-ids",
        value_name = "LABEL_ID",
        value_delimiter = ','
    )]
    pub label_ids: Vec<String>,
}

/// Arguments for `codeup mr tree`.
#[derive(Debug, Args)]
pub struct MrTreeArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Merge request ID. Get via: yunxiao codeup mr list --repo-id <REPO_ID>
    #[arg(long)]
    pub mr_id: String,
    /// Target/base patchset ID.
    #[arg(long = "from-patchset")]
    pub from_patchset_id: String,
    /// Source/head patchset ID.
    #[arg(long = "to-patchset")]
    pub to_patchset_id: String,
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
    /// Update a comment on a merge request.
    Update(MrCommentsUpdateArgs),
    /// Delete a comment from a merge request.
    Delete(MrCommentsDeleteArgs),
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

/// Arguments for `codeup mr comments update`.
#[derive(Debug, Args)]
pub struct MrCommentsUpdateArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Merge request ID. Get via: yunxiao codeup mr list --repo-id <REPO_ID>
    #[arg(long)]
    pub mr_id: String,
    /// Comment biz ID.
    #[arg(long = "comment-id")]
    pub comment_id: String,
    /// New comment content.
    #[arg(long)]
    pub content: Option<String>,
    /// Mark the comment thread resolved or unresolved.
    #[arg(long)]
    pub resolved: Option<bool>,
}

/// Arguments for `codeup mr comments delete`.
#[derive(Debug, Args)]
pub struct MrCommentsDeleteArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Merge request ID. Get via: yunxiao codeup mr list --repo-id <REPO_ID>
    #[arg(long)]
    pub mr_id: String,
    /// Comment biz ID.
    #[arg(long = "comment-id")]
    pub comment_id: String,
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

fn build_mr_create_body(c: &MrCreateArgs) -> Result<serde_json::Value> {
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
    if !c.workitem_ids.is_empty() {
        if c.workitem_ids.iter().any(|id| id.contains(',')) {
            return Err(crate::error::CliError::Config(
                "Each --workitem-id must be a single work item ID and cannot contain commas. Repeat --workitem-id to associate multiple work items.".into(),
            ));
        }
        body["workItemIds"] = json!(c.workitem_ids.join(","));
    }
    Ok(body)
}

fn build_mr_merge_body(m: &MrMergeArgs) -> Result<serde_json::Value> {
    let mut body = json!({
        "mergeType": m.merge_type,
    });
    if let Some(ref message) = m.merge_message {
        require_non_empty_text(message, "--merge-message")?;
        body["mergeMessage"] = json!(message);
    }
    if m.remove_source_branch {
        body["removeSourceBranch"] = json!(true);
    }
    Ok(body)
}

fn build_mr_review_body(r: &MrReviewArgs) -> Result<serde_json::Value> {
    let mut body = json!({});
    if let Some(ref opinion) = r.opinion {
        body["reviewOpinion"] = json!(opinion);
    }
    if let Some(ref comment) = r.comment {
        require_non_empty_text(comment, "--comment")?;
        body["reviewComment"] = json!(comment);
    }
    if !r.submit_draft_comment_ids.is_empty() {
        body["submitDraftCommentIds"] = json!(normalized_non_empty_values(
            &r.submit_draft_comment_ids,
            "--draft-comment-id"
        )?);
    }
    require_non_empty_body(
        &body,
        "mr review requires --opinion, --comment, or --draft-comment-id",
    )?;
    Ok(body)
}

fn build_mr_update_body(u: &MrUpdateArgs) -> Result<serde_json::Value> {
    let mut body = json!({});
    if let Some(ref title) = u.title {
        require_non_empty_text(title, "--title")?;
        body["title"] = json!(title);
    }
    if let Some(ref description) = u.description {
        body["description"] = json!(description);
    }
    require_non_empty_body(
        &body,
        "mr update requires at least one of --title or --description",
    )?;
    Ok(body)
}

fn build_mr_person_body(p: &MrPersonArgs) -> Result<serde_json::Value> {
    Ok(json!({
        "userIds": normalized_non_empty_values(&p.user_ids, "--user-id")?,
    }))
}

fn build_mr_labels_attach_body(l: &MrLabelsAttachArgs) -> Result<serde_json::Value> {
    Ok(json!({
        "label_id_list": normalized_non_empty_values(&l.label_ids, "--label-id")?,
    }))
}

fn build_mr_comment_update_body(c: &MrCommentsUpdateArgs) -> Result<serde_json::Value> {
    let mut body = json!({});
    if let Some(ref content) = c.content {
        require_non_empty_text(content, "--content")?;
        body["content"] = json!(content);
    }
    if let Some(resolved) = c.resolved {
        body["resolved"] = json!(resolved);
    }
    require_non_empty_body(
        &body,
        "comments update requires at least one of --content or --resolved",
    )?;
    Ok(body)
}

fn normalized_non_empty_values(values: &[String], flag: &str) -> Result<Vec<String>> {
    if values.is_empty() {
        return Err(crate::error::CliError::Config(format!(
            "{flag} must be provided at least once"
        )));
    }
    values
        .iter()
        .map(|value| {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                Err(crate::error::CliError::Config(format!(
                    "{flag} values cannot be empty"
                )))
            } else {
                Ok(trimmed.to_string())
            }
        })
        .collect()
}

fn require_non_empty_text(value: &str, flag: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(crate::error::CliError::Config(format!(
            "{flag} cannot be empty"
        )));
    }
    Ok(())
}

fn require_non_empty_body(body: &serde_json::Value, message: &str) -> Result<()> {
    let is_empty = match body.as_object() {
        Some(obj) => obj.is_empty(),
        None => false,
    };
    if is_empty {
        return Err(crate::error::CliError::Config(message.into()));
    }
    Ok(())
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
            let body = build_mr_create_body(c)?;
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
        MrCmds::Merge(m) => {
            let body = build_mr_merge_body(m)?;
            let data = client
                .post(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}/changeRequests/{}/merge",
                        m.repo_id, m.mr_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        MrCmds::Review(r) => {
            let body = build_mr_review_body(r)?;
            let data = client
                .post(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}/changeRequests/{}/review",
                        r.repo_id, r.mr_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        MrCmds::Close(c) => {
            let data = client
                .post(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}/changeRequests/{}/close",
                        c.repo_id, c.mr_id
                    ),
                    &json!({}),
                )
                .await?;
            output::print_output(&data, format)?;
        }
        MrCmds::Reopen(r) => {
            let data = client
                .post(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}/changeRequests/{}/reopen",
                        r.repo_id, r.mr_id
                    ),
                    &json!({}),
                )
                .await?;
            output::print_output(&data, format)?;
        }
        MrCmds::Update(u) => {
            let body = build_mr_update_body(u)?;
            let data = client
                .put(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}/changeRequests/{}",
                        u.repo_id, u.mr_id
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        MrCmds::Person(p) => {
            let body = build_mr_person_body(p)?;
            let data = client
                .post(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}/changeRequests/{}/person/{}",
                        p.repo_id, p.mr_id, p.person_type
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        MrCmds::Labels(l) => match &l.command {
            MrLabelsCmds::List(list) => {
                let data = client
                    .get(
                        &format!(
                            "/oapi/v1/codeup/organizations/{oid}/repositories/{}/changeRequests/{}/labels",
                            list.repo_id, list.mr_id
                        ),
                        &[],
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
            MrLabelsCmds::Attach(attach) => {
                let body = build_mr_labels_attach_body(attach)?;
                let data = client
                    .post(
                        &format!(
                            "/oapi/v1/codeup/organizations/{oid}/repositories/{}/changeRequests/{}/labels",
                            attach.repo_id, attach.mr_id
                        ),
                        &body,
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
        },
        MrCmds::Tree(t) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}/changeRequests/{}/diffs/changeTree",
                        t.repo_id, t.mr_id
                    ),
                    &[
                        ("fromPatchSetId", t.from_patchset_id.as_str()),
                        ("toPatchSetId", t.to_patchset_id.as_str()),
                    ],
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
            MrCommentsCmds::Update(u) => {
                let body = build_mr_comment_update_body(u)?;
                let data = client
                    .put(
                        &format!(
                            "/oapi/v1/codeup/organizations/{oid}/repositories/{}/changeRequests/{}/comments/{}",
                            u.repo_id, u.mr_id, u.comment_id
                        ),
                        &body,
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
            MrCommentsCmds::Delete(d) => {
                let data = client
                    .delete(
                        &format!(
                            "/oapi/v1/codeup/organizations/{oid}/repositories/{}/changeRequests/{}/comments/{}",
                            d.repo_id, d.mr_id, d.comment_id
                        ),
                        &[],
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::Cli;
    use clap::Parser;

    #[test]
    fn build_mr_create_body_includes_workitem_ids() {
        let args = MrCreateArgs {
            repo_id: "2813489".into(),
            source: "feature/login".into(),
            target: "master".into(),
            title: "Add login".into(),
            description: Some("desc".into()),
            workitem_ids: vec!["wi-1".into(), "wi-2".into()],
            source_project_id: None,
            target_project_id: None,
        };

        let body = build_mr_create_body(&args).unwrap();

        assert_eq!(body["sourceProjectId"], 2813489);
        assert_eq!(body["targetProjectId"], 2813489);
        assert_eq!(body["workItemIds"], "wi-1,wi-2");
        assert_eq!(body["description"], "desc");
    }

    #[test]
    fn build_mr_create_body_omits_empty_workitem_ids() {
        let args = MrCreateArgs {
            repo_id: "2813489".into(),
            source: "feature/login".into(),
            target: "master".into(),
            title: "Add login".into(),
            description: None,
            workitem_ids: Vec::new(),
            source_project_id: Some(1),
            target_project_id: Some(2),
        };

        let body = build_mr_create_body(&args).unwrap();

        assert_eq!(body["sourceProjectId"], 1);
        assert_eq!(body["targetProjectId"], 2);
        assert!(body.get("workItemIds").is_none());
        assert!(body.get("description").is_none());
    }

    #[test]
    fn build_mr_create_body_rejects_comma_in_single_workitem_id() {
        let args = MrCreateArgs {
            repo_id: "2813489".into(),
            source: "feature/login".into(),
            target: "master".into(),
            title: "Add login".into(),
            description: None,
            workitem_ids: vec!["wi-1,wi-2".into()],
            source_project_id: None,
            target_project_id: None,
        };

        let err = build_mr_create_body(&args).unwrap_err();

        assert!(err.to_string().contains("cannot contain commas"));
    }

    #[test]
    fn cli_parses_repeated_workitem_id_flags() {
        let cli = Cli::parse_from([
            "yunxiao",
            "codeup",
            "mr",
            "create",
            "--repo-id",
            "2813489",
            "--source",
            "feature/login",
            "--target",
            "master",
            "--title",
            "Add login",
            "--workitem-id",
            "wi-1",
            "--workitem-id",
            "wi-2",
        ]);

        let crate::cli::Commands::Codeup(codeup) = cli.command else {
            panic!("expected codeup command");
        };
        let CodeupCommands::Mr(mr) = codeup.command else {
            panic!("expected mr command");
        };
        let MrCmds::Create(args) = mr.command else {
            panic!("expected mr create command");
        };

        assert_eq!(args.workitem_ids, vec!["wi-1", "wi-2"]);
    }

    #[test]
    fn build_mr_merge_body_matches_api() {
        let args = MrMergeArgs {
            repo_id: "2813489".into(),
            mr_id: "12".into(),
            merge_type: "squash".into(),
            merge_message: Some("Merge feature".into()),
            remove_source_branch: true,
        };

        let body = build_mr_merge_body(&args).unwrap();

        assert_eq!(body["mergeType"], "squash");
        assert_eq!(body["mergeMessage"], "Merge feature");
        assert_eq!(body["removeSourceBranch"], true);
    }

    #[test]
    fn build_mr_review_body_includes_optional_fields() {
        let args = MrReviewArgs {
            repo_id: "2813489".into(),
            mr_id: "12".into(),
            opinion: Some("PASS".into()),
            comment: Some("looks good".into()),
            submit_draft_comment_ids: vec!["draft-1".into(), "draft-2".into()],
        };

        let body = build_mr_review_body(&args).unwrap();

        assert_eq!(body["reviewOpinion"], "PASS");
        assert_eq!(body["reviewComment"], "looks good");
        assert_eq!(body["submitDraftCommentIds"], json!(["draft-1", "draft-2"]));
    }

    #[test]
    fn build_mr_update_body_requires_a_change() {
        let args = MrUpdateArgs {
            repo_id: "2813489".into(),
            mr_id: "12".into(),
            title: None,
            description: None,
        };

        let err = build_mr_update_body(&args).unwrap_err();

        assert!(err.to_string().contains("at least one"));
    }

    #[test]
    fn build_mr_update_body_allows_empty_description_to_clear_it() {
        let args = MrUpdateArgs {
            repo_id: "2813489".into(),
            mr_id: "12".into(),
            title: None,
            description: Some(String::new()),
        };

        let body = build_mr_update_body(&args).unwrap();

        assert_eq!(body["description"], "");
    }

    #[test]
    fn build_mr_person_body_uses_user_ids_array() {
        let args = MrPersonArgs {
            repo_id: "2813489".into(),
            mr_id: "12".into(),
            person_type: "REVIEWER".into(),
            user_ids: vec!["u1".into(), "u2".into()],
        };

        let body = build_mr_person_body(&args).unwrap();

        assert_eq!(body["userIds"], json!(["u1", "u2"]));
    }

    #[test]
    fn build_mr_labels_attach_body_uses_documented_field_name() {
        let args = MrLabelsAttachArgs {
            repo_id: "2813489".into(),
            mr_id: "12".into(),
            label_ids: vec!["bug".into(), "urgent".into()],
        };

        let body = build_mr_labels_attach_body(&args).unwrap();

        assert_eq!(body["label_id_list"], json!(["bug", "urgent"]));
    }

    #[test]
    fn build_mr_comment_update_body_allows_resolve_only() {
        let args = MrCommentsUpdateArgs {
            repo_id: "2813489".into(),
            mr_id: "12".into(),
            comment_id: "comment-1".into(),
            content: None,
            resolved: Some(true),
        };

        let body = build_mr_comment_update_body(&args).unwrap();

        assert_eq!(body["resolved"], true);
        assert!(body.get("content").is_none());
    }

    #[test]
    fn cli_parses_new_mr_commands() {
        let merge = parse_mr_command([
            "yunxiao",
            "codeup",
            "mr",
            "merge",
            "--repo-id",
            "2813489",
            "--mr-id",
            "12",
            "--merge-type",
            "ff-only",
        ]);
        assert!(matches!(merge, MrCmds::Merge(_)));

        let review = parse_mr_command([
            "yunxiao",
            "codeup",
            "mr",
            "review",
            "--repo-id",
            "2813489",
            "--mr-id",
            "12",
            "--opinion",
            "NOT_PASS",
            "--draft-comment-id",
            "draft-1",
            "--draft-comment-id",
            "draft-2",
        ]);
        assert!(matches!(review, MrCmds::Review(_)));

        let close = parse_mr_command([
            "yunxiao",
            "codeup",
            "mr",
            "close",
            "--repo-id",
            "2813489",
            "--mr-id",
            "12",
        ]);
        assert!(matches!(close, MrCmds::Close(_)));

        let reopen = parse_mr_command([
            "yunxiao",
            "codeup",
            "mr",
            "reopen",
            "--repo-id",
            "2813489",
            "--mr-id",
            "12",
        ]);
        assert!(matches!(reopen, MrCmds::Reopen(_)));

        let update = parse_mr_command([
            "yunxiao",
            "codeup",
            "mr",
            "update",
            "--repo-id",
            "2813489",
            "--mr-id",
            "12",
            "--title",
            "New title",
        ]);
        assert!(matches!(update, MrCmds::Update(_)));

        let person = parse_mr_command([
            "yunxiao",
            "codeup",
            "mr",
            "person",
            "--repo-id",
            "2813489",
            "--mr-id",
            "12",
            "--type",
            "SUBSCRIBER",
            "--user-id",
            "u1",
            "--user-id",
            "u2",
        ]);
        assert!(matches!(person, MrCmds::Person(_)));

        let labels_attach = parse_mr_command([
            "yunxiao",
            "codeup",
            "mr",
            "labels",
            "attach",
            "--repo-id",
            "2813489",
            "--mr-id",
            "12",
            "--label-ids",
            "bug,urgent",
        ]);
        assert!(matches!(labels_attach, MrCmds::Labels(_)));

        let labels_list = parse_mr_command([
            "yunxiao",
            "codeup",
            "mr",
            "labels",
            "list",
            "--repo-id",
            "2813489",
            "--mr-id",
            "12",
        ]);
        assert!(matches!(labels_list, MrCmds::Labels(_)));

        let tree = parse_mr_command([
            "yunxiao",
            "codeup",
            "mr",
            "tree",
            "--repo-id",
            "2813489",
            "--mr-id",
            "12",
            "--from-patchset",
            "base",
            "--to-patchset",
            "head",
        ]);
        assert!(matches!(tree, MrCmds::Tree(_)));
    }

    #[test]
    fn cli_parses_comment_update_and_delete() {
        let update = parse_mr_command([
            "yunxiao",
            "codeup",
            "mr",
            "comments",
            "update",
            "--repo-id",
            "2813489",
            "--mr-id",
            "12",
            "--comment-id",
            "comment-1",
            "--resolved",
            "true",
        ]);

        let MrCmds::Comments(comments) = update else {
            panic!("expected comments command");
        };
        assert!(matches!(comments.command, MrCommentsCmds::Update(_)));

        let delete = parse_mr_command([
            "yunxiao",
            "codeup",
            "mr",
            "comments",
            "delete",
            "--repo-id",
            "2813489",
            "--mr-id",
            "12",
            "--comment-id",
            "comment-1",
        ]);

        let MrCmds::Comments(comments) = delete else {
            panic!("expected comments command");
        };
        assert!(matches!(comments.command, MrCommentsCmds::Delete(_)));
    }

    fn parse_mr_command<const N: usize>(args: [&str; N]) -> MrCmds {
        let cli = Cli::parse_from(args);
        let crate::cli::Commands::Codeup(codeup) = cli.command else {
            panic!("expected codeup command");
        };
        let CodeupCommands::Mr(mr) = codeup.command else {
            panic!("expected mr command");
        };
        mr.command
    }
}
