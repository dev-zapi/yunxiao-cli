//! Program sub-operations for `projex programs`.

use clap::{Args, Subcommand};
use serde_json::json;

use super::require_org;
use crate::client::ApiClient;
use crate::config::types::OutputFormat;
use crate::error::Result;
use crate::output;

/// Arguments for `projex programs`.
#[derive(Debug, Args)]
pub struct ProgramsArgs {
    #[command(subcommand)]
    pub command: ProgramsCmds,
}

/// Program operations.
#[derive(Debug, Subcommand)]
pub enum ProgramsCmds {
    /// Search programs by keyword.
    Search(ProgramSearchArgs),
}

/// Arguments for `projex programs search`.
#[derive(Debug, Args)]
pub struct ProgramSearchArgs {
    /// Search keyword.
    #[arg(long)]
    pub keyword: Option<String>,
}

/// Execute program sub-operations.
pub(super) async fn exec_programs(
    args: &ProgramsArgs,
    client: &ApiClient,
    org_id: &Option<String>,
    format: &OutputFormat,
) -> Result<()> {
    let oid = require_org(org_id)?;
    match &args.command {
        ProgramsCmds::Search(s) => {
            let mut body = json!({});
            if let Some(ref kw) = s.keyword {
                body["keyword"] = json!(kw);
            }
            let data = client
                .post(
                    &format!("/oapi/v1/projex/organizations/{oid}/programs:search"),
                    &body,
                )
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}
