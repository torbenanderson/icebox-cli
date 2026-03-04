//! macOS Secure Enclave backend boundary.
//!
//! Raw Security.framework FFI lives in this module as implementation expands.

/// macOS enclave backend identifier.
pub fn backend_name() -> &'static str {
    "secure-enclave"
}
