//! Non-macOS enclave placeholder backend.

/// Stub backend identifier for non-macOS targets.
pub fn backend_name() -> &'static str {
    "stub"
}
