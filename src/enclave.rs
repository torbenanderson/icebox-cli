//! Local wrapping-key creation helpers for identity registration.

use std::error::Error;
use std::fmt::{Display, Formatter};

/// Errors returned by enclave/keychain wrapping-key operations.
#[derive(Debug)]
pub(crate) enum EnclaveError {
    /// Test-only deterministic failure hook.
    ForcedFailure,
    /// Platform API returned an error.
    Platform(String),
}

impl Display for EnclaveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ForcedFailure => f.write_str("forced enclave failure"),
            Self::Platform(message) => write!(f, "{message}"),
        }
    }
}

impl Error for EnclaveError {}

fn maybe_force_failure() -> Result<(), EnclaveError> {
    if std::env::var("ICEBOX_TEST_FORCE_ENCLAVE_ERROR")
        .ok()
        .as_deref()
        == Some("1")
    {
        return Err(EnclaveError::ForcedFailure);
    }
    Ok(())
}

/// Creates a per-agent wrapping key and returns the stable key reference string.
pub(crate) fn create_wrapping_key(agent_name: &str) -> Result<String, EnclaveError> {
    maybe_force_failure()?;
    let key_ref = format!("com.icebox.identity.{agent_name}.wrapping-key");
    if std::env::var("ICEBOX_TEST_FAKE_ENCLAVE").ok().as_deref() == Some("1") {
        return Ok(key_ref);
    }
    platform::create_wrapping_key(&key_ref)?;
    Ok(key_ref)
}

#[cfg(target_os = "macos")]
mod platform {
    use super::EnclaveError;
    use core_foundation::base::TCFType;
    use core_foundation::boolean::CFBoolean;
    use core_foundation::data::CFData;
    use core_foundation::dictionary::CFDictionary;
    use core_foundation::error::CFError;
    use core_foundation::number::CFNumber;
    use core_foundation::string::CFString;
    use security_framework_sys::base::errSecDuplicateItem;
    use security_framework_sys::item::{
        kSecAttrIsPermanent, kSecAttrKeySizeInBits, kSecAttrKeyType,
        kSecAttrKeyTypeECSECPrimeRandom, kSecAttrLabel, kSecAttrTokenID,
        kSecAttrTokenIDSecureEnclave, kSecPrivateKeyAttrs,
    };
    use security_framework_sys::key::SecKeyCreateRandomKey;
    use std::ptr;

    pub(crate) fn create_wrapping_key(key_ref: &str) -> Result<(), EnclaveError> {
        let key_type_attr = unsafe { CFString::wrap_under_get_rule(kSecAttrKeyType) };
        let key_type_ec = unsafe { CFString::wrap_under_get_rule(kSecAttrKeyTypeECSECPrimeRandom) };
        let key_size_attr = unsafe { CFString::wrap_under_get_rule(kSecAttrKeySizeInBits) };
        let token_attr = unsafe { CFString::wrap_under_get_rule(kSecAttrTokenID) };
        let token_enclave = unsafe { CFString::wrap_under_get_rule(kSecAttrTokenIDSecureEnclave) };
        let private_attrs_attr = unsafe { CFString::wrap_under_get_rule(kSecPrivateKeyAttrs) };
        let is_permanent_attr = unsafe { CFString::wrap_under_get_rule(kSecAttrIsPermanent) };
        let label_attr = unsafe { CFString::wrap_under_get_rule(kSecAttrLabel) };

        let key_size = CFNumber::from(256_i32);
        let label_data = CFData::from_buffer(key_ref.as_bytes());
        let true_value = CFBoolean::true_value();

        let private_attrs = CFDictionary::from_CFType_pairs(&[
            (is_permanent_attr.clone(), true_value.as_CFType()),
            (label_attr.clone(), label_data.as_CFType()),
        ]);
        let attrs = CFDictionary::from_CFType_pairs(&[
            (key_type_attr.clone(), key_type_ec.as_CFType()),
            (key_size_attr.clone(), key_size.as_CFType()),
            (token_attr.clone(), token_enclave.as_CFType()),
            (private_attrs_attr.clone(), private_attrs.as_CFType()),
        ]);

        let mut error_ref = ptr::null_mut();
        let key_ref_ptr =
            unsafe { SecKeyCreateRandomKey(attrs.as_concrete_TypeRef(), &mut error_ref) };
        if !key_ref_ptr.is_null() {
            unsafe {
                core_foundation::base::CFRelease(key_ref_ptr.cast());
            }
            return Ok(());
        }

        if !error_ref.is_null() {
            let error = unsafe { CFError::wrap_under_create_rule(error_ref) };
            if error.code() == errSecDuplicateItem as isize {
                return Ok(());
            }
            if error.code() == -26276 {
                return Err(EnclaveError::Platform(
                    "secure enclave key generation failed (OSStatus -26276): check supported hardware and code-signing entitlements".to_string(),
                ));
            }
            let message = error.description().to_string();
            return Err(EnclaveError::Platform(format!(
                "secure enclave key creation failed: {message}"
            )));
        }

        Err(EnclaveError::Platform(
            "secure enclave key creation failed".to_string(),
        ))
    }
}

#[cfg(not(target_os = "macos"))]
mod platform {
    use super::EnclaveError;

    pub(crate) fn create_wrapping_key(_key_ref: &str) -> Result<(), EnclaveError> {
        // Non-macOS builds intentionally fail closed for local-enclave registration.
        // This stub exists for CI/compilation coverage only; Secure Enclave flow is macOS-only.
        Err(EnclaveError::Platform(
            "secure enclave is unavailable on this platform".to_string(),
        ))
    }
}
