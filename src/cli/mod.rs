//! CLI module – root command definition and global flags.
//!
//! Defines the [`Cli`] struct that is the entry-point for argument parsing.
//! Global flags (output format, timeout, log level, token, endpoint, org ID)
//! are declared here and threaded into subcommand handlers.

pub mod commands;

pub use commands::Commands;

use crate::config::types::{LogLevel, OutputFormat};
use clap::Parser;

/// YunXiao CLI – Alibaba Cloud DevOps command-line client.
///
/// A comprehensive CLI for interacting with the Yunxiao (云效) platform,
/// covering authentication, project collaboration, code management,
/// pipelines, application delivery, packages, testing, and more.
#[derive(Debug, Parser)]
#[command(
    name = "yunxiao",
    version = concat!(env!("CARGO_PKG_VERSION"), "\nBuild time: ", env!("BUILD_TIME"), "\nGit commit: ", env!("GIT_COMMIT_HASH")),
    about = "YunXiao CLI - Alibaba Cloud DevOps command-line client",
    long_about = "A comprehensive command-line interface for the Alibaba Cloud \
                  YunXiao (云效) DevOps platform. Manage projects, code, pipelines, \
                  deployments, tests, and more from your terminal."
)]
pub struct Cli {
    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Commands,

    /// Output format (json, text, table, markdown).
    #[arg(short, long, global = true, env = "YUNXIAO_CLI_OUTPUT")]
    pub output: Option<OutputFormat>,

    /// API request timeout in seconds.
    #[arg(long, global = true, env = "YUNXIAO_CLI_TIMEOUT")]
    pub timeout: Option<u64>,

    /// Log level (debug, info, warn, error).
    #[arg(long, global = true, env = "YUNXIAO_CLI_LOG_LEVEL")]
    pub log_level: Option<LogLevel>,

    /// Personal access token for API authentication.
    #[arg(long, global = true, env = "YUNXIAO_CLI_TOKEN")]
    pub token: Option<String>,

    /// API endpoint (URL or domain, e.g. https://openapi-rdc.aliyuncs.com).
    #[arg(long, global = true, env = "YUNXIAO_CLI_ENDPOINT")]
    pub endpoint: Option<String>,

    /// Organization ID for org-scoped operations. Get via: yunxiao org list or set via: yunxiao config set organization_id <ID>
    #[arg(long, global = true, env = "YUNXIAO_CLI_ORG_ID")]
    pub org_id: Option<String>,
}
