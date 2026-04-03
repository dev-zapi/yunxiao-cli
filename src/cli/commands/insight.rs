//! `insight` subcommand – efficiency analytics and metrics.
//!
//! The Yunxiao Insight API surface is limited in the public OpenAPI.
//! These commands are placeholder stubs that will be fleshed out once the
//! API becomes available.

use clap::{Args, Subcommand};
use serde_json::json;

use crate::config::types::OutputFormat;
use crate::error::Result;
use crate::output;

/// Arguments for the `insight` subcommand.
#[derive(Debug, Args)]
pub struct InsightArgs {
    #[command(subcommand)]
    pub command: InsightCommands,
}

/// Available insight operations (placeholder).
#[derive(Debug, Subcommand)]
pub enum InsightCommands {
    /// List efficiency metrics.
    Metrics(MetricsArgs),
    /// Get a specific report.
    Reports(ReportsArgs),
}

// ─────────────────────────── Metrics ────────────────────────────────────

/// Arguments for `insight metrics`.
#[derive(Debug, Args)]
pub struct MetricsArgs {
    #[command(subcommand)]
    pub command: MetricsCmds,
}

/// Metrics operations.
#[derive(Debug, Subcommand)]
pub enum MetricsCmds {
    /// List available metrics, optionally filtered by dimension.
    List(MetricsListArgs),
}

/// Arguments for `insight metrics list`.
#[derive(Debug, Args)]
pub struct MetricsListArgs {
    /// Filter by dimension (e.g. delivery, quality, collaboration).
    #[arg(long)]
    pub dimension: Option<String>,
}

// ─────────────────────────── Reports ────────────────────────────────────

/// Arguments for `insight reports`.
#[derive(Debug, Args)]
pub struct ReportsArgs {
    #[command(subcommand)]
    pub command: ReportsCmds,
}

/// Report operations.
#[derive(Debug, Subcommand)]
pub enum ReportsCmds {
    /// Get a specific report by ID.
    Get(ReportGetArgs),
}

/// Arguments for `insight reports get`.
#[derive(Debug, Args)]
pub struct ReportGetArgs {
    /// Report ID.
    #[arg(long)]
    pub report_id: String,
}

// ─────────────────────────── Execute ────────────────────────────────────

/// Execute the `insight` subcommand tree.
///
/// Currently prints informational "coming soon" messages because the
/// Yunxiao Insight public API is limited.
pub async fn execute(args: &InsightArgs, format: &OutputFormat) -> Result<()> {
    match &args.command {
        InsightCommands::Metrics(m) => match &m.command {
            MetricsCmds::List(l) => {
                let dimension_msg = l
                    .dimension
                    .as_deref()
                    .unwrap_or("all");
                let data = json!({
                    "status": "coming_soon",
                    "message": format!(
                        "Insight metrics (dimension: {}) are not yet available via the public API. \
                         This feature will be enabled once Yunxiao publishes the Insight OpenAPI endpoints.",
                        dimension_msg
                    ),
                });
                output::print_output(&data, format)?;
            }
        },
        InsightCommands::Reports(r) => match &r.command {
            ReportsCmds::Get(g) => {
                let data = json!({
                    "status": "coming_soon",
                    "message": format!(
                        "Insight report '{}' cannot be retrieved yet. \
                         The Insight API is not publicly available. Stay tuned for updates.",
                        g.report_id
                    ),
                });
                output::print_output(&data, format)?;
            }
        },
    }
    Ok(())
}
