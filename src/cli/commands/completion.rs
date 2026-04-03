//! `completion` subcommand – generate shell completion scripts.
//!
//! Uses [`clap_complete`] to emit completion scripts for Bash, Zsh, Fish,
//! PowerShell, and Elvish to stdout.

use clap::{Args, Subcommand, ValueEnum};
use clap_complete::{self, Shell};
use std::io;

use crate::error::Result;

/// Arguments for the `completion` subcommand.
#[derive(Debug, Args)]
pub struct CompletionArgs {
    #[command(subcommand)]
    pub command: CompletionCommands,
}

/// Completion operations.
#[derive(Debug, Subcommand)]
pub enum CompletionCommands {
    /// Generate a completion script for the specified shell.
    Generate(GenerateArgs),
}

/// Arguments for `completion generate`.
#[derive(Debug, Args)]
pub struct GenerateArgs {
    /// Target shell.
    #[arg(long, value_enum)]
    pub shell: ShellChoice,
}

/// Supported shell targets for completion generation.
#[derive(Debug, Clone, ValueEnum)]
pub enum ShellChoice {
    /// Bash shell.
    Bash,
    /// Zsh shell.
    Zsh,
    /// Fish shell.
    Fish,
    /// PowerShell.
    Powershell,
    /// Elvish shell.
    Elvish,
}

impl From<ShellChoice> for Shell {
    fn from(choice: ShellChoice) -> Self {
        match choice {
            ShellChoice::Bash => Shell::Bash,
            ShellChoice::Zsh => Shell::Zsh,
            ShellChoice::Fish => Shell::Fish,
            ShellChoice::Powershell => Shell::PowerShell,
            ShellChoice::Elvish => Shell::Elvish,
        }
    }
}

/// Execute the `completion` subcommand.
///
/// Generates a shell completion script and writes it to stdout.
/// Users typically redirect the output to a file or source it directly.
pub fn execute(args: &CompletionArgs) -> Result<()> {
    match &args.command {
        CompletionCommands::Generate(g) => {
            let shell: Shell = g.shell.clone().into();
            // Build a clap::Command from the CLI struct so clap_complete can
            // introspect all subcommands and flags.
            let mut cmd = <crate::cli::Cli as clap::CommandFactory>::command();
            clap_complete::generate(shell, &mut cmd, "yunxiao", &mut io::stdout());
            Ok(())
        }
    }
}
