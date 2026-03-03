//! Structured user-facing diagnostics for CLI boundary errors.

use clap::error::ErrorKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum IceErrorCode {
    InputValidation,
    IdentitySetup,
    DuplicateAgentName,
    InvalidAgentName,
    InvalidConfig,
    DuplicateConfigAgentNames,
    EnclaveFailure,
}

impl IceErrorCode {
    pub(crate) const fn code(self) -> &'static str {
        match self {
            Self::InputValidation => "ICE-701",
            Self::IdentitySetup => "ICE-306",
            Self::DuplicateAgentName => "ICE-307",
            Self::InvalidAgentName => "ICE-308",
            Self::InvalidConfig => "ICE-309",
            Self::DuplicateConfigAgentNames => "ICE-310",
            Self::EnclaveFailure => "ICE-311",
        }
    }

    pub(crate) const fn message(self) -> &'static str {
        match self {
            Self::InputValidation => "Invalid input. See `--help` for usage.",
            Self::IdentitySetup => "Identity setup failed.",
            Self::DuplicateAgentName => "Agent already exists.",
            Self::InvalidAgentName => "Invalid agent name.",
            Self::InvalidConfig => "Config is invalid.",
            Self::DuplicateConfigAgentNames => "Config has duplicate agent names.",
            Self::EnclaveFailure => "Secure Enclave operation failed.",
        }
    }
}

pub(crate) fn map_clap_error(kind: ErrorKind) -> IceErrorCode {
    match kind {
        ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => IceErrorCode::InputValidation,
        _ => IceErrorCode::InputValidation,
    }
}

pub(crate) fn format_cli_error(
    code: IceErrorCode,
    debug_enabled: bool,
    detail: Option<&str>,
) -> String {
    if debug_enabled {
        match detail {
            Some(detail) => format!("[{}] {}\n{detail}", code.code(), code.message()),
            None => format!("[{}] {}", code.code(), code.message()),
        }
    } else {
        format!("[{}] {}", code.code(), code.message())
    }
}

pub(crate) fn format_runtime_error(
    code: IceErrorCode,
    debug_enabled: bool,
    detail: Option<&str>,
) -> String {
    if matches!(
        code,
        IceErrorCode::DuplicateAgentName
            | IceErrorCode::InvalidAgentName
            | IceErrorCode::InvalidConfig
            | IceErrorCode::DuplicateConfigAgentNames
            | IceErrorCode::EnclaveFailure
    ) {
        if let Some(detail) = detail {
            return format!("[{}] {detail}", code.code());
        }
    }
    if debug_enabled {
        match detail {
            Some(detail) => format!("[{}] {}\n{detail}", code.code(), code.message()),
            None => format!("[{}] {}", code.code(), code.message()),
        }
    } else {
        format!("[{}] {}", code.code(), code.message())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_default_error_message_uses_safe_text_only() {
        let rendered = format_cli_error(
            IceErrorCode::InputValidation,
            false,
            Some("unexpected argument '--oops' found"),
        );

        assert_eq!(rendered, "[ICE-701] Invalid input. See `--help` for usage.");
    }

    #[test]
    fn format_debug_error_message_includes_detail() {
        let rendered = format_cli_error(
            IceErrorCode::InputValidation,
            true,
            Some("unexpected argument '--oops' found"),
        );

        assert!(rendered.contains("[ICE-701]"));
        assert!(rendered.contains("unexpected argument '--oops' found"));
    }

    #[test]
    fn format_runtime_error_hides_detail_in_default_mode() {
        let rendered = format_runtime_error(
            IceErrorCode::IdentitySetup,
            false,
            Some("failed to create agent directory: permission denied"),
        );

        assert_eq!(rendered, "[ICE-306] Identity setup failed.");
    }

    #[test]
    fn format_duplicate_name_error_shows_friendly_detail_in_default_mode() {
        let rendered = format_runtime_error(
            IceErrorCode::DuplicateAgentName,
            false,
            Some(
                "Agent claw already exists. Choose a different name or remove the existing agent.",
            ),
        );

        assert_eq!(
            rendered,
            "[ICE-307] Agent claw already exists. Choose a different name or remove the existing agent."
        );
    }

    #[test]
    fn format_invalid_name_error_shows_friendly_detail_in_default_mode() {
        let rendered = format_runtime_error(
            IceErrorCode::InvalidAgentName,
            false,
            Some("Invalid agent name. Use [a-z0-9-]{3,32} and do not start with '-'."),
        );

        assert_eq!(
            rendered,
            "[ICE-308] Invalid agent name. Use [a-z0-9-]{3,32} and do not start with '-'."
        );
    }

    #[test]
    fn format_invalid_config_error_shows_friendly_detail_in_default_mode() {
        let rendered = format_runtime_error(
            IceErrorCode::InvalidConfig,
            false,
            Some("Config is invalid. Fix /tmp/test/config.json or reinitialize."),
        );

        assert_eq!(
            rendered,
            "[ICE-309] Config is invalid. Fix /tmp/test/config.json or reinitialize."
        );
    }

    #[test]
    fn format_duplicate_config_names_error_shows_friendly_detail_in_default_mode() {
        let rendered = format_runtime_error(
            IceErrorCode::DuplicateConfigAgentNames,
            false,
            Some(
                "Config has duplicate agent names. Resolve duplicates in /tmp/test/config.json and retry.",
            ),
        );

        assert_eq!(
            rendered,
            "[ICE-310] Config has duplicate agent names. Resolve duplicates in /tmp/test/config.json and retry."
        );
    }

    #[test]
    fn format_enclave_failure_error_shows_friendly_detail_in_default_mode() {
        let rendered = format_runtime_error(
            IceErrorCode::EnclaveFailure,
            false,
            Some(
                "Secure Enclave operation failed. Check supported hardware and signing/entitlements.",
            ),
        );

        assert_eq!(
            rendered,
            "[ICE-311] Secure Enclave operation failed. Check supported hardware and signing/entitlements."
        );
    }
}
