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

#[cfg(test)]
#[cfg(not(target_os = "macos"))]
mod non_macos_tests {
    use super::*;

    #[test]
    fn enclave_stub_returns_stub() {
        assert_eq!(enclave_backend_name(), "stub");
    }
}

#[cfg(test)]
#[cfg(target_os = "macos")]
mod macos_tests {
    use super::*;

    #[test]
    fn enclave_darwin_returns_secure_enclave() {
        assert_eq!(enclave_backend_name(), "secure-enclave");
    }
}
