# E1-07 Execution Spec

## Objective

- Deliver E1-07 (Disable core dumps).
- Backlog contract: Set `RLIMIT_CORE = 0` at process start to prevent secret leakage via crash dumps

## Problem

- Why this exists: implement the backlog contract in a way that is testable, deterministic, and easy to extend.

## Scope

- In scope:
  - Set `RLIMIT_CORE = 0` at process start to prevent secret leakage via crash dumps
- Out of scope:
  - Unrelated backlog items outside E1-07
  - Cross-epic behavior changes not requested by E1-07

## Acceptance Criteria

- AC1: E1-07 behavior matches backlog description: Set `RLIMIT_CORE = 0` at process start to prevent secret leakage via crash dumps
- AC2: CLI output/errors are deterministic and user-safe.
- AC3: Changes are validated with mapped tests.

## Rust Implementation Plan

- Crate/module touch points:
  - `src/lib.rs` (startup hardening gate) and `src/hardening.rs` (core-dump limit logic).
- Unix runtime hardening implementation:
  - use `rustix::process::{getrlimit, setrlimit}` with `Resource::Core`.
  - set soft core limit (`current`) to `0` and preserve existing hard limit (`maximum`).
  - on `setrlimit` failure, return deterministic `CoreDumpHardeningError::SetLimit(errno)`.
- Non-Unix behavior:
  - no-op return `Ok(())` to keep cross-platform startup behavior deterministic.
- Keep interfaces explicit:
  - prefer small pure functions for parsing/validation paths.
  - avoid hidden global state.
- Error handling:
  - return `Result<T, E>` from fallible logic.
  - avoid `unwrap()` / `expect()` in non-test code paths.
- I/O behavior:
  - perform atomic/checked writes where files are modified.
  - keep side effects localized and observable.

## Security/Runtime Notes

- Keep secret-handling boundaries unchanged unless explicitly in scope.
- Preserve direct-exec/no-shell guarantees where relevant.
- Preserve user-safe default errors (no sensitive internals in normal mode).

## Test Mapping

- Linked tests from `docs/plan/TESTING.md`:
- T-E1-07
- Add at least:
  - one happy-path test
  - one failure-path test

## ADR Triage

- ADR required? (no):
- Rationale: keep in spec unless long-lived cross-feature decision exists.

## Docs Impact

- [x] docs/plan/spec/PKT-E1-07-work-item.md
- [ ] docs/plan/TESTING.md (if test mappings are added/changed)
- [ ] docs/architecture/decisions/ADR-*.md (if ADR required)
- [ ] docs/README.md (if user-facing behavior changed)

## Validation Commands

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`

## Execution Notes

- Commit split plan will be finalized in the issue `Execution Plan` comment during `execute`.

---

*Last updated: 2026-02-19*
