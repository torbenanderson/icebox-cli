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

fn maybe_force_wrap_failure() -> Result<(), EnclaveError> {
    if std::env::var("ICEBOX_TEST_FORCE_ENCLAVE_WRAP_ERROR")
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

/// Wraps raw key bytes with the per-agent wrapping key and returns ciphertext bytes.
pub(crate) fn wrap_private_key(
    wrapping_key_ref: &str,
    raw_private_key: &[u8],
) -> Result<Vec<u8>, EnclaveError> {
    maybe_force_wrap_failure()?;
    if std::env::var("ICEBOX_TEST_FAKE_ENCLAVE").ok().as_deref() == Some("1") {
        let mut out = b"fake-enclave-wrap-v1:".to_vec();
        out.extend(raw_private_key.iter().map(|byte| byte ^ 0xAA));
        return Ok(out);
    }
    platform::wrap_private_key(wrapping_key_ref, raw_private_key)
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
    use security_framework_sys::base::{errSecDuplicateItem, errSecItemNotFound};
    use security_framework_sys::item::{
        kSecAttrIsPermanent, kSecAttrKeySizeInBits, kSecAttrKeyType,
        kSecAttrKeyTypeECSECPrimeRandom, kSecAttrLabel, kSecAttrTokenID,
        kSecAttrTokenIDSecureEnclave, kSecClass, kSecClassKey, kSecPrivateKeyAttrs, kSecReturnRef,
    };
    use security_framework_sys::key::{
        Algorithm, SecKeyCopyPublicKey, SecKeyCreateEncryptedData, SecKeyCreateRandomKey,
        SecKeyIsAlgorithmSupported, kSecKeyOperationTypeEncrypt,
    };
    use security_framework_sys::keychain_item::SecItemCopyMatching;
    use std::ffi::c_void;
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
        let label = CFString::new(key_ref);
        let true_value = CFBoolean::true_value();

        let private_attrs = CFDictionary::from_CFType_pairs(&[
            (is_permanent_attr.clone(), true_value.as_CFType()),
            (label_attr.clone(), label.as_CFType()),
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

    fn find_private_key(
        key_ref: &str,
    ) -> Result<security_framework_sys::base::SecKeyRef, EnclaveError> {
        let class_attr = unsafe { CFString::wrap_under_get_rule(kSecClass) };
        let class_key = unsafe { CFString::wrap_under_get_rule(kSecClassKey) };
        let label_attr = unsafe { CFString::wrap_under_get_rule(kSecAttrLabel) };
        let return_ref_attr = unsafe { CFString::wrap_under_get_rule(kSecReturnRef) };
        let key_type_attr = unsafe { CFString::wrap_under_get_rule(kSecAttrKeyType) };
        let key_type_ec = unsafe { CFString::wrap_under_get_rule(kSecAttrKeyTypeECSECPrimeRandom) };
        let label = CFString::new(key_ref);
        let true_value = CFBoolean::true_value();

        let query = CFDictionary::from_CFType_pairs(&[
            (class_attr, class_key.as_CFType()),
            (label_attr, label.as_CFType()),
            (return_ref_attr, true_value.as_CFType()),
            (key_type_attr, key_type_ec.as_CFType()),
        ]);

        let mut result_ref: *const c_void = ptr::null();
        let status = unsafe { SecItemCopyMatching(query.as_concrete_TypeRef(), &mut result_ref) };
        if status == errSecItemNotFound {
            return Err(EnclaveError::Platform(
                "secure enclave key lookup failed: wrapping key not found".to_string(),
            ));
        }
        if status != 0 {
            return Err(EnclaveError::Platform(format!(
                "secure enclave key lookup failed (OSStatus {status})"
            )));
        }
        if result_ref.is_null() {
            return Err(EnclaveError::Platform(
                "secure enclave key lookup failed: empty key reference".to_string(),
            ));
        }

        Ok(result_ref as *mut _)
    }

    pub(crate) fn wrap_private_key(
        wrapping_key_ref: &str,
        raw_private_key: &[u8],
    ) -> Result<Vec<u8>, EnclaveError> {
        let private_key = find_private_key(wrapping_key_ref)?;
        let public_key = unsafe { SecKeyCopyPublicKey(private_key) };
        if public_key.is_null() {
            unsafe { core_foundation::base::CFRelease(private_key.cast()) };
            return Err(EnclaveError::Platform(
                "secure enclave wrap failed: missing public key".to_string(),
            ));
        }

        let algorithm = security_framework_sys::key::SecKeyAlgorithm::from(
            Algorithm::ECIESEncryptionStandardX963SHA256AESGCM,
        );
        let supported = unsafe {
            SecKeyIsAlgorithmSupported(public_key, kSecKeyOperationTypeEncrypt, algorithm)
        };
        if supported == 0 {
            unsafe {
                core_foundation::base::CFRelease(public_key.cast());
                core_foundation::base::CFRelease(private_key.cast());
            }
            return Err(EnclaveError::Platform(
                "secure enclave wrap failed: encryption algorithm unsupported".to_string(),
            ));
        }

        let plaintext = CFData::from_buffer(raw_private_key);
        let mut error_ref = ptr::null_mut();
        let encrypted = unsafe {
            SecKeyCreateEncryptedData(
                public_key,
                algorithm,
                plaintext.as_concrete_TypeRef(),
                &mut error_ref,
            )
        };

        unsafe {
            core_foundation::base::CFRelease(public_key.cast());
            core_foundation::base::CFRelease(private_key.cast());
        }

        if encrypted.is_null() {
            if !error_ref.is_null() {
                let error = unsafe { CFError::wrap_under_create_rule(error_ref) };
                return Err(EnclaveError::Platform(format!(
                    "secure enclave wrap failed: {}",
                    error.description()
                )));
            }
            return Err(EnclaveError::Platform(
                "secure enclave wrap failed".to_string(),
            ));
        }

        let wrapped = unsafe { CFData::wrap_under_create_rule(encrypted) };
        Ok(wrapped.bytes().to_vec())
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

    pub(crate) fn wrap_private_key(
        _wrapping_key_ref: &str,
        _raw_private_key: &[u8],
    ) -> Result<Vec<u8>, EnclaveError> {
        Err(EnclaveError::Platform(
            "secure enclave is unavailable on this platform".to_string(),
        ))
    }
}
