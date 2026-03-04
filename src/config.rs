//! Runtime configuration model and persistence helpers.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

fn default_schema_version() -> u16 {
    1
}

/// Registry entry for a known agent identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentRecord {
    /// Stable agent identifier used as selector key.
    #[serde(rename = "agentId")]
    pub agent_id: String,
    /// Human-readable agent name.
    pub name: String,
    /// Persisted DID compatibility field.
    pub did: String,
}

/// Runtime configuration persisted at `~/.icebox/config.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// On-disk config format version.
    #[serde(default = "default_schema_version", rename = "schemaVersion")]
    pub schema_version: u16,
    /// Active agent selector by stable identifier.
    #[serde(default, rename = "activeAgentId")]
    pub active_agent_id: Option<String>,
    /// Registry of known agents.
    #[serde(default)]
    pub agents: Vec<AgentRecord>,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            schema_version: default_schema_version(),
            active_agent_id: None,
            agents: Vec::new(),
        }
    }
}

/// Config persistence and validation failures.
#[derive(Debug)]
pub enum ConfigError {
    /// Filesystem operation failed.
    Io {
        /// User-facing operation label.
        op: &'static str,
        /// Source I/O error.
        source: std::io::Error,
    },
    /// Config JSON parsing failed.
    Parse {
        /// Source parse error.
        source: serde_json::Error,
    },
    /// Config serialization failed.
    Serialize {
        /// Source serialization error.
        source: serde_json::Error,
    },
    /// Config contains duplicate agent names.
    DuplicateAgentNames,
    /// Config contains an invalid agent name.
    InvalidAgentName {
        /// Invalid stored name.
        name: String,
    },
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { op, source } => write!(f, "{op}: {source}"),
            Self::Parse { source } => write!(f, "failed to parse config.json: {source}"),
            Self::Serialize { source } => write!(f, "failed to serialize config.json: {source}"),
            Self::DuplicateAgentNames => f.write_str("duplicate agent name in config registry"),
            Self::InvalidAgentName { name } => {
                write!(f, "invalid agent name in config registry: {name}")
            }
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Parse { source } => Some(source),
            Self::Serialize { source } => Some(source),
            Self::DuplicateAgentNames => None,
            Self::InvalidAgentName { .. } => None,
        }
    }
}

fn io_err(op: &'static str, source: std::io::Error) -> ConfigError {
    ConfigError::Io { op, source }
}

fn config_path(home: &Path) -> PathBuf {
    home.join("config.json")
}

fn canonical_agent_name(raw: &str) -> Result<String, ConfigError> {
    let parsed =
        crate::agent::IdentityName::parse(raw).map_err(|_| ConfigError::InvalidAgentName {
            name: raw.to_owned(),
        })?;
    Ok(parsed.as_str().to_owned())
}

fn canonical_agent_names(config: &RuntimeConfig) -> Result<HashSet<String>, ConfigError> {
    let mut names = HashSet::with_capacity(config.agents.len());
    for agent in &config.agents {
        let canonical = canonical_agent_name(&agent.name)?;
        names.insert(canonical);
    }
    Ok(names)
}

fn ensure_no_duplicate_agent_names(config: &RuntimeConfig) -> Result<(), ConfigError> {
    let mut seen = HashSet::new();
    for agent in &config.agents {
        let canonical = canonical_agent_name(&agent.name)?;
        if !seen.insert(canonical) {
            return Err(ConfigError::DuplicateAgentNames);
        }
    }
    Ok(())
}

/// Loads config if present.
pub fn load(home: &Path) -> Result<Option<RuntimeConfig>, ConfigError> {
    let path = config_path(home);
    let bytes = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(io_err("failed to read config.json", err)),
    };

    let config = serde_json::from_slice::<RuntimeConfig>(&bytes)
        .map_err(|source| ConfigError::Parse { source })?;
    Ok(Some(config))
}

/// Loads config, returning default config when not found.
pub fn load_or_default(home: &Path) -> Result<RuntimeConfig, ConfigError> {
    Ok(load(home)?.unwrap_or_default())
}

/// Repairs stale `activeAgentId` selector drift and persists if repair occurred.
pub fn load_or_default_with_repair(home: &Path) -> Result<RuntimeConfig, ConfigError> {
    let mut config = load_or_default(home)?;
    ensure_no_duplicate_agent_names(&config)?;
    let repaired = repair_stale_active_agent_id(&mut config);
    if repaired {
        save(home, &config)?;
    }
    Ok(config)
}

/// Appends a registry entry and updates active agent selector atomically.
pub fn append_agent_and_set_active(home: &Path, agent: AgentRecord) -> Result<(), ConfigError> {
    let mut config = load_or_default_with_repair(home)?;
    let candidate = canonical_agent_name(&agent.name)?;
    let existing_names = canonical_agent_names(&config)?;
    if existing_names.contains(&candidate) {
        return Err(ConfigError::DuplicateAgentNames);
    }
    config.active_agent_id = Some(agent.agent_id.clone());
    config.agents.push(agent);
    save(home, &config)
}

/// Returns true when the canonicalized name already exists in registry.
pub fn has_agent_name(home: &Path, name: &str) -> Result<bool, ConfigError> {
    let config = load_or_default_with_repair(home)?;
    let candidate = canonical_agent_name(name)?;
    Ok(canonical_agent_names(&config)?.contains(&candidate))
}

fn repair_stale_active_agent_id(config: &mut RuntimeConfig) -> bool {
    let Some(active_id) = config.active_agent_id.as_deref() else {
        return false;
    };
    let exists = config
        .agents
        .iter()
        .any(|agent| agent.agent_id == active_id);
    if exists {
        return false;
    }
    config.active_agent_id = None;
    true
}

/// Saves config via temp-file + rename to avoid partial writes.
pub fn save(home: &Path, config: &RuntimeConfig) -> Result<(), ConfigError> {
    fs::create_dir_all(home).map_err(|err| io_err("failed to create ICEBOX_HOME", err))?;

    let path = config_path(home);
    let tmp_path = path.with_extension("json.tmp");
    let payload =
        serde_json::to_vec_pretty(config).map_err(|source| ConfigError::Serialize { source })?;

    let mut out = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&tmp_path)
        .map_err(|err| io_err("failed to create config.json.tmp", err))?;
    out.write_all(&payload)
        .map_err(|err| io_err("failed to write config.json.tmp", err))?;
    out.flush()
        .map_err(|err| io_err("failed to flush config.json.tmp", err))?;

    fs::rename(&tmp_path, &path).map_err(|err| io_err("failed to replace config.json", err))?;
    Ok(())
}
