//! yunxiao-cli – entry point.
//!
//! Parses CLI arguments, initialises logging, resolves configuration, and
//! dispatches to the appropriate subcommand handler.

use clap::Parser;
use log::debug;

use yunxiao_cli::cli::commands::Commands;
use yunxiao_cli::cli::Cli;
use yunxiao_cli::config;

#[tokio::main]
async fn main() {
    // Parse command-line arguments via clap derive.
    let cli = Cli::parse();

    // Resolve log level and initialise the logger.
    let log_level = config::resolve_log_level(cli.log_level.as_ref());
    env_logger::Builder::new()
        .filter_level(log_level.to_level_filter())
        .init();
    debug!("Log level set to {}", log_level);

    // Resolve the output format once for all subcommands.
    let format = config::resolve_output_format(cli.output.as_ref());
    debug!("Output format: {}", format);

    // Convenience references for optional global flags.
    let cli_token = cli.token.as_deref();
    let cli_endpoint = cli.endpoint.as_deref();
    let cli_timeout = cli.timeout;
    let cli_org_id = cli.org_id.as_deref();

    // Dispatch to the appropriate subcommand handler.
    let result = match &cli.command {
        Commands::Auth(args) => {
            yunxiao_cli::cli::commands::auth::execute(
                args,
                &format,
                cli_token,
                cli_endpoint,
                cli_timeout,
            )
            .await
        }
        Commands::Config(args) => {
            yunxiao_cli::cli::commands::config_cmd::execute(args, &format).await
        }
        Commands::Org(args) => {
            yunxiao_cli::cli::commands::org::execute(
                args,
                &format,
                cli_token,
                cli_endpoint,
                cli_timeout,
                cli_org_id,
            )
            .await
        }
        Commands::Projex(args) => {
            yunxiao_cli::cli::commands::projex::execute(
                args,
                &format,
                cli_token,
                cli_endpoint,
                cli_timeout,
                cli_org_id,
            )
            .await
        }
        Commands::Codeup(args) => {
            yunxiao_cli::cli::commands::codeup::execute(
                args,
                &format,
                cli_token,
                cli_endpoint,
                cli_timeout,
                cli_org_id,
            )
            .await
        }
        Commands::Flow(args) => {
            yunxiao_cli::cli::commands::flow::execute(
                args,
                &format,
                cli_token,
                cli_endpoint,
                cli_timeout,
                cli_org_id,
            )
            .await
        }
        Commands::Appstack(args) => {
            yunxiao_cli::cli::commands::appstack::execute(
                args,
                &format,
                cli_token,
                cli_endpoint,
                cli_timeout,
                cli_org_id,
            )
            .await
        }
        Commands::Packages(args) => {
            yunxiao_cli::cli::commands::packages::execute(
                args,
                &format,
                cli_token,
                cli_endpoint,
                cli_timeout,
                cli_org_id,
            )
            .await
        }
        Commands::Testhub(args) => {
            yunxiao_cli::cli::commands::testhub::execute(
                args,
                &format,
                cli_token,
                cli_endpoint,
                cli_timeout,
                cli_org_id,
            )
            .await
        }
        Commands::Insight(args) => {
            yunxiao_cli::cli::commands::insight::execute(args, &format).await
        }
        Commands::Thoughts(args) => {
            yunxiao_cli::cli::commands::thoughts::execute(args, &format).await
        }
        Commands::Completion(args) => {
            // Completion is synchronous – no API calls needed.
            yunxiao_cli::cli::commands::completion::execute(args)
        }
    };

    // Handle top-level errors with a user-friendly message and exit code.
    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
