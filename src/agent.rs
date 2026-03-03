//! Agent-facing identity adapters.
//!
//! Command naming can remain "agent" while internal types stay identity-neutral.

use ed25519_dalek::SigningKey;
use rand_core::{OsRng, RngCore};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Canonical identity name used by registration flows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityName(String);

impl IdentityName {
    /// Parses and validates an identity name.
    pub fn parse(raw: &str) -> Result<Self, IdentityNameError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(IdentityNameError::Empty);
        }
        if trimmed.len() < 3 || trimmed.len() > 32 {
            return Err(IdentityNameError::InvalidFormat);
        }
        if trimmed.starts_with('-') {
            return Err(IdentityNameError::InvalidFormat);
        }
        let valid = trimmed
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-');
        if !valid {
            return Err(IdentityNameError::InvalidFormat);
        }

        Ok(Self(trimmed.to_owned()))
    }

    /// Returns the stored identity name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Validation errors for [`IdentityName`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentityNameError {
    /// Name is empty after trimming whitespace.
    Empty,
    /// Name does not match canonical policy.
    InvalidFormat,
}

impl Display for IdentityNameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str("identity name cannot be empty"),
            Self::InvalidFormat => f.write_str("identity name must match [a-z0-9-]{3,32}"),
        }
    }
}

impl Error for IdentityNameError {}

/// Registration errors for `register-agent`.
#[derive(Debug)]
pub enum RegisterAgentError {
    /// Identity name failed validation.
    InvalidName(IdentityNameError),
    /// Home directory could not be resolved.
    MissingHomeDir,
    /// Filesystem operation failed.
    Io {
        /// User-facing operation label.
        op: &'static str,
        /// Source I/O error.
        source: std::io::Error,
    },
    /// Secure enclave operation failed.
    Enclave {
        /// User-facing operation label.
        op: &'static str,
        /// Source enclave error detail.
        source: String,
    },
}

impl Display for RegisterAgentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidName(err) => write!(f, "{err}"),
            Self::MissingHomeDir => f.write_str("could not resolve home directory"),
            Self::Io { op, source } => write!(f, "{op}: {source}"),
            Self::Enclave { op, source } => write!(f, "{op}: {source}"),
        }
    }
}

impl Error for RegisterAgentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidName(err) => Some(err),
            Self::MissingHomeDir => None,
            Self::Io { source, .. } => Some(source),
            Self::Enclave { .. } => None,
        }
    }
}

fn io_err(op: &'static str, source: std::io::Error) -> RegisterAgentError {
    RegisterAgentError::Io { op, source }
}

fn enclave_err(op: &'static str, source: crate::enclave::EnclaveError) -> RegisterAgentError {
    RegisterAgentError::Enclave {
        op,
        source: source.to_string(),
    }
}

fn config_err(op: &'static str, source: crate::config::ConfigError) -> RegisterAgentError {
    RegisterAgentError::Io {
        op,
        source: std::io::Error::other(source.to_string()),
    }
}

fn cleanup_file_if_created(path: &Path, created: bool) -> Result<(), std::io::Error> {
    if !created {
        return Ok(());
    }
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err),
    }
}

fn force_key_enc_persist_failure() -> Result<(), RegisterAgentError> {
    if std::env::var("ICEBOX_TEST_FORCE_KEY_ENC_PERSIST_ERROR")
        .ok()
        .as_deref()
        == Some("1")
    {
        return Err(io_err(
            "failed to create key.enc",
            std::io::Error::other("forced key.enc persistence failure"),
        ));
    }
    Ok(())
}

fn resolve_icebox_home() -> Result<PathBuf, RegisterAgentError> {
    if let Ok(override_home) = std::env::var("ICEBOX_HOME") {
        return Ok(PathBuf::from(override_home));
    }

    let home = std::env::var_os("HOME").ok_or(RegisterAgentError::MissingHomeDir)?;
    Ok(PathBuf::from(home).join(".icebox"))
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn generate_agent_id() -> String {
    let mut random = [0u8; 16];
    OsRng.fill_bytes(&mut random);
    format!(
        "{}-{}-{}-{}-{}",
        bytes_to_hex(&random[0..4]),
        bytes_to_hex(&random[4..6]),
        bytes_to_hex(&random[6..8]),
        bytes_to_hex(&random[8..10]),
        bytes_to_hex(&random[10..16]),
    )
}

fn did_from_public_key(public_key: &[u8; 32]) -> String {
    format!("did:key:ed25519-raw:{}", bytes_to_hex(public_key))
}

/// Registers an agent by generating a new Ed25519 keypair and writing identity artifacts.
///
/// For E2-01, this writes only `identity.pub` under
/// `~/.icebox/identities/<name>/` (or `$ICEBOX_HOME/identities/<name>/`).
pub fn register_agent(raw_name: &str) -> Result<(), RegisterAgentError> {
    let name = IdentityName::parse(raw_name).map_err(RegisterAgentError::InvalidName)?;
    let home = resolve_icebox_home()?;
    let config_home = home.clone();
    let _existing_config = crate::config::load_or_default_with_repair(&config_home)
        .map_err(|err| config_err("failed to load config.json", err))?;

    let agent_dir = home.join("identities").join(name.as_str());

    let key_ref_path = agent_dir.join("enclave.keyref");
    let key_enc_path = agent_dir.join("key.enc");
    let identity_pub_path = agent_dir.join("identity.pub");

    let mut key_ref_created = false;
    let mut key_enc_created = false;
    let mut identity_pub_created = false;

    let result = (|| -> Result<(), RegisterAgentError> {
        fs::create_dir_all(&agent_dir)
            .map_err(|err| io_err("failed to create agent directory", err))?;

        let wrapping_key_ref = crate::enclave::create_wrapping_key(name.as_str())
            .map_err(|err| enclave_err("failed to create enclave wrapping key", err))?;
        let mut key_ref_out = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&key_ref_path)
            .map_err(|err| io_err("failed to create enclave.keyref", err))?;
        key_ref_created = true;
        key_ref_out
            .write_all(wrapping_key_ref.as_bytes())
            .map_err(|err| io_err("failed to write enclave.keyref", err))?;

        let signing_key = SigningKey::generate(&mut OsRng);
        let wrapped_private_key =
            crate::enclave::wrap_private_key(&wrapping_key_ref, &signing_key.to_bytes())
                .map_err(|err| enclave_err("failed to wrap private key", err))?;

        force_key_enc_persist_failure()?;
        let mut key_enc_out = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&key_enc_path)
            .map_err(|err| io_err("failed to create key.enc", err))?;
        key_enc_created = true;
        key_enc_out
            .write_all(&wrapped_private_key)
            .map_err(|err| io_err("failed to write key.enc", err))?;

        let public_key = signing_key.verifying_key();
        let public_key_bytes = public_key.to_bytes();
        let mut out = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&identity_pub_path)
            .map_err(|err| io_err("failed to create identity.pub", err))?;
        identity_pub_created = true;
        out.write_all(&public_key_bytes)
            .map_err(|err| io_err("failed to write identity.pub", err))?;

        let agent_record = crate::config::AgentRecord {
            agent_id: generate_agent_id(),
            name: name.as_str().to_owned(),
            did: did_from_public_key(&public_key_bytes),
        };
        crate::config::append_agent_and_set_active(&config_home, agent_record)
            .map_err(|err| config_err("failed to persist config.json", err))?;

        Ok(())
    })();

    if let Err(err) = result {
        cleanup_file_if_created(&identity_pub_path, identity_pub_created)
            .map_err(|cleanup_err| io_err("failed to clean identity.pub", cleanup_err))?;
        cleanup_file_if_created(&key_enc_path, key_enc_created)
            .map_err(|cleanup_err| io_err("failed to clean key.enc", cleanup_err))?;
        cleanup_file_if_created(&key_ref_path, key_ref_created)
            .map_err(|cleanup_err| io_err("failed to clean enclave.keyref", cleanup_err))?;
        return Err(err);
    }

    Ok(())
}
