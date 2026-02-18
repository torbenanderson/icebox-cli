//! DID/domain identity scaffolding.

#[cfg(target_os = "macos")]
mod enclave_darwin;
#[cfg(not(target_os = "macos"))]
mod enclave_stub;

/// Returns the active enclave backend name for the current platform.
pub fn enclave_backend_name() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        enclave_darwin::backend_name()
    }

    #[cfg(not(target_os = "macos"))]
    {
        enclave_stub::backend_name()
    }
}
