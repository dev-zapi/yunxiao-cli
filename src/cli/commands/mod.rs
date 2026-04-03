//! Commands registry for the YunXiao CLI.
//!
//! Defines the [`Commands`] enum that maps every top-level subcommand to its
//! argument struct. Each variant is handled by the corresponding module's
//! `execute` function.

pub mod appstack;
pub mod auth;
pub mod codeup;
pub mod completion;
pub mod config_cmd;
pub mod flow;
pub mod insight;
pub mod org;
pub mod packages;
pub mod projex;
pub mod testhub;
pub mod thoughts;

use clap::Subcommand;

/// Top-level subcommands available in the YunXiao CLI.
///
/// Each variant carries its own argument struct (parsed by clap) and
/// delegates execution to the module-level `execute` function.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Authentication management – login, token management, user info.
    Auth(auth::AuthArgs),

    /// Local configuration management.
    Config(config_cmd::ConfigArgs),

    /// Organization and member management.
    Org(org::OrgArgs),

    /// Project collaboration – work items, sprints, versions.
    Projex(projex::ProjexArgs),

    /// Code management – repositories, branches, merge requests.
    Codeup(codeup::CodeupArgs),

    /// Pipeline management – create, run, query pipelines.
    Flow(flow::FlowArgs),

    /// Application delivery – apps, environments, deployments.
    Appstack(appstack::AppstackArgs),

    /// Package repository management.
    Packages(packages::PackagesArgs),

    /// Test management – test cases, plans, results.
    Testhub(testhub::TesthubArgs),

    /// Efficiency analytics and metrics.
    Insight(insight::InsightArgs),

    /// Knowledge base – documents, pages.
    Thoughts(thoughts::ThoughtsArgs),

    /// Generate shell completion scripts.
    Completion(completion::CompletionArgs),
}
