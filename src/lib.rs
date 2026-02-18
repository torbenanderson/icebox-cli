//! Icebox CLI library surface.
//!
//! The binary entrypoint remains thin in `src/main.rs`; core parsing and module
//! wiring live here so they are testable and reusable.

pub mod agent;
pub mod config;
pub mod did;
pub mod runner;
pub mod vault;

use clap::{Parser, error::ErrorKind};

/// Top-level CLI parser scaffold for Icebox.
#[derive(Debug, Parser)]
#[command(
    name = "icebox-cli",
    version,
    about = "Secure credential broker for AI agents",
    after_help = "Repository: https://github.com/torbenanderson/icebox-cli"
)]
pub struct Cli {}

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
    match parse_cli_from(std::env::args_os()) {
        Ok(_) => 0,
        Err(err) => {
            let exit_code = match err.kind() {
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => 0,
                _ => 2,
            };

            let _ = err.print();
            exit_code
        }
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
}
