//! Thin binary entrypoint for `icebox-cli`.
//! `lib.rs` owns the CLI logic, and `run()` returns the final process exit code.

fn main() {
    std::process::exit(icebox_cli::run());
}
