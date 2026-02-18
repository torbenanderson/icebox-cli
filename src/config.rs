//! Runtime configuration model.

/// Minimal runtime configuration scaffold.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeConfig {
    /// On-disk config format version.
    pub schema_version: u16,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self { schema_version: 1 }
    }
}
