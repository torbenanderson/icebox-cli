//! Vault creation and sealed-entry persistence.

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use crypto_box::PublicKey as X25519PublicKey;
use ed25519_dalek::VerifyingKey;
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

const VAULT_VERSION: u16 = 1;
const VAULT_FORMAT: &str = "icebox.vault.legacy-v1";
const IDENTITY_PUBLIC_KEY_LENGTH: usize = 32;

/// Logical handle to an identity-scoped vault.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VaultRef {
    /// Identifier for the owning identity.
    pub identity_id: String,
}

impl VaultRef {
    /// Creates a vault reference for a non-empty identity id.
    pub fn for_identity(identity_id: &str) -> Result<Self, &'static str> {
        if identity_id.trim().is_empty() {
            return Err("identity id cannot be empty");
        }

        Ok(Self {
            identity_id: identity_id.to_owned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VaultEntry {
    pub service: String,
    #[serde(rename = "sealedBlob")]
    pub sealed_blob: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VaultFile {
    #[serde(default = "default_vault_format")]
    pub format: String,
    pub version: u16,
    pub entries: Vec<VaultEntry>,
}

fn default_vault_format() -> String {
    VAULT_FORMAT.to_owned()
}

impl Default for VaultFile {
    fn default() -> Self {
        Self {
            format: default_vault_format(),
            version: VAULT_VERSION,
            entries: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum VaultError {
    MissingHomeDir,
    ServiceNameEmpty,
    SecretValueEmpty,
    MissingActiveAgent,
    MissingIdentityPublicKey {
        path: PathBuf,
    },
    InvalidIdentityPublicKey {
        path: PathBuf,
    },
    InvalidIdentityPublicKeyLength {
        path: PathBuf,
        len: usize,
    },
    InvalidVaultJson {
        path: PathBuf,
        source: serde_json::Error,
    },
    Serialize {
        op: &'static str,
        source: serde_json::Error,
    },
    Io {
        op: &'static str,
        source: std::io::Error,
    },
    Crypto {
        op: &'static str,
        source: String,
    },
}

impl Display for VaultError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingHomeDir => f.write_str("could not resolve home directory"),
            Self::ServiceNameEmpty => f.write_str("service name cannot be empty"),
            Self::SecretValueEmpty => f.write_str("secret value cannot be empty"),
            Self::MissingActiveAgent => f.write_str(
                "No active agent configured. Register an agent first with `icebox-cli register-agent <name>`.",
            ),
            Self::MissingIdentityPublicKey { path } => {
                write!(f, "missing identity public key: {}", path.display())
            }
            Self::InvalidIdentityPublicKey { path } => {
                write!(f, "invalid identity public key bytes: {}", path.display())
            }
            Self::InvalidIdentityPublicKeyLength { path, len } => write!(
                f,
                "invalid identity public key length ({len}) at {}",
                path.display()
            ),
            Self::InvalidVaultJson { path, source } => {
                write!(f, "failed to parse {}: {source}", path.display())
            }
            Self::Serialize { op, source } => write!(f, "{op}: {source}"),
            Self::Io { op, source } => write!(f, "{op}: {source}"),
            Self::Crypto { op, source } => write!(f, "{op}: {source}"),
        }
    }
}

impl Error for VaultError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidVaultJson { source, .. } => Some(source),
            Self::Serialize { source, .. } => Some(source),
            Self::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

fn io_err(op: &'static str, source: std::io::Error) -> VaultError {
    VaultError::Io { op, source }
}

fn serialize_err(op: &'static str, source: serde_json::Error) -> VaultError {
    VaultError::Serialize { op, source }
}

fn crypto_err(op: &'static str, source: impl ToString) -> VaultError {
    VaultError::Crypto {
        op,
        source: source.to_string(),
    }
}

fn resolve_icebox_home() -> Result<PathBuf, VaultError> {
    if let Ok(override_home) = std::env::var("ICEBOX_HOME") {
        return Ok(PathBuf::from(override_home));
    }
    let home = std::env::var_os("HOME").ok_or(VaultError::MissingHomeDir)?;
    Ok(PathBuf::from(home).join(".icebox"))
}

fn load_active_agent(home: &Path) -> Result<crate::config::AgentRecord, VaultError> {
    let config =
        crate::config::load_or_default_with_repair(home).map_err(|source| VaultError::Io {
            op: "failed to load config.json",
            source: std::io::Error::other(source.to_string()),
        })?;
    let active_id = config
        .active_agent_id
        .ok_or(VaultError::MissingActiveAgent)?;
    config
        .agents
        .into_iter()
        .find(|agent| agent.agent_id == active_id)
        .ok_or(VaultError::MissingActiveAgent)
}

fn read_identity_public_key(path: &Path) -> Result<[u8; IDENTITY_PUBLIC_KEY_LENGTH], VaultError> {
    let bytes = fs::read(path).map_err(|err| {
        if err.kind() == std::io::ErrorKind::NotFound {
            VaultError::MissingIdentityPublicKey {
                path: path.to_path_buf(),
            }
        } else {
            io_err("failed to read identity.pub", err)
        }
    })?;
    if bytes.len() != IDENTITY_PUBLIC_KEY_LENGTH {
        return Err(VaultError::InvalidIdentityPublicKeyLength {
            path: path.to_path_buf(),
            len: bytes.len(),
        });
    }
    let mut out = [0u8; IDENTITY_PUBLIC_KEY_LENGTH];
    out.copy_from_slice(&bytes);
    Ok(out)
}

fn ed25519_public_to_x25519(public_key: &[u8; 32]) -> Result<X25519PublicKey, VaultError> {
    let verifying = VerifyingKey::from_bytes(public_key)
        .map_err(|_| crypto_err("failed to parse Ed25519 public key", "invalid key"))?;
    Ok(X25519PublicKey::from(verifying.to_montgomery().to_bytes()))
}

fn seal_secret_for_identity(
    identity_public_key: &[u8; 32],
    secret: &[u8],
) -> Result<Vec<u8>, VaultError> {
    let recipient_public = ed25519_public_to_x25519(identity_public_key)?;
    recipient_public
        .seal(&mut OsRng, secret)
        .map_err(|err| crypto_err("failed to seal secret", err))
}

fn load_or_create_vault(path: &Path) -> Result<VaultFile, VaultError> {
    match fs::read(path) {
        Ok(bytes) => serde_json::from_slice::<VaultFile>(&bytes).map_err(|source| {
            VaultError::InvalidVaultJson {
                path: path.to_path_buf(),
                source,
            }
        }),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(VaultFile::default()),
        Err(err) => Err(io_err("failed to read vault.enc", err)),
    }
}

fn save_vault(path: &Path, vault: &VaultFile) -> Result<(), VaultError> {
    let payload = serde_json::to_vec_pretty(vault)
        .map_err(|source| serialize_err("failed to serialize vault.enc", source))?;
    let tmp_path = path.with_extension("enc.tmp");
    let mut out = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&tmp_path)
        .map_err(|err| io_err("failed to create vault.enc.tmp", err))?;
    out.write_all(&payload)
        .map_err(|err| io_err("failed to write vault.enc.tmp", err))?;
    out.flush()
        .map_err(|err| io_err("failed to flush vault.enc.tmp", err))?;
    fs::rename(&tmp_path, path).map_err(|err| io_err("failed to replace vault.enc", err))?;
    Ok(())
}

pub fn add_secret_to_active_agent(service: &str, secret_value: &str) -> Result<(), VaultError> {
    let trimmed_service = service.trim();
    if trimmed_service.is_empty() {
        return Err(VaultError::ServiceNameEmpty);
    }
    if secret_value.is_empty() {
        return Err(VaultError::SecretValueEmpty);
    }

    let home = resolve_icebox_home()?;
    let active_agent = load_active_agent(&home)?;
    let agent_dir = home.join("identities").join(active_agent.name);
    let identity_pub_path = agent_dir.join("identity.pub");
    let identity_public_key = read_identity_public_key(&identity_pub_path)?;

    let sealed = seal_secret_for_identity(&identity_public_key, secret_value.as_bytes())?;
    let sealed_blob = BASE64_STANDARD.encode(sealed);

    let vault_path = agent_dir.join("vault.enc");
    let mut vault = load_or_create_vault(&vault_path)?;
    if let Some(existing) = vault
        .entries
        .iter_mut()
        .find(|entry| entry.service == trimmed_service)
    {
        existing.sealed_blob = sealed_blob;
    } else {
        vault.entries.push(VaultEntry {
            service: trimmed_service.to_owned(),
            sealed_blob,
        });
    }

    save_vault(&vault_path, &vault)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_box::SecretKey as X25519SecretKey;
    use ed25519_dalek::SigningKey;

    fn open_sealed_blob_for_test(
        identity_signing_key: &SigningKey,
        blob: &[u8],
    ) -> Result<Vec<u8>, VaultError> {
        let x25519_secret = X25519SecretKey::from(identity_signing_key.to_scalar_bytes());
        x25519_secret
            .unseal(blob)
            .map_err(|err| crypto_err("failed to open sealed blob", err))
    }

    #[test]
    fn seal_round_trip_with_identity_keypair() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let public_key = signing_key.verifying_key().to_bytes();
        let plaintext = b"sk-test-secret";

        let sealed = seal_secret_for_identity(&public_key, plaintext).expect("seal should succeed");
        let opened = open_sealed_blob_for_test(&signing_key, &sealed).expect("open should succeed");
        assert_eq!(opened, plaintext);
    }

    #[test]
    fn open_fails_when_ciphertext_is_tampered() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let public_key = signing_key.verifying_key().to_bytes();
        let plaintext = b"sk-test-secret";

        let mut sealed =
            seal_secret_for_identity(&public_key, plaintext).expect("seal should succeed");
        let last = sealed
            .len()
            .checked_sub(1)
            .expect("sealed payload should be non-empty");
        sealed[last] ^= 0x01;

        let err = open_sealed_blob_for_test(&signing_key, &sealed)
            .expect_err("tampered ciphertext should fail");
        assert!(
            err.to_string().contains("failed to open sealed blob"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn vault_deserialize_backfills_missing_format_for_legacy_files() {
        let parsed: VaultFile =
            serde_json::from_str(r#"{"version":1,"entries":[]}"#).expect("vault json should parse");
        assert_eq!(parsed.format, VAULT_FORMAT);
        assert_eq!(parsed.version, 1);
        assert!(parsed.entries.is_empty());
    }
}
