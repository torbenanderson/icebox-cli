//! Icebox CLI binary entrypoint.
//!
//! This crate currently provides a minimal bootstrap command used to verify
//! project wiring and documentation generation.

/// Runs the current bootstrap CLI behavior.
///
/// Today this emits a simple startup message to standard output.
/// As command parsing and subcommands are added, this function remains the
/// single entrypoint for top-level CLI execution flow.
pub fn run() {
    // Bootstrap placeholder output until real subcommands are wired.
    println!("Hello, world!");
}

fn main() {
    run();
}
