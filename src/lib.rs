//! Icebox CLI library surface.
//!
//! The binary entrypoint remains thin in `src/main.rs`; core parsing and module
//! wiring live here so they are testable and reusable.

pub mod agent;
pub mod config;
pub mod did;
mod enclave;
mod error;
mod hardening;
pub mod runner;
mod util;
pub mod vault;

use clap::{Parser, error::ErrorKind};
use std::ffi::OsStr;

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    /// Registers a new agent identity.
    RegisterAgent {
        /// Agent name to register.
        name: String,
    },
}

/// Top-level CLI parser scaffold for Icebox.
#[derive(Debug, Parser)]
#[command(
    name = "icebox-cli",
    version,
    about = "Secure credential broker for AI agents",
    after_help = "Environment:\n  ICEBOX_HOME  Override Icebox home path (default: ~/.icebox)\n\nRepository: https://github.com/torbenanderson/icebox-cli"
)]
pub struct Cli {
    /// Enables debug diagnostics in CLI error output.
    #[arg(long)]
    pub debug: bool,
    /// Supported Icebox commands.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Parses CLI arguments into [`Cli`].
pub fn parse_cli_from<I, T>(args: I) -> Result<Cli, clap::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    Cli::try_parse_from(args)
}

/// Runs CLI parsing and prints user-facing diagnostics when parsing fails.
pub fn run() -> i32 {
    run_from_args(std::env::args_os().collect())
}

fn run_from_args(args: Vec<std::ffi::OsString>) -> i32 {
    let debug_enabled = args.iter().any(|arg| arg == OsStr::new("--debug"));

    if let Err(err) = hardening::disable_core_dumps() {
        eprintln!("Security hardening failed: {err}");
        return 1;
    }

    match parse_cli_from(args) {
        Ok(cli) => match run_command(cli) {
            Ok(()) => 0,
            Err(err) => {
                let (code, detail) = match &err {
                    agent::RegisterAgentError::InvalidName(_) => {
                        (error::IceErrorCode::InvalidAgentName, Some(err.to_string()))
                    }
                    agent::RegisterAgentError::DuplicateName { .. } => (
                        error::IceErrorCode::DuplicateAgentName,
                        Some(err.to_string()),
                    ),
                    agent::RegisterAgentError::DuplicateRegistryNames { .. } => (
                        error::IceErrorCode::DuplicateConfigAgentNames,
                        Some(err.to_string()),
                    ),
                    agent::RegisterAgentError::InvalidConfig { .. } => {
                        (error::IceErrorCode::InvalidConfig, Some(err.to_string()))
                    }
                    agent::RegisterAgentError::Enclave { .. } => (
                        error::IceErrorCode::EnclaveFailure,
                        if debug_enabled {
                            Some(err.to_string())
                        } else {
                            Some(
                                "Secure Enclave operation failed. Check supported hardware and signing/entitlements."
                                    .to_string(),
                            )
                        },
                    ),
                    _ => (error::IceErrorCode::IdentitySetup, Some(err.to_string())),
                };
                eprintln!(
                    "{}",
                    error::format_runtime_error(code, debug_enabled, detail.as_deref())
                );
                1
            }
        },
        Err(err) => {
            let exit_code = match err.kind() {
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => 0,
                _ => 2,
            };

            if exit_code == 0 {
                let _ = err.print();
                return 0;
            }

            let code = error::map_clap_error(err.kind());
            let detail = err.to_string();
            eprintln!(
                "{}",
                error::format_cli_error(code, debug_enabled, Some(detail.as_str()))
            );

            exit_code
        }
    }
}

fn run_command(cli: Cli) -> Result<(), agent::RegisterAgentError> {
    match cli.command {
        Some(Commands::RegisterAgent { name }) => agent::register_agent(&name),
        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::error::ErrorKind;

    #[test]
    fn parse_empty_args_succeeds() {
        let result = parse_cli_from(["icebox-cli"]);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_unknown_flag_fails() {
        let err = parse_cli_from(["icebox-cli", "--not-a-real-flag"])
            .expect_err("expected parse failure");
        assert_eq!(err.kind(), ErrorKind::UnknownArgument);
    }

    #[test]
    fn parse_register_agent_command_succeeds() {
        let result = parse_cli_from(["icebox-cli", "register-agent", "claw"])
            .expect("expected command parse success");
        match result.command {
            Some(Commands::RegisterAgent { name }) => assert_eq!(name, "claw"),
            _ => panic!("expected register-agent command"),
        }
    }
}
