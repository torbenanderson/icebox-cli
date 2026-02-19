//! Command runner scaffolding.

/// Request payload for a future secure command run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunRequest {
    /// Identity context used to resolve secrets.
    pub identity_id: String,
    /// Service key to load from vault.
    pub service: String,
}

impl RunRequest {
    /// Validates and constructs a run request.
    pub fn new(identity_id: &str, service: &str) -> Result<Self, &'static str> {
        if identity_id.trim().is_empty() {
            return Err("identity id cannot be empty");
        }
        if service.trim().is_empty() {
            return Err("service cannot be empty");
        }

        Ok(Self {
            identity_id: identity_id.to_owned(),
            service: service.to_owned(),
        })
    }
}
