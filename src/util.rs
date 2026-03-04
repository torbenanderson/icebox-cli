//! Shared utility helpers for deterministic formatting/id generation.

use rand_core::{OsRng, RngCore};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::path::PathBuf;

/// Encodes bytes to lowercase hexadecimal.
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

/// Generates a random UUID-like agent identifier (MVP local format).
pub fn generate_agent_id() -> String {
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

/// Home-path resolution failures for ICEBOX_HOME.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolveIceboxHomeError {
    /// HOME is unavailable and ICEBOX_HOME was not provided.
    MissingHomeDir,
}

impl Display for ResolveIceboxHomeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingHomeDir => f.write_str("could not resolve home directory"),
        }
    }
}

impl Error for ResolveIceboxHomeError {}

/// Resolves the effective Icebox home path.
pub fn resolve_icebox_home() -> Result<PathBuf, ResolveIceboxHomeError> {
    if let Ok(override_home) = std::env::var("ICEBOX_HOME") {
        return Ok(PathBuf::from(override_home));
    }

    let home = std::env::var_os("HOME").ok_or(ResolveIceboxHomeError::MissingHomeDir)?;
    Ok(PathBuf::from(home).join(".icebox"))
}

/// Builds an identity directory path under ICEBOX_HOME.
pub fn agent_dir(home: &Path, name: &str) -> PathBuf {
    home.join("identities").join(name)
}

/// Builds the identity public-key path.
pub fn identity_pub_path(agent_dir: &Path) -> PathBuf {
    agent_dir.join("identity.pub")
}

/// Builds the identity vault path.
pub fn vault_path(agent_dir: &Path) -> PathBuf {
    agent_dir.join("vault.enc")
}
