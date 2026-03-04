//! DID/domain identity scaffolding.

#[cfg(target_os = "macos")]
mod backend_darwin;
#[cfg(not(target_os = "macos"))]
mod backend_stub;

/// Returns the active enclave backend name for the current platform.
pub fn enclave_backend_name() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        backend_darwin::backend_name()
    }

    #[cfg(not(target_os = "macos"))]
    {
        backend_stub::backend_name()
    }
}

/// Derives the MVP DID compatibility identifier from a raw Ed25519 public key.
pub fn did_from_public_key(public_key: &[u8; 32]) -> String {
    format!(
        "did:key:ed25519-raw:{}",
        crate::util::bytes_to_hex(public_key)
    )
}

#[cfg(test)]
#[cfg(not(target_os = "macos"))]
mod non_macos_tests {
    use super::*;

    #[test]
    fn backend_stub_returns_stub() {
        assert_eq!(enclave_backend_name(), "stub");
    }

    #[test]
    fn did_from_public_key_returns_raw_hex_did() {
        let public_key = [0xABu8; 32];
        assert_eq!(
            did_from_public_key(&public_key),
            "did:key:ed25519-raw:abababababababababababababababababababababababababababababababab"
        );
    }
}

#[cfg(test)]
#[cfg(target_os = "macos")]
mod macos_tests {
    use super::*;

    #[test]
    fn backend_darwin_returns_secure_enclave() {
        assert_eq!(enclave_backend_name(), "secure-enclave");
    }
}
