//! Structured user-facing diagnostics for CLI boundary errors.

use clap::error::ErrorKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum IceErrorCode {
    InputValidation,
    IdentitySetup,
}

impl IceErrorCode {
    pub(crate) const fn code(self) -> &'static str {
        match self {
            Self::InputValidation => "ICE-701",
            Self::IdentitySetup => "ICE-306",
        }
    }

    pub(crate) const fn message(self) -> &'static str {
        match self {
            Self::InputValidation => "Invalid input. See `--help` for usage.",
            Self::IdentitySetup => "Identity setup failed.",
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
}
