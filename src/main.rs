//! Icebox CLI binary entrypoint.
//!
//! This crate currently provides a minimal bootstrap command used to verify
//! project wiring and documentation generation.

use clap::{Parser, error::ErrorKind};

/// Top-level CLI parser scaffold for Icebox.
#[derive(Debug, Parser)]
#[command(
    name = "icebox",
    version,
    about = "Secure credential broker for AI agents",
    after_help = "Repository: https://github.com/torbenanderson/icebox-cli"
)]
pub struct Cli {}

/// Runs the current bootstrap CLI behavior.
///
/// Today this parses command-line input via `clap` and returns parse outcomes
/// to the caller. As subcommands are added, this remains the single entrypoint
/// for top-level CLI execution flow.
pub fn run() {
    if let Err(err) = Cli::try_parse() {
        let exit_code = match err.kind() {
            ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => 0,
            _ => 2,
        };

        // clap already formats user-facing diagnostics; we print and exit non-zero.
        let _ = err.print();
        std::process::exit(exit_code);
    }
}

fn main() {
    run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_args_succeeds() {
        let result = Cli::try_parse_from(["icebox"]);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_unknown_flag_fails() {
        let err = Cli::try_parse_from(["icebox", "--not-a-real-flag"])
            .expect_err("expected parse failure");
        assert_eq!(err.kind(), ErrorKind::UnknownArgument);
    }
}
