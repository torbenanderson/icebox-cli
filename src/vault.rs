//! Vault domain scaffolding.

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
