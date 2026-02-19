//! Runtime hardening checks applied during process startup.

use std::fmt;

#[cfg(unix)]
use rustix::process::{Resource, Rlimit, getrlimit, setrlimit};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum CoreDumpHardeningError {
    GetLimit(i32),
    SetLimit(i32),
}

impl fmt::Display for CoreDumpHardeningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GetLimit(code) => write!(f, "unable to read process core limit (errno {})", code),
            Self::SetLimit(code) => {
                write!(f, "unable to disable process core dumps (errno {})", code)
            }
        }
    }
}

#[cfg(unix)]
fn disable_core_dumps_with<G, S>(
    mut get_limit: G,
    mut set_limit: S,
) -> Result<(), CoreDumpHardeningError>
where
    G: FnMut() -> Result<Rlimit, i32>,
    S: FnMut(Rlimit) -> Result<(), i32>,
{
    let current = get_limit().map_err(CoreDumpHardeningError::GetLimit)?;

    let hardened = Rlimit {
        current: Some(0),
        maximum: current.maximum,
    };
    set_limit(hardened).map_err(CoreDumpHardeningError::SetLimit)?;

    Ok(())
}

#[cfg(all(unix, test))]
fn current_core_limit() -> Rlimit {
    getrlimit(Resource::Core)
}

pub(crate) fn disable_core_dumps() -> Result<(), CoreDumpHardeningError> {
    #[cfg(unix)]
    {
        disable_core_dumps_with(
            || {
                let current = getrlimit(Resource::Core);
                Ok(current)
            },
            |limit| setrlimit(Resource::Core, limit).map_err(|errno| errno.raw_os_error()),
        )
    }

    #[cfg(not(unix))]
    {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    #[test]
    fn e1_07_happy_path_sets_core_soft_limit_to_zero() {
        disable_core_dumps().expect("core-dump hardening should succeed");
        let current = current_core_limit();
        assert_eq!(current.current, Some(0));
    }

    #[cfg(unix)]
    #[test]
    fn e1_07_failure_path_surfaces_set_limit_error() {
        let err = disable_core_dumps_with(
            || {
                Ok(Rlimit {
                    current: Some(1024),
                    maximum: Some(2048),
                })
            },
            |_limit| Err(rustix::io::Errno::PERM.raw_os_error()),
        )
        .expect_err("setrlimit failure must be returned");

        assert_eq!(
            err,
            CoreDumpHardeningError::SetLimit(rustix::io::Errno::PERM.raw_os_error())
        );
    }
}
