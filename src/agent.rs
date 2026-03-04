//! Agent-facing identity adapters.
//!
//! Command naming can remain "agent" while internal types stay identity-neutral.

use ed25519_dalek::SigningKey;
use rand_core::OsRng;
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
    /// Agent name already exists in config registry.
    DuplicateName {
        /// Existing canonical agent name.
        name: String,
    },
    /// Config contains duplicate agent names.
    DuplicateRegistryNames {
        /// Resolved config path.
        path: PathBuf,
    },
    /// Config file is invalid and must be repaired.
    InvalidConfig {
        /// Resolved config path.
        path: PathBuf,
    },
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
            Self::InvalidName(_) => {
                f.write_str("Invalid agent name. Use [a-z0-9-]{3,32} and do not start with '-'.")
            }
            Self::MissingHomeDir => f.write_str("could not resolve home directory"),
            Self::DuplicateName { name } => write!(
                f,
                "Agent {name} already exists. Choose a different name or remove the existing agent."
            ),
            Self::DuplicateRegistryNames { path } => write!(
                f,
                "Config has duplicate agent names. Resolve duplicates in {} and retry.",
                path.display()
            ),
            Self::InvalidConfig { path } => write!(
                f,
                "Config is invalid. Fix {} or reinitialize.",
                path.display()
            ),
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
            Self::DuplicateName { .. } => None,
            Self::DuplicateRegistryNames { .. } => None,
            Self::InvalidConfig { .. } => None,
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

impl RegisterAgentError {
    fn from_config_error(
        op: &'static str,
        source: crate::config::ConfigError,
        config_path: &Path,
    ) -> Self {
        match source {
            crate::config::ConfigError::DuplicateAgentNames => Self::DuplicateRegistryNames {
                path: config_path.to_path_buf(),
            },
            crate::config::ConfigError::Parse { .. }
            | crate::config::ConfigError::Serialize { .. }
            | crate::config::ConfigError::InvalidAgentName { .. } => Self::InvalidConfig {
                path: config_path.to_path_buf(),
            },
            crate::config::ConfigError::Io { source, .. } => Self::Io { op, source },
        }
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

#[derive(Debug)]
struct RegistrationCleanup {
    key_ref_path: PathBuf,
    key_enc_path: PathBuf,
    identity_pub_path: PathBuf,
    key_ref_created: bool,
    key_enc_created: bool,
    identity_pub_created: bool,
}

impl RegistrationCleanup {
    fn new(key_ref_path: PathBuf, key_enc_path: PathBuf, identity_pub_path: PathBuf) -> Self {
        Self {
            key_ref_path,
            key_enc_path,
            identity_pub_path,
            key_ref_created: false,
            key_enc_created: false,
            identity_pub_created: false,
        }
    }

    fn cleanup_on_error(&self) -> Result<(), RegisterAgentError> {
        cleanup_file_if_created(&self.identity_pub_path, self.identity_pub_created)
            .map_err(|cleanup_err| io_err("failed to clean identity.pub", cleanup_err))?;
        cleanup_file_if_created(&self.key_enc_path, self.key_enc_created)
            .map_err(|cleanup_err| io_err("failed to clean key.enc", cleanup_err))?;
        cleanup_file_if_created(&self.key_ref_path, self.key_ref_created)
            .map_err(|cleanup_err| io_err("failed to clean enclave.keyref", cleanup_err))?;
        Ok(())
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

fn write_new_file(
    path: &Path,
    bytes: &[u8],
    create_op: &'static str,
    write_op: &'static str,
) -> Result<(), RegisterAgentError> {
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|err| io_err(create_op, err))?;
    out.write_all(bytes).map_err(|err| io_err(write_op, err))
}

fn create_wrapping_key_ref(
    name: &IdentityName,
    cleanup: &mut RegistrationCleanup,
) -> Result<String, RegisterAgentError> {
    let wrapping_key_ref = crate::enclave::create_wrapping_key(name.as_str())
        .map_err(|err| enclave_err("failed to create enclave wrapping key", err))?;
    write_new_file(
        &cleanup.key_ref_path,
        wrapping_key_ref.as_bytes(),
        "failed to create enclave.keyref",
        "failed to write enclave.keyref",
    )?;
    cleanup.key_ref_created = true;
    Ok(wrapping_key_ref)
}

fn create_wrapped_private_key(
    wrapping_key_ref: &str,
) -> Result<(SigningKey, Vec<u8>), RegisterAgentError> {
    let signing_key = SigningKey::generate(&mut OsRng);
    let wrapped_private_key =
        crate::enclave::wrap_private_key(wrapping_key_ref, &signing_key.to_bytes())
            .map_err(|err| enclave_err("failed to wrap private key", err))?;
    Ok((signing_key, wrapped_private_key))
}

fn persist_wrapped_private_key(
    wrapped_private_key: &[u8],
    cleanup: &mut RegistrationCleanup,
) -> Result<(), RegisterAgentError> {
    force_key_enc_persist_failure()?;
    write_new_file(
        &cleanup.key_enc_path,
        wrapped_private_key,
        "failed to create key.enc",
        "failed to write key.enc",
    )?;
    cleanup.key_enc_created = true;
    Ok(())
}

fn persist_identity_public_key(
    signing_key: &SigningKey,
    cleanup: &mut RegistrationCleanup,
) -> Result<[u8; 32], RegisterAgentError> {
    let public_key_bytes = signing_key.verifying_key().to_bytes();
    write_new_file(
        &cleanup.identity_pub_path,
        &public_key_bytes,
        "failed to create identity.pub",
        "failed to write identity.pub",
    )?;
    cleanup.identity_pub_created = true;
    Ok(public_key_bytes)
}

fn persist_agent_registry_entry(
    config_home: &Path,
    config_path: &Path,
    config: &mut crate::config::RuntimeConfig,
    name: &IdentityName,
    public_key_bytes: &[u8; 32],
) -> Result<(), RegisterAgentError> {
    let agent_record = crate::config::AgentRecord {
        agent_id: crate::util::generate_agent_id(),
        name: name.as_str().to_owned(),
        did: crate::did::did_from_public_key(public_key_bytes),
    };
    crate::config::append_agent_and_set_active_in_memory(config_home, config, agent_record).map_err(
        |err| {
            RegisterAgentError::from_config_error("failed to persist config.json", err, config_path)
        },
    )
}

/// Registers an agent by generating a new Ed25519 keypair and writing identity artifacts.
///
/// For E2-01, this writes only `identity.pub` under
/// `~/.icebox/identities/<name>/` (or `$ICEBOX_HOME/identities/<name>/`).
pub fn register_agent(raw_name: &str) -> Result<(), RegisterAgentError> {
    let name = IdentityName::parse(raw_name).map_err(RegisterAgentError::InvalidName)?;
    let home =
        crate::util::resolve_icebox_home().map_err(|_| RegisterAgentError::MissingHomeDir)?;
    let config_home = home.clone();
    let config_path = config_home.join("config.json");
    let mut config = crate::config::load_or_default_with_repair(&config_home).map_err(|err| {
        RegisterAgentError::from_config_error("failed to load config.json", err, &config_path)
    })?;
    let exists =
        crate::config::has_agent_name_in_config(&config, name.as_str()).map_err(|err| {
            RegisterAgentError::from_config_error("failed to load config.json", err, &config_path)
        })?;
    if exists {
        return Err(RegisterAgentError::DuplicateName {
            name: name.as_str().to_owned(),
        });
    }

    let agent_dir = home.join("identities").join(name.as_str());

    let mut cleanup = RegistrationCleanup::new(
        agent_dir.join("enclave.keyref"),
        agent_dir.join("key.enc"),
        agent_dir.join("identity.pub"),
    );

    let result = (|| -> Result<(), RegisterAgentError> {
        fs::create_dir_all(&agent_dir)
            .map_err(|err| io_err("failed to create agent directory", err))?;

        let wrapping_key_ref = create_wrapping_key_ref(&name, &mut cleanup)?;
        let (signing_key, wrapped_private_key) = create_wrapped_private_key(&wrapping_key_ref)?;
        persist_wrapped_private_key(&wrapped_private_key, &mut cleanup)?;
        let public_key_bytes = persist_identity_public_key(&signing_key, &mut cleanup)?;
        persist_agent_registry_entry(
            &config_home,
            &config_path,
            &mut config,
            &name,
            &public_key_bytes,
        )?;
        Ok(())
    })();

    if let Err(err) = result {
        cleanup.cleanup_on_error()?;
        return Err(err);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_error_duplicate_maps_to_duplicate_registry_names() {
        let err = RegisterAgentError::from_config_error(
            "failed to persist config.json",
            crate::config::ConfigError::DuplicateAgentNames,
            Path::new("/tmp/config.json"),
        );
        assert!(matches!(
            err,
            RegisterAgentError::DuplicateRegistryNames { .. }
        ));
    }

    #[test]
    fn config_error_parse_maps_to_invalid_config() {
        let parse_err = serde_json::from_str::<serde_json::Value>("{")
            .expect_err("fixture should produce parse error");
        let err = RegisterAgentError::from_config_error(
            "failed to load config.json",
            crate::config::ConfigError::Parse { source: parse_err },
            Path::new("/tmp/config.json"),
        );
        assert!(matches!(err, RegisterAgentError::InvalidConfig { .. }));
    }

    #[test]
    fn config_error_serialize_maps_to_invalid_config() {
        struct FailSerialize;
        impl serde::Serialize for FailSerialize {
            fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                Err(serde::ser::Error::custom("intentional serialize failure"))
            }
        }

        let fake_serialize_err =
            serde_json::to_vec(&FailSerialize).expect_err("fixture should produce serialize error");
        let err = RegisterAgentError::from_config_error(
            "failed to persist config.json",
            crate::config::ConfigError::Serialize {
                source: fake_serialize_err,
            },
            Path::new("/tmp/config.json"),
        );
        assert!(matches!(err, RegisterAgentError::InvalidConfig { .. }));
    }

    #[test]
    fn config_error_invalid_name_maps_to_invalid_config() {
        let err = RegisterAgentError::from_config_error(
            "failed to load config.json",
            crate::config::ConfigError::InvalidAgentName {
                name: "Bad_Name".to_owned(),
            },
            Path::new("/tmp/config.json"),
        );
        assert!(matches!(err, RegisterAgentError::InvalidConfig { .. }));
    }

    #[test]
    fn config_error_io_maps_to_io() {
        let err = RegisterAgentError::from_config_error(
            "failed to persist config.json",
            crate::config::ConfigError::Io {
                op: "failed to replace config.json",
                source: std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied"),
            },
            Path::new("/tmp/config.json"),
        );
        assert!(matches!(err, RegisterAgentError::Io { .. }));
    }
}
