//! Agent-facing identity adapters.
//!
//! Command naming can remain "agent" while internal types stay identity-neutral.

use std::error::Error;
use std::fmt::{Display, Formatter};

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
}

impl Display for IdentityNameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str("identity name cannot be empty"),
        }
    }
}

impl Error for IdentityNameError {}
