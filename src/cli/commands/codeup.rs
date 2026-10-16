//! `codeup` subcommand – code management.
//!
//! Manages repositories, branches, commits, files, comparisons, and
//! change requests via the Yunxiao Codeup API.

use clap::{Args, Subcommand};
use serde_json::json;
use std::io::{IsTerminal, Write};

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
    /// Manage code groups.
    Groups(GroupsArgs),
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
    /// Create a repository, optionally importing from a template.
    Create(Box<ReposCreateArgs>),
    /// Query template repositories.
    Templates(ReposTemplatesArgs),
    /// Delete a repository.
    ///
    /// By default prompts twice: first a y/N confirmation, then requires typing
    /// the exact repository ID or full path (path_with_namespace). Pass --yes
    /// to skip both prompts for automation.
    Delete(ReposDeleteArgs),
    /// Update repository metadata (partial update).
    Update(ReposUpdateArgs),
    /// Archive a repository.
    Archive(ReposArchiveArgs),
    /// Transfer a repository to another group.
    Transfer(ReposTransferArgs),
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
    /// Filter by keyword, fuzzy-matching the repository path.
    #[arg(long)]
    pub search: Option<String>,
    /// Sort field (created_at, name, path, last_activity_at).
    #[arg(long)]
    pub order_by: Option<String>,
    /// Sort direction (asc, desc).
    #[arg(long)]
    pub sort: Option<String>,
    /// Filter by archived state (true/false). Omit to include both.
    #[arg(long)]
    pub archived: Option<bool>,
}

/// Arguments for `codeup repos get`.
#[derive(Debug, Args)]
pub struct ReposGetArgs {
    /// Repository ID. Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
}

/// Arguments for `codeup repos create`.
#[derive(Debug, Args)]
pub struct ReposCreateArgs {
    /// Repository name (required).
    #[arg(long)]
    pub name: String,
    /// Repository path (required).
    #[arg(long)]
    pub path: String,
    /// Parent code group space ID. Accepts a numeric ID or full path; omit to create at the organization root.
    #[arg(long = "namespace-id")]
    pub namespace_id: Option<String>,
    /// Repository description.
    #[arg(long)]
    pub description: Option<String>,
    /// Repository visibility. One of: private, internal.
    #[arg(long)]
    pub visibility: Option<String>,
    /// Type of README to auto-create. One of: EMPTY, USER_GUIDE.
    #[arg(long = "readme-type")]
    pub readme_type: Option<String>,
    /// Comma-separated repository tags, e.g. "tag1,tag2".
    #[arg(long)]
    pub tags: Option<String>,
    /// Repository avatar URL.
    #[arg(long = "avatar-url")]
    pub avatar_url: Option<String>,
    /// Template repository ID to import from. Accepts a numeric ID or full path.
    #[arg(long = "template-id")]
    pub template_id: Option<String>,
    /// Template type: 1 (custom) or 2 (system preset).
    #[arg(long = "template-type")]
    pub template_type: Option<i64>,
    /// Import all branches from the template repository.
    #[arg(long = "sync-all-branches")]
    pub sync_all_branches: bool,
    /// Automatically create missing parent paths.
    #[arg(long = "create-parent-path")]
    pub create_parent_path: bool,
}

/// Arguments for `codeup repos templates`.
#[derive(Debug, Args)]
pub struct ReposTemplatesArgs {
    #[command(subcommand)]
    pub command: ReposTemplatesCmds,
}

/// Template-repository operations.
#[derive(Debug, Subcommand)]
pub enum ReposTemplatesCmds {
    /// List template repositories.
    List(ReposTemplatesListArgs),
}

/// Arguments for `codeup repos templates list`.
#[derive(Debug, Args)]
pub struct ReposTemplatesListArgs {
    /// Template type (1 = custom, 2 = system preset). Required.
    #[arg(long)]
    pub template_type: u32,
    /// Page number (1-based).
    #[arg(long, default_value = "1")]
    pub page: u32,
    /// Number of results per page.
    #[arg(long, default_value = "20")]
    pub per_page: u32,
}

// ─────────────────────────── Groups ─────────────────────────────────────

/// Arguments for `codeup groups`.
#[derive(Debug, Args)]
pub struct GroupsArgs {
    #[command(subcommand)]
    pub command: GroupsCmds,
}

/// Code-group operations.
#[derive(Debug, Subcommand)]
pub enum GroupsCmds {
    /// Create a new code group.
    Create(GroupsCreateArgs),
    /// Update a code group's metadata (partial update).
    Update(GroupsUpdateArgs),
    /// List code groups (and root spaces).
    List(GroupsListArgs),
    /// Get a code group or root space by ID or full path.
    Get(GroupsGetArgs),
    /// Manage repositories within a code group.
    Repos(GroupsReposArgs),
}

/// Arguments for `codeup groups create`.
#[derive(Debug, Args)]
pub struct GroupsCreateArgs {
    /// Code group display name (required).
    #[arg(long)]
    pub name: String,
    /// Code group path segment (required).
    #[arg(long)]
    pub path: String,
    /// Parent group or root-space ID. Omit to create under the org root.
    #[arg(long)]
    pub parent_id: Option<i64>,
    /// Description text (up to 65535 characters).
    #[arg(long)]
    pub description: Option<String>,
    /// Visibility: `private` (group members only) or `internal` (org-wide).
    #[arg(long)]
    pub visibility: Option<String>,
    /// Avatar URL.
    #[arg(long)]
    pub avatar: Option<String>,
}

/// Arguments for `codeup groups update`.
#[derive(Debug, Args)]
pub struct GroupsUpdateArgs {
    /// Group ID, or full path (automatically percent-encoded if it contains '/').
    #[arg(long)]
    pub group_id: String,
    /// New display name.
    #[arg(long)]
    pub name: Option<String>,
    /// New path segment.
    #[arg(long)]
    pub path: Option<String>,
    /// New description.
    #[arg(long)]
    pub description: Option<String>,
    /// New visibility: `private` or `internal`.
    #[arg(long)]
    pub visibility: Option<String>,
    /// New avatar URL.
    #[arg(long)]
    pub avatar: Option<String>,
}

/// Arguments for `codeup groups list`.
#[derive(Debug, Args)]
pub struct GroupsListArgs {
    /// Parent group or root-space ID, or full path. Omit to list the current
    /// user's group spaces.
    #[arg(long)]
    pub parent_id: Option<String>,
    /// Search keyword.
    #[arg(long)]
    pub search: Option<String>,
    /// Sort field: created_at or updated_at (default updated_at).
    #[arg(long)]
    pub order_by: Option<String>,
    /// Sort direction: asc or desc (default desc).
    #[arg(long)]
    pub sort: Option<String>,
    /// Page number (1-based).
    #[arg(long, default_value = "1")]
    pub page: u32,
    /// Number of results per page.
    #[arg(long, default_value = "20")]
    pub per_page: u32,
}

/// Arguments for `codeup groups get`.
#[derive(Debug, Args)]
pub struct GroupsGetArgs {
    /// Group or root-space ID, or full path.
    #[arg(long)]
    pub group_id: String,
}

/// Arguments for `codeup groups repos`.
#[derive(Debug, Args)]
pub struct GroupsReposArgs {
    #[command(subcommand)]
    pub command: GroupsReposCmds,
}

/// Operations on repositories within a code group.
#[derive(Debug, Subcommand)]
pub enum GroupsReposCmds {
    /// List repositories in a code group.
    List(GroupsReposListArgs),
}

/// Arguments for `codeup groups repos list`.
#[derive(Debug, Args)]
pub struct GroupsReposListArgs {
    /// Group or root-space ID, or full path.
    #[arg(long)]
    pub group_id: String,
    /// Search keyword (repository name).
    #[arg(long)]
    pub search: Option<String>,
    /// Include repositories in subgroups.
    #[arg(long)]
    pub include_subgroups: bool,
    /// Page number (1-based).
    #[arg(long, default_value = "1")]
    pub page: u32,
    /// Number of results per page.
    #[arg(long, default_value = "20")]
    pub per_page: u32,
}

/// Arguments for `codeup repos delete`.
#[derive(Debug, Args)]
pub struct ReposDeleteArgs {
    /// Repository ID or URL-encoded full path (path_with_namespace).
    /// Get via: yunxiao codeup repos list
    #[arg(long)]
    pub repo_id: String,
    /// Skip the interactive confirmation prompts (for automation).
    #[arg(long)]
    pub yes: bool,
}

/// Arguments for `codeup repos update`.
#[derive(Debug, Args)]
pub struct ReposUpdateArgs {
    /// Repository ID or URL-encoded full path (e.g. 60de...%2FDemoRepo).
    #[arg(long)]
    pub repo_id: String,
    /// New repository name.
    #[arg(long)]
    pub name: Option<String>,
    /// New repository description (max 65535 characters).
    #[arg(long)]
    pub description: Option<String>,
    /// Default branch.
    #[arg(long)]
    pub default_branch: Option<String>,
    /// Visibility: private or internal.
    #[arg(long, value_parser = ["private", "internal"])]
    pub visibility: Option<String>,
    /// Comma-separated repository tags.
    #[arg(long, value_delimiter = ',', value_name = "TAG")]
    pub tags: Vec<String>,
}

/// Arguments for `codeup repos archive`.
#[derive(Debug, Args)]
pub struct ReposArchiveArgs {
    /// Repository ID or URL-encoded full path (e.g. 60de...%2FDemoRepo).
    #[arg(long)]
    pub repo_id: String,
}

/// Arguments for `codeup repos transfer`.
#[derive(Debug, Args)]
pub struct ReposTransferArgs {
    /// Repository ID or path_with_namespace full path.
    #[arg(long)]
    pub repo_id: String,
    /// Target group ID or path_with_namespace full path.
    #[arg(long)]
    pub group_id: String,
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

/// Render an identifier that may be a numeric ID or a full path.
///
/// Numeric strings are emitted as JSON numbers so the API's typed field is
/// preserved; anything else is emitted as a JSON string (the URL-encoded
/// full path the Codeup API also accepts for namespace/template fields).
fn id_or_path_value(value: &str) -> serde_json::Value {
    match value.trim().parse::<i64>() {
        Ok(id) => json!(id),
        Err(_) => json!(value),
    }
}

/// Build the request body for `codeup repos create`.
///
/// Required fields (`name`, `path`) are always present. All other flags are
/// omitted from the body when unset. Template-related flags are nested into
/// a single `templateProject` object and only included when `template_id` is
/// given. Visibility is validated against the API's allowed values.
fn build_repo_create_body(c: &ReposCreateArgs) -> Result<serde_json::Value> {
    let mut body = json!({
        "name": c.name,
        "path": c.path,
    });
    if let Some(ref namespace_id) = c.namespace_id {
        body["namespaceId"] = id_or_path_value(namespace_id);
    }
    if let Some(ref description) = c.description {
        body["description"] = json!(description);
    }
    if let Some(ref visibility) = c.visibility {
        match visibility.as_str() {
            "private" | "internal" => body["visibility"] = json!(visibility),
            other => {
                return Err(crate::error::CliError::Config(format!(
                    "--visibility must be one of: private, internal (got \"{other}\")"
                )));
            }
        }
    }
    if let Some(ref readme_type) = c.readme_type {
        body["readMeType"] = json!(readme_type);
    }
    if let Some(ref tags) = c.tags {
        let values: Vec<String> = tags
            .split(',')
            .map(|tag| tag.trim().to_string())
            .filter(|tag| !tag.is_empty())
            .collect();
        if !values.is_empty() {
            body["repositoryTags"] = json!(values);
        }
    }
    if let Some(ref avatar_url) = c.avatar_url {
        body["avatarUrl"] = json!(avatar_url);
    }
    if let Some(ref template_id) = c.template_id {
        let mut template = json!({
            "templateProjectId": id_or_path_value(template_id),
        });
        if let Some(template_type) = c.template_type {
            template["templateType"] = json!(template_type);
        }
        if c.sync_all_branches {
            template["syncAllBranches"] = json!(true);
        }
        body["templateProject"] = template;
    }
    Ok(body)
}

/// Build the request path for `codeup repos create`, appending the
/// `createParentPath` query parameter when the flag is set.
fn repos_create_path(org_id: &str, create_parent_path: bool) -> String {
    let base = format!("/oapi/v1/codeup/organizations/{org_id}/repositories");
    if create_parent_path {
        format!("{base}?createParentPath=true")
    } else {
        base
    }
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

/// Percent-encode a repository ID when it's a full path containing '/'.
/// Plain numeric IDs pass through unchanged; paths like
/// `60de7a6852743a5162b5f957/DemoRepo` become URL-encoded as the API expects.
fn encode_repo_path(value: &str) -> String {
    if value.contains('/') {
        urlencoding::encode(value).into_owned()
    } else {
        value.to_string()
    }
}

fn build_repo_update_body(u: &ReposUpdateArgs) -> Result<serde_json::Value> {
    let mut body = json!({});
    if let Some(ref name) = u.name {
        body["name"] = json!(name);
    }
    if let Some(ref description) = u.description {
        body["description"] = json!(description);
    }
    if let Some(ref branch) = u.default_branch {
        body["default_branch"] = json!(branch);
    }
    if let Some(ref visibility) = u.visibility {
        body["visibility"] = json!(visibility);
    }
    if !u.tags.is_empty() {
        body["repositoryTags"] = json!(normalized_non_empty_values(&u.tags, "--tags")?);
    }
    Ok(body)
}

fn build_repo_transfer_body(t: &ReposTransferArgs) -> Result<serde_json::Value> {
    Ok(json!({
        "repositoryId": t.repo_id,
        "groupId": t.group_id,
    }))
}

/// Validate a group visibility value (`private` or `internal`).
fn require_valid_group_visibility(value: &str) -> Result<()> {
    if value != "private" && value != "internal" {
        return Err(crate::error::CliError::Config(format!(
            "--visibility must be 'private' or 'internal' (got '{value}')"
        )));
    }
    Ok(())
}

/// Build the request body for `codeup groups create`.
///
/// `name` and `path` are always sent. Optional fields (`parentId`, `description`,
/// `visibility`, `avatar`) are omitted from the body when unset so the API applies
/// its own defaults. `visibility`, when provided, must be `private` or `internal`.
fn build_group_create_body(c: &GroupsCreateArgs) -> Result<serde_json::Value> {
    require_non_empty_text(&c.name, "--name")?;
    require_non_empty_text(&c.path, "--path")?;
    let mut body = json!({
        "name": c.name,
        "path": c.path,
    });
    if let Some(parent_id) = c.parent_id {
        body["parentId"] = json!(parent_id);
    }
    if let Some(ref description) = c.description {
        body["description"] = json!(description);
    }
    if let Some(ref visibility) = c.visibility {
        require_valid_group_visibility(visibility)?;
        body["visibility"] = json!(visibility);
    }
    if let Some(ref avatar) = c.avatar {
        body["avatar"] = json!(avatar);
    }
    Ok(body)
}

/// Build the request body for `codeup groups update`.
///
/// Partial-update semantics: only fields the user passed are included in the
/// body. At least one field must be set, otherwise the API would receive an
/// empty payload.
fn build_group_update_body(u: &GroupsUpdateArgs) -> Result<serde_json::Value> {
    let mut body = json!({});
    if let Some(ref name) = u.name {
        require_non_empty_text(name, "--name")?;
        body["name"] = json!(name);
    }
    if let Some(ref path) = u.path {
        require_non_empty_text(path, "--path")?;
        body["path"] = json!(path);
    }
    if let Some(ref description) = u.description {
        body["description"] = json!(description);
    }
    if let Some(ref visibility) = u.visibility {
        require_valid_group_visibility(visibility)?;
        body["visibility"] = json!(visibility);
    }
    if let Some(ref avatar) = u.avatar {
        body["avatar"] = json!(avatar);
    }
    require_non_empty_body(
        &body,
        "groups update requires at least one of --name, --path, --description, --visibility, or --avatar",
    )?;
    Ok(body)
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
        CodeupCommands::Groups(g) => exec_groups(g, &client, &org_id, format).await,
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

// ─────────────────────────── Groups ─────────────────────────────────────

/// Percent-encode a group identifier for use as a single URL path segment.
///
/// The API accepts either a numeric ID or a full path (e.g. `org/DemoGroup`).
/// When the value contains a `/`, encode it so the whole string stays one
/// segment on the wire. Plain IDs are returned untouched.
fn encode_group_id(group_id: &str) -> String {
    if group_id.contains('/') {
        urlencoding::encode(group_id).into_owned()
    } else {
        group_id.to_string()
    }
}

/// Build query parameters for `codeup groups list` (the namespaces endpoint).
///
/// Pagination is always sent. Optional filters are omitted when unset so the
/// API applies its own defaults.
fn build_namespaces_query(
    parent_id: Option<&str>,
    search: Option<&str>,
    order_by: Option<&str>,
    sort: Option<&str>,
    page: u32,
    per_page: u32,
) -> Vec<(String, String)> {
    let mut params: Vec<(String, String)> = Vec::new();
    if let Some(p) = parent_id {
        params.push(("parentId".into(), p.to_string()));
    }
    params.push(("page".into(), page.to_string()));
    params.push(("perPage".into(), per_page.to_string()));
    if let Some(s) = search {
        params.push(("search".into(), s.to_string()));
    }
    if let Some(o) = order_by {
        params.push(("orderBy".into(), o.to_string()));
    }
    if let Some(s) = sort {
        params.push(("sort".into(), s.to_string()));
    }
    params
}

/// Build query parameters for `codeup groups repos list`.
///
/// Pagination is always sent. `search` and `includeSubgroups` are omitted when
/// absent / false so the API uses its defaults.
fn build_group_repos_query(
    search: Option<&str>,
    include_subgroups: bool,
    page: u32,
    per_page: u32,
) -> Vec<(String, String)> {
    let mut params: Vec<(String, String)> = Vec::new();
    params.push(("page".into(), page.to_string()));
    params.push(("perPage".into(), per_page.to_string()));
    if let Some(s) = search {
        params.push(("search".into(), s.to_string()));
    }
    if include_subgroups {
        params.push(("includeSubgroups".into(), "true".into()));
    }
    params
}

/// Execute code-group sub-operations.
async fn exec_groups(
    args: &GroupsArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        GroupsCmds::Create(c) => {
            let body = build_group_create_body(c)?;
            let data = client
                .post(
                    &format!("/oapi/v1/codeup/organizations/{oid}/groups"),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        GroupsCmds::Update(u) => {
            let body = build_group_update_body(u)?;
            let encoded = encode_group_id(&u.group_id);
            let data = client
                .put(
                    &format!("/oapi/v1/codeup/organizations/{oid}/groups/{encoded}"),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        GroupsCmds::List(l) => {
            let query = build_namespaces_query(
                l.parent_id.as_deref(),
                l.search.as_deref(),
                l.order_by.as_deref(),
                l.sort.as_deref(),
                l.page,
                l.per_page,
            );
            let params_ref: Vec<(&str, &str)> = query
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();
            let data = client
                .get(
                    &format!("/oapi/v1/codeup/organizations/{oid}/namespaces"),
                    &params_ref,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        GroupsCmds::Get(g) => {
            let data = client
                .get(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/namespaces/{}",
                        encode_group_id(&g.group_id)
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        GroupsCmds::Repos(r) => match &r.command {
            GroupsReposCmds::List(l) => {
                let query = build_group_repos_query(
                    l.search.as_deref(),
                    l.include_subgroups,
                    l.page,
                    l.per_page,
                );
                let params_ref: Vec<(&str, &str)> = query
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect();
                let data = client
                    .get(
                        &format!(
                            "/oapi/v1/codeup/organizations/{oid}/groups/{}/repositories",
                            encode_group_id(&l.group_id)
                        ),
                        &params_ref,
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
        },
    }
    Ok(())
}

// ─────────────────────────── Repos ──────────────────────────────────────

/// Build query parameters for `codeup repos list`.
///
/// Pagination is always sent. Optional filters are omitted when unset so the
/// API applies its own defaults. `archived` is tri-state: `None` sends nothing,
/// `Some(true)` / `Some(false)` send the explicit value.
fn build_repos_list_params(l: &ReposListArgs) -> Vec<(&'static str, String)> {
    let mut params: Vec<(&'static str, String)> = vec![
        ("page", l.page.to_string()),
        ("perPage", l.per_page.to_string()),
    ];
    if let Some(ref search) = l.search {
        params.push(("search", search.clone()));
    }
    if let Some(ref order_by) = l.order_by {
        params.push(("orderBy", order_by.clone()));
    }
    if let Some(ref sort) = l.sort {
        params.push(("sort", sort.clone()));
    }
    if let Some(archived) = l.archived {
        params.push(("archived", archived.to_string()));
    }
    params
}

/// Build query parameters for `codeup repos templates list`.
///
/// `templateType` is required by the API; pagination uses the same defaults as
/// other list endpoints.
fn build_templates_list_params(t: &ReposTemplatesListArgs) -> Vec<(&'static str, String)> {
    vec![
        ("page", t.page.to_string()),
        ("perPage", t.per_page.to_string()),
        ("templateType", t.template_type.to_string()),
    ]
}

/// Check if user input is an affirmative response (y/yes, case-insensitive).
fn is_affirmative(input: &str) -> bool {
    matches!(input.trim().to_ascii_lowercase().as_str(), "y" | "yes")
}

/// Check if user input matches the repository ID or full path for delete confirmation.
///
/// Comparison is exact after trimming surrounding whitespace from both sides,
/// so `--repo-id` may be either a repository ID or a `path_with_namespace`.
fn matches_delete_confirmation(input: &str, repo_id: &str) -> bool {
    input.trim() == repo_id.trim()
}

/// Interactive double confirmation for repository deletion.
///
/// Returns `Ok(true)` if deletion should proceed, `Ok(false)` if the user
/// cancelled, or `Err` on I/O failure or non-interactive mode without `--yes`.
fn confirm_repo_deletion(repo_id: &str, yes: bool) -> Result<bool> {
    if yes {
        return Ok(true);
    }

    if !std::io::stdin().is_terminal() {
        return Err(crate::error::CliError::Config(
            "Refusing to delete in non-interactive mode. Pass --yes to confirm.".into(),
        ));
    }

    // First confirmation: y/N
    eprint!(
        "This will permanently delete repository '{}' and all its contents. Continue? [y/N] ",
        repo_id
    );
    std::io::stderr().flush()?;
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    if !is_affirmative(line.trim()) {
        eprintln!("Deletion cancelled.");
        return Ok(false);
    }

    // Second confirmation: type exact repo_id
    eprint!("Type the repository ID or full path to confirm deletion: ");
    std::io::stderr().flush()?;
    line.clear();
    std::io::stdin().read_line(&mut line)?;

    if !matches_delete_confirmation(&line, repo_id) {
        eprintln!(
            "Confirmation did not match '{}'. Deletion cancelled.",
            repo_id
        );
        return Ok(false);
    }

    Ok(true)
}

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
            let params = build_repos_list_params(l);
            let params_ref: Vec<(&str, &str)> =
                params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            let data = client
                .get(
                    &format!("/oapi/v1/codeup/organizations/{oid}/repositories"),
                    &params_ref,
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
        ReposCmds::Create(c) => {
            let body = build_repo_create_body(c)?;
            let data = client
                .post(&repos_create_path(oid, c.create_parent_path), &body)
                .await?;
            output::print_output(&data, format)?;
        }
        ReposCmds::Templates(t) => match &t.command {
            ReposTemplatesCmds::List(l) => {
                let params = build_templates_list_params(l);
                let params_ref: Vec<(&str, &str)> =
                    params.iter().map(|(k, v)| (*k, v.as_str())).collect();
                let data = client
                    .get(
                        &format!("/oapi/v1/codeup/organizations/{oid}/repositories/templates"),
                        &params_ref,
                    )
                    .await?;
                output::print_output(&data, format)?;
            }
        },
        ReposCmds::Delete(d) => {
            if !confirm_repo_deletion(&d.repo_id, d.yes)? {
                return Ok(());
            }
            let data = client
                .delete(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}",
                        d.repo_id
                    ),
                    &[],
                )
                .await?;
            output::print_output(&data, format)?;
        }
        ReposCmds::Update(u) => {
            let body = build_repo_update_body(u)?;
            let data = client
                .put(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}",
                        encode_repo_path(&u.repo_id)
                    ),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
        ReposCmds::Archive(a) => {
            let data = client
                .post(
                    &format!(
                        "/oapi/v1/codeup/organizations/{oid}/repositories/{}/archive",
                        encode_repo_path(&a.repo_id)
                    ),
                    &json!({}),
                )
                .await?;
            output::print_output(&data, format)?;
        }
        ReposCmds::Transfer(t) => {
            let body = build_repo_transfer_body(t)?;
            let data = client
                .post(
                    &format!("/oapi/v1/codeup/organizations/{oid}/repositories/transfer"),
                    &body,
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

    #[test]
    fn repos_list_params_omit_absent_filters_and_keep_defaults() {
        let args = ReposListArgs {
            page: 1,
            per_page: 20,
            search: None,
            order_by: None,
            sort: None,
            archived: None,
        };

        let params = build_repos_list_params(&args);

        assert_eq!(
            params,
            vec![("page", "1".to_string()), ("perPage", "20".to_string())]
        );
    }

    #[test]
    fn repos_list_params_tristate_archived() {
        let absent = build_repos_list_params(&ReposListArgs {
            page: 1,
            per_page: 20,
            search: None,
            order_by: None,
            sort: None,
            archived: None,
        });
        assert!(!absent.iter().any(|(k, _)| *k == "archived"));

        let true_set = build_repos_list_params(&ReposListArgs {
            page: 1,
            per_page: 20,
            search: None,
            order_by: None,
            sort: None,
            archived: Some(true),
        });
        assert!(true_set
            .iter()
            .any(|(k, v)| *k == "archived" && v == "true"));

        let false_set = build_repos_list_params(&ReposListArgs {
            page: 1,
            per_page: 20,
            search: None,
            order_by: None,
            sort: None,
            archived: Some(false),
        });
        assert!(false_set
            .iter()
            .any(|(k, v)| *k == "archived" && v == "false"));
    }

    #[test]
    fn repos_list_params_include_search_order_by_sort() {
        let args = ReposListArgs {
            page: 2,
            per_page: 50,
            search: Some("TestRepo".into()),
            order_by: Some("name".into()),
            sort: Some("asc".into()),
            archived: None,
        };

        let params = build_repos_list_params(&args);

        assert_eq!(
            params,
            vec![
                ("page", "2".to_string()),
                ("perPage", "50".to_string()),
                ("search", "TestRepo".to_string()),
                ("orderBy", "name".to_string()),
                ("sort", "asc".to_string()),
            ]
        );
    }

    // ───── groups ─────

    #[test]
    fn build_namespaces_query_omits_absent_filters() {
        let query = build_namespaces_query(None, None, None, None, 1, 20);
        assert_eq!(
            query,
            vec![
                ("page".to_string(), "1".to_string()),
                ("perPage".to_string(), "20".to_string()),
            ]
        );
    }

    #[test]
    fn templates_list_params_always_include_template_type() {
        let args = ReposTemplatesListArgs {
            template_type: 1,
            page: 1,
            per_page: 20,
        };
        let params = build_templates_list_params(&args);
        assert_eq!(
            params,
            vec![
                ("page", "1".to_string()),
                ("perPage", "20".to_string()),
                ("templateType", "1".to_string()),
            ]
        );

        let args2 = ReposTemplatesListArgs {
            template_type: 2,
            page: 3,
            per_page: 10,
        };
        let params2 = build_templates_list_params(&args2);
        assert_eq!(params2[2], ("templateType", "2".to_string()));
        assert_eq!(params2[0], ("page", "3".to_string()));
    }

    #[test]
    fn cli_parses_repos_templates_list() {
        let cli = Cli::parse_from([
            "yunxiao",
            "codeup",
            "repos",
            "templates",
            "list",
            "--template-type",
            "2",
            "--page",
            "3",
            "--per-page",
            "10",
        ]);
        let crate::cli::Commands::Codeup(codeup) = cli.command else {
            panic!("expected codeup command");
        };
        let CodeupCommands::Repos(repos) = codeup.command else {
            panic!("expected repos command");
        };
        let ReposCmds::Templates(templates) = repos.command else {
            panic!("expected templates subcommand");
        };
        match templates.command {
            ReposTemplatesCmds::List(l) => {
                assert_eq!(l.template_type, 2);
                assert_eq!(l.page, 3);
                assert_eq!(l.per_page, 10);
            }
        }
    }

    #[test]
    fn cli_repos_templates_list_requires_template_type() {
        let result = Cli::try_parse_from(["yunxiao", "codeup", "repos", "templates", "list"]);
        assert!(result.is_err());
    }

    #[test]
    fn build_namespaces_query_includes_provided_filters() {
        let query = build_namespaces_query(
            Some("1"),
            Some("Test"),
            Some("updated_at"),
            Some("desc"),
            2,
            50,
        );
        assert_eq!(
            query,
            vec![
                ("parentId".to_string(), "1".to_string()),
                ("page".to_string(), "2".to_string()),
                ("perPage".to_string(), "50".to_string()),
                ("search".to_string(), "Test".to_string()),
                ("orderBy".to_string(), "updated_at".to_string()),
                ("sort".to_string(), "desc".to_string()),
            ]
        );
    }

    #[test]
    fn build_group_repos_query_omits_defaults() {
        let query = build_group_repos_query(None, false, 1, 20);
        assert_eq!(
            query,
            vec![
                ("page".to_string(), "1".to_string()),
                ("perPage".to_string(), "20".to_string()),
            ]
        );
    }

    #[test]
    fn build_group_repos_query_includes_optional() {
        let query = build_group_repos_query(Some("TestRepo"), true, 3, 10);
        assert_eq!(
            query,
            vec![
                ("page".to_string(), "3".to_string()),
                ("perPage".to_string(), "10".to_string()),
                ("search".to_string(), "TestRepo".to_string()),
                ("includeSubgroups".to_string(), "true".to_string()),
            ]
        );
    }

    #[test]
    fn encode_group_id_leaves_plain_id_untouched() {
        assert_eq!(encode_group_id("123"), "123");
    }

    #[test]
    fn encode_group_id_percent_encodes_slash() {
        assert_eq!(
            encode_group_id("60de7a6852743a5162b5f957/DemoGroup"),
            "60de7a6852743a5162b5f957%2FDemoGroup"
        );
    }

    #[test]
    fn cli_parses_groups_commands() {
        let list =
            parse_groups_command(["yunxiao", "codeup", "groups", "list", "--parent-id", "1"]);
        assert!(matches!(list, GroupsCmds::List(_)));

        let get = parse_groups_command(["yunxiao", "codeup", "groups", "get", "--group-id", "123"]);
        let GroupsCmds::Get(args) = get else {
            panic!("expected groups get command");
        };
        assert_eq!(args.group_id, "123");

        let repos = parse_groups_command([
            "yunxiao",
            "codeup",
            "groups",
            "repos",
            "list",
            "--group-id",
            "123",
            "--include-subgroups",
        ]);
        let GroupsCmds::Repos(r) = repos else {
            panic!("expected groups repos command");
        };
        let GroupsReposCmds::List(list_args) = r.command;
        assert_eq!(list_args.group_id, "123");
        assert!(list_args.include_subgroups);
    }

    fn parse_groups_command<const N: usize>(args: [&str; N]) -> GroupsCmds {
        let cli = Cli::parse_from(args);
        let crate::cli::Commands::Codeup(codeup) = cli.command else {
            panic!("expected codeup command");
        };
        let CodeupCommands::Groups(groups) = codeup.command else {
            panic!("expected groups command");
        };
        groups.command
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

    // ── repos create ─────────────────────────────────────────────────────

    #[test]
    fn build_repo_create_body_omits_absent_optionals() {
        let args = ReposCreateArgs {
            name: "demo-repo".into(),
            path: "demo-repo".into(),
            namespace_id: None,
            description: None,
            visibility: None,
            readme_type: None,
            tags: None,
            avatar_url: None,
            template_id: None,
            template_type: None,
            sync_all_branches: false,
            create_parent_path: false,
        };

        let body = build_repo_create_body(&args).unwrap();

        assert_eq!(body["name"], "demo-repo");
        assert_eq!(body["path"], "demo-repo");
        assert!(body.get("namespaceId").is_none());
        assert!(body.get("description").is_none());
        assert!(body.get("visibility").is_none());
        assert!(body.get("readMeType").is_none());
        assert!(body.get("repositoryTags").is_none());
        assert!(body.get("avatarUrl").is_none());
        assert!(body.get("templateProject").is_none());
    }

    #[test]
    fn build_repo_create_body_includes_all_optionals() {
        let args = ReposCreateArgs {
            name: "demo-repo".into(),
            path: "demo-repo".into(),
            namespace_id: Some("2813489".into()),
            description: Some("desc".into()),
            visibility: Some("private".into()),
            readme_type: Some("USER_GUIDE".into()),
            tags: Some("tag1,tag2".into()),
            avatar_url: Some("https://example/avatar".into()),
            template_id: Some("7".into()),
            template_type: Some(2),
            sync_all_branches: true,
            create_parent_path: true,
        };

        let body = build_repo_create_body(&args).unwrap();

        assert_eq!(body["namespaceId"], 2813489);
        assert_eq!(body["description"], "desc");
        assert_eq!(body["visibility"], "private");
        assert_eq!(body["readMeType"], "USER_GUIDE");
        assert_eq!(body["repositoryTags"], json!(["tag1", "tag2"]));
        assert_eq!(body["avatarUrl"], "https://example/avatar");
        let tpl = &body["templateProject"];
        assert_eq!(tpl["templateProjectId"], 7);
        assert_eq!(tpl["templateType"], 2);
        assert_eq!(tpl["syncAllBranches"], true);
    }

    #[test]
    fn build_repo_create_body_converts_comma_tags_to_array() {
        let args = ReposCreateArgs {
            name: "r".into(),
            path: "r".into(),
            namespace_id: None,
            description: None,
            visibility: None,
            readme_type: None,
            tags: Some(" alpha , beta ".into()),
            avatar_url: None,
            template_id: None,
            template_type: None,
            sync_all_branches: false,
            create_parent_path: false,
        };

        let body = build_repo_create_body(&args).unwrap();

        assert_eq!(body["repositoryTags"], json!(["alpha", "beta"]));
    }

    #[test]
    fn build_repo_create_body_omits_template_project_without_template_id() {
        let args = ReposCreateArgs {
            name: "r".into(),
            path: "r".into(),
            namespace_id: None,
            description: None,
            visibility: None,
            readme_type: None,
            tags: None,
            avatar_url: None,
            template_id: None,
            template_type: Some(1),
            sync_all_branches: true,
            create_parent_path: false,
        };

        let body = build_repo_create_body(&args).unwrap();

        assert!(body.get("templateProject").is_none());
    }

    #[test]
    fn build_repo_create_body_rejects_invalid_visibility() {
        let args = ReposCreateArgs {
            name: "r".into(),
            path: "r".into(),
            namespace_id: None,
            description: None,
            visibility: Some("public".into()),
            readme_type: None,
            tags: None,
            avatar_url: None,
            template_id: None,
            template_type: None,
            sync_all_branches: false,
            create_parent_path: false,
        };

        let err = build_repo_create_body(&args).unwrap_err();
        assert!(err.to_string().contains("visibility"));
    }

    #[test]
    fn build_repo_create_body_accepts_full_path_namespace() {
        let args_numeric = ReposCreateArgs {
            name: "r".into(),
            path: "r".into(),
            namespace_id: Some("2813489".into()),
            description: None,
            visibility: None,
            readme_type: None,
            tags: None,
            avatar_url: None,
            template_id: None,
            template_type: None,
            sync_all_branches: false,
            create_parent_path: false,
        };
        let body = build_repo_create_body(&args_numeric).unwrap();
        assert_eq!(body["namespaceId"], 2813489);

        let args_path = ReposCreateArgs {
            name: "r".into(),
            path: "r".into(),
            namespace_id: Some("group/subgroup".into()),
            description: None,
            visibility: None,
            readme_type: None,
            tags: None,
            avatar_url: None,
            template_id: None,
            template_type: None,
            sync_all_branches: false,
            create_parent_path: false,
        };
        let body = build_repo_create_body(&args_path).unwrap();
        assert_eq!(body["namespaceId"], "group/subgroup");
    }

    #[test]
    fn repos_create_path_appends_query_only_when_requested() {
        assert_eq!(
            repos_create_path("org-123", false),
            "/oapi/v1/codeup/organizations/org-123/repositories"
        );
        assert_eq!(
            repos_create_path("org-123", true),
            "/oapi/v1/codeup/organizations/org-123/repositories?createParentPath=true"
        );
    }

    #[test]
    fn cli_parses_repos_create() {
        let cli = Cli::parse_from([
            "yunxiao",
            "codeup",
            "repos",
            "create",
            "--name",
            "demo-repo",
            "--path",
            "demo-repo",
            "--namespace-id",
            "2813489",
            "--description",
            "desc",
            "--visibility",
            "private",
            "--readme-type",
            "USER_GUIDE",
            "--tags",
            "t1,t2",
            "--avatar-url",
            "https://example/a",
            "--template-id",
            "7",
            "--template-type",
            "2",
            "--sync-all-branches",
            "--create-parent-path",
        ]);

        let crate::cli::Commands::Codeup(codeup) = cli.command else {
            panic!("expected codeup command");
        };
        let CodeupCommands::Repos(repos) = codeup.command else {
            panic!("expected repos command");
        };
        let ReposCmds::Create(c) = repos.command else {
            panic!("expected repos create command");
        };

        assert_eq!(c.name, "demo-repo");
        assert_eq!(c.path, "demo-repo");
        assert_eq!(c.namespace_id.as_deref(), Some("2813489"));
        assert_eq!(c.description.as_deref(), Some("desc"));
        assert_eq!(c.visibility.as_deref(), Some("private"));
        assert_eq!(c.readme_type.as_deref(), Some("USER_GUIDE"));
        assert_eq!(c.tags.as_deref(), Some("t1,t2"));
        assert_eq!(c.avatar_url.as_deref(), Some("https://example/a"));
        assert_eq!(c.template_id.as_deref(), Some("7"));
        assert_eq!(c.template_type, Some(2));
        assert!(c.sync_all_branches);
        assert!(c.create_parent_path);
    }

    #[test]
    fn cli_repos_create_requires_name_and_path() {
        let missing_name =
            Cli::try_parse_from(["yunxiao", "codeup", "repos", "create", "--path", "p"]);
        assert!(missing_name.is_err());

        let missing_path =
            Cli::try_parse_from(["yunxiao", "codeup", "repos", "create", "--name", "n"]);
        assert!(missing_path.is_err());
    }

    // ── T4: repos delete confirmation pure functions ─────────────────────

    #[test]
    fn matches_delete_confirmation_exact_id() {
        assert!(matches_delete_confirmation("2813489", "2813489"));
    }

    #[test]
    fn matches_delete_confirmation_full_path() {
        assert!(matches_delete_confirmation(
            "60de7a6852743a5162b5f957/DemoRepo",
            "60de7a6852743a5162b5f957/DemoRepo"
        ));
    }

    #[test]
    fn matches_delete_confirmation_mismatch() {
        assert!(!matches_delete_confirmation("2813489", "2813490"));
        assert!(!matches_delete_confirmation(
            "DemoRepo",
            "60de7a6852743a5162b5f957/DemoRepo"
        ));
    }

    #[test]
    fn matches_delete_confirmation_trims_whitespace() {
        assert!(matches_delete_confirmation("  2813489\n", "2813489"));
        assert!(matches_delete_confirmation(
            "60de7a6852743a5162b5f957/DemoRepo ",
            " 60de7a6852743a5162b5f957/DemoRepo"
        ));
    }

    #[test]
    fn is_affirmative_accepts_yes_variants() {
        assert!(is_affirmative("y"));
        assert!(is_affirmative("Y"));
        assert!(is_affirmative("yes"));
        assert!(is_affirmative("YES"));
        assert!(is_affirmative(" y "));
    }

    #[test]
    fn is_affirmative_rejects_others() {
        assert!(!is_affirmative(""));
        assert!(!is_affirmative("n"));
        assert!(!is_affirmative("no"));
        assert!(!is_affirmative("yeah"));
        assert!(!is_affirmative("   "));
    }

    #[test]
    fn cli_parses_repos_delete_with_yes_flag() {
        let cli = Cli::parse_from([
            "yunxiao",
            "codeup",
            "repos",
            "delete",
            "--repo-id",
            "2813489",
            "--yes",
        ]);
        let crate::cli::Commands::Codeup(codeup) = cli.command else {
            panic!("expected codeup command");
        };
        let CodeupCommands::Repos(repos) = codeup.command else {
            panic!("expected repos command");
        };
        let ReposCmds::Delete(args) = repos.command else {
            panic!("expected repos delete command");
        };
        assert_eq!(args.repo_id, "2813489");
        assert!(args.yes);
    }

    #[test]
    fn cli_repos_delete_yes_defaults_false() {
        let cli = Cli::parse_from([
            "yunxiao",
            "codeup",
            "repos",
            "delete",
            "--repo-id",
            "2813489",
        ]);
        let crate::cli::Commands::Codeup(codeup) = cli.command else {
            panic!("expected codeup command");
        };
        let CodeupCommands::Repos(repos) = codeup.command else {
            panic!("expected repos command");
        };
        let ReposCmds::Delete(args) = repos.command else {
            panic!("expected repos delete command");
        };
        assert_eq!(args.repo_id, "2813489");
        assert!(!args.yes);
    }

    fn parse_repos_command<const N: usize>(args: [&str; N]) -> ReposCmds {
        let cli = Cli::parse_from(args);
        let crate::cli::Commands::Codeup(codeup) = cli.command else {
            panic!("expected codeup command");
        };
        let CodeupCommands::Repos(repos) = codeup.command else {
            panic!("expected repos command");
        };
        repos.command
    }

    #[test]
    fn encode_repo_path_passes_through_plain_id() {
        assert_eq!(encode_repo_path("2813489"), "2813489");
    }

    #[test]
    fn encode_repo_path_percent_encodes_slash_in_full_path() {
        let encoded = encode_repo_path("60de7a6852743a5162b5f957/DemoRepo");
        assert_eq!(encoded, "60de7a6852743a5162b5f957%2FDemoRepo");
    }

    #[test]
    fn build_repo_update_body_includes_all_set_fields() {
        let args = ReposUpdateArgs {
            repo_id: "2813489".into(),
            name: Some("demo-repo".into()),
            description: Some("desc".into()),
            default_branch: Some("main".into()),
            visibility: Some("private".into()),
            tags: vec!["alpha".into(), "beta".into()],
        };
        let body = build_repo_update_body(&args).unwrap();
        assert_eq!(body["name"], "demo-repo");
        assert_eq!(body["description"], "desc");
        assert_eq!(body["default_branch"], "main");
        assert_eq!(body["visibility"], "private");
        assert_eq!(body["repositoryTags"], json!(["alpha", "beta"]));
    }

    #[test]
    fn build_repo_update_body_omits_name_when_absent() {
        let args = ReposUpdateArgs {
            repo_id: "2813489".into(),
            name: None,
            description: Some("desc".into()),
            default_branch: Some("main".into()),
            visibility: Some("private".into()),
            tags: vec!["t".into()],
        };
        let body = build_repo_update_body(&args).unwrap();
        assert!(body.get("name").is_none());
        assert_eq!(body["description"], "desc");
        assert_eq!(body["default_branch"], "main");
        assert_eq!(body["visibility"], "private");
        assert_eq!(body["repositoryTags"], json!(["t"]));
    }

    #[test]
    fn build_repo_update_body_omits_description_when_absent() {
        let args = ReposUpdateArgs {
            repo_id: "2813489".into(),
            name: Some("demo".into()),
            description: None,
            default_branch: Some("main".into()),
            visibility: Some("private".into()),
            tags: vec!["t".into()],
        };
        let body = build_repo_update_body(&args).unwrap();
        assert!(body.get("description").is_none());
        assert_eq!(body["name"], "demo");
    }

    #[test]
    fn build_repo_update_body_omits_default_branch_when_absent() {
        let args = ReposUpdateArgs {
            repo_id: "2813489".into(),
            name: Some("demo".into()),
            description: Some("desc".into()),
            default_branch: None,
            visibility: Some("private".into()),
            tags: vec!["t".into()],
        };
        let body = build_repo_update_body(&args).unwrap();
        assert!(body.get("default_branch").is_none());
        assert_eq!(body["name"], "demo");
    }

    #[test]
    fn build_repo_update_body_omits_visibility_when_absent() {
        let args = ReposUpdateArgs {
            repo_id: "2813489".into(),
            name: Some("demo".into()),
            description: Some("desc".into()),
            default_branch: Some("main".into()),
            visibility: None,
            tags: vec!["t".into()],
        };
        let body = build_repo_update_body(&args).unwrap();
        assert!(body.get("visibility").is_none());
        assert_eq!(body["name"], "demo");
    }

    #[test]
    fn build_repo_update_body_omits_tags_when_absent() {
        let args = ReposUpdateArgs {
            repo_id: "2813489".into(),
            name: Some("demo".into()),
            description: Some("desc".into()),
            default_branch: Some("main".into()),
            visibility: Some("private".into()),
            tags: vec![],
        };
        let body = build_repo_update_body(&args).unwrap();
        assert!(body.get("repositoryTags").is_none());
        assert_eq!(body["name"], "demo");
    }

    #[test]
    fn build_repo_update_body_rejects_empty_tag_string() {
        let args = ReposUpdateArgs {
            repo_id: "2813489".into(),
            name: None,
            description: None,
            default_branch: None,
            visibility: None,
            tags: vec!["".into()],
        };
        let err = build_repo_update_body(&args).unwrap_err();
        assert!(err.to_string().contains("cannot be empty"));
    }

    #[test]
    fn build_repo_update_body_all_empty_returns_empty_body() {
        let args = ReposUpdateArgs {
            repo_id: "2813489".into(),
            name: None,
            description: None,
            default_branch: None,
            visibility: None,
            tags: vec![],
        };
        let body = build_repo_update_body(&args).unwrap();
        assert!(body.as_object().unwrap().is_empty());
    }

    #[test]
    fn build_repo_transfer_body_uses_repo_and_group_id() {
        let args = ReposTransferArgs {
            repo_id: "60de7a6852743a5162b5f957/DemoRepo".into(),
            group_id: "60de7a6852743a5162b5f957/target-group".into(),
        };
        let body = build_repo_transfer_body(&args).unwrap();
        assert_eq!(body["repositoryId"], "60de7a6852743a5162b5f957/DemoRepo");
        assert_eq!(body["groupId"], "60de7a6852743a5162b5f957/target-group");
    }

    #[test]
    fn cli_parses_repos_update_with_all_flags() {
        let ReposCmds::Update(args) = parse_repos_command([
            "yunxiao",
            "codeup",
            "repos",
            "update",
            "--repo-id",
            "2813489",
            "--name",
            "demo",
            "--description",
            "desc",
            "--default-branch",
            "main",
            "--visibility",
            "private",
            "--tags",
            "a,b",
        ]) else {
            panic!("expected repos update");
        };
        assert_eq!(args.repo_id, "2813489");
        assert_eq!(args.name.as_deref(), Some("demo"));
        assert_eq!(args.description.as_deref(), Some("desc"));
        assert_eq!(args.default_branch.as_deref(), Some("main"));
        assert_eq!(args.visibility.as_deref(), Some("private"));
        assert_eq!(args.tags, vec!["a", "b"]);
    }

    #[test]
    fn cli_rejects_invalid_repos_update_visibility() {
        let err = Cli::try_parse_from([
            "yunxiao",
            "codeup",
            "repos",
            "update",
            "--repo-id",
            "2813489",
            "--visibility",
            "public",
        ])
        .unwrap_err();
        assert!(err.to_string().contains("private"));
        assert!(err.to_string().contains("internal"));
    }

    #[test]
    fn cli_parses_repos_archive() {
        let ReposCmds::Archive(args) = parse_repos_command([
            "yunxiao",
            "codeup",
            "repos",
            "archive",
            "--repo-id",
            "2813489",
        ]) else {
            panic!("expected repos archive");
        };
        assert_eq!(args.repo_id, "2813489");
    }

    #[test]
    fn cli_parses_repos_transfer() {
        let ReposCmds::Transfer(args) = parse_repos_command([
            "yunxiao",
            "codeup",
            "repos",
            "transfer",
            "--repo-id",
            "2813489",
            "--group-id",
            "60de7a6852743a5162b5f957/target",
        ]) else {
            panic!("expected repos transfer");
        };
        assert_eq!(args.repo_id, "2813489");
        assert_eq!(args.group_id, "60de7a6852743a5162b5f957/target");
    }

    #[test]
    fn build_group_create_body_includes_only_required_fields() {
        let args = GroupsCreateArgs {
            name: "DemoGroup".into(),
            path: "demo-group".into(),
            parent_id: None,
            description: None,
            visibility: None,
            avatar: None,
        };

        let body = build_group_create_body(&args).unwrap();

        assert_eq!(body["name"], "DemoGroup");
        assert_eq!(body["path"], "demo-group");
        assert!(body.get("parentId").is_none());
        assert!(body.get("description").is_none());
        assert!(body.get("visibility").is_none());
        assert!(body.get("avatar").is_none());
    }

    #[test]
    fn build_group_create_body_includes_all_optional_fields() {
        let args = GroupsCreateArgs {
            name: "DemoGroup".into(),
            path: "demo-group".into(),
            parent_id: Some(42),
            description: Some("group description".into()),
            visibility: Some("private".into()),
            avatar: Some("https://example/example/w/100/h/100".into()),
        };

        let body = build_group_create_body(&args).unwrap();

        assert_eq!(body["name"], "DemoGroup");
        assert_eq!(body["path"], "demo-group");
        assert_eq!(body["parentId"], 42);
        assert_eq!(body["description"], "group description");
        assert_eq!(body["visibility"], "private");
        assert_eq!(body["avatar"], "https://example/example/w/100/h/100");
    }

    #[test]
    fn build_group_create_body_rejects_invalid_visibility() {
        let args = GroupsCreateArgs {
            name: "DemoGroup".into(),
            path: "demo-group".into(),
            parent_id: None,
            description: None,
            visibility: Some("public".into()),
            avatar: None,
        };

        let err = build_group_create_body(&args).unwrap_err();

        let msg = err.to_string();
        assert!(msg.contains("private"));
        assert!(msg.contains("internal"));
    }

    #[test]
    fn build_group_update_body_omits_unset_fields() {
        let args = GroupsUpdateArgs {
            group_id: "123".into(),
            name: None,
            path: None,
            description: Some("new description".into()),
            visibility: None,
            avatar: None,
        };

        let body = build_group_update_body(&args).unwrap();

        assert_eq!(body["description"], "new description");
        assert!(body.get("name").is_none());
        assert!(body.get("path").is_none());
        assert!(body.get("visibility").is_none());
        assert!(body.get("avatar").is_none());
    }

    #[test]
    fn build_group_update_body_requires_at_least_one_field() {
        let args = GroupsUpdateArgs {
            group_id: "123".into(),
            name: None,
            path: None,
            description: None,
            visibility: None,
            avatar: None,
        };

        let err = build_group_update_body(&args).unwrap_err();

        assert!(err.to_string().contains("at least one"));
    }

    #[test]
    fn build_group_update_body_rejects_invalid_visibility() {
        let args = GroupsUpdateArgs {
            group_id: "123".into(),
            name: None,
            path: None,
            description: None,
            visibility: Some("public".into()),
            avatar: None,
        };

        let err = build_group_update_body(&args).unwrap_err();

        assert!(err.to_string().contains("private"));
    }

    #[test]
    fn cli_parses_groups_create_required_only() {
        let cli = Cli::parse_from([
            "yunxiao",
            "codeup",
            "groups",
            "create",
            "--name",
            "DemoGroup",
            "--path",
            "demo-group",
        ]);
        let crate::cli::Commands::Codeup(codeup) = cli.command else {
            panic!("expected codeup command");
        };
        let CodeupCommands::Groups(groups) = codeup.command else {
            panic!("expected groups command");
        };
        let GroupsCmds::Create(args) = groups.command else {
            panic!("expected groups create command");
        };

        assert_eq!(args.name, "DemoGroup");
        assert_eq!(args.path, "demo-group");
        assert!(args.parent_id.is_none());
        assert!(args.visibility.is_none());
    }

    #[test]
    fn cli_parses_groups_update_flags() {
        let cli = Cli::parse_from([
            "yunxiao",
            "codeup",
            "groups",
            "update",
            "--group-id",
            "123",
            "--name",
            "NewName",
            "--visibility",
            "internal",
        ]);
        let crate::cli::Commands::Codeup(codeup) = cli.command else {
            panic!("expected codeup command");
        };
        let CodeupCommands::Groups(groups) = codeup.command else {
            panic!("expected groups command");
        };
        let GroupsCmds::Update(args) = groups.command else {
            panic!("expected groups update command");
        };

        assert_eq!(args.group_id, "123");
        assert_eq!(args.name.as_deref(), Some("NewName"));
        assert_eq!(args.visibility.as_deref(), Some("internal"));
        assert!(args.description.is_none());
        assert!(args.avatar.is_none());
    }
}
