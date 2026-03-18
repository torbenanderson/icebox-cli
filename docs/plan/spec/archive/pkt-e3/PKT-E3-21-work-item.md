# E3-21 Execution Spec

## Objective

- Deliver E3-21 (Identity/config refactor baseline).
- Backlog contract: Refactor identity/config foundations before broader E3 delivery: clarify DID backend naming (non-enclave behavior labels), tighten config-to-runtime error mapping (invalid config gets dedicated code path), split `register-agent` into smaller units, and reduce duplicate canonical-name scans with reusable/cached canonical sets.

## Problem

- Why this exists: implement the backlog contract in a way that is testable, deterministic, and easy to extend.

## Scope

- In scope:
  - Refactor identity/config foundations before broader E3 delivery: clarify DID backend naming (non-enclave behavior labels), tighten config-to-runtime error mapping (invalid config gets dedicated code path), split `register-agent` into smaller units, and reduce duplicate canonical-name scans with reusable/cached canonical sets.
- Out of scope:
  - Unrelated backlog items outside E3-21
  - Cross-epic behavior changes not requested by E3-21

## Acceptance Criteria

- AC1: E3-21 behavior matches backlog description: Refactor identity/config foundations before broader E3 delivery: clarify DID backend naming (non-enclave behavior labels), tighten config-to-runtime error mapping (invalid config gets dedicated code path), split `register-agent` into smaller units, and reduce duplicate canonical-name scans with reusable/cached canonical sets.
- AC2: Invalid/corrupt config mapping reaches the dedicated runtime invalid-config code path (`ICE-309`), with deterministic user-safe messaging.
- AC3: Changes are validated with mapped tests.

## Rust Implementation Plan

- Crate/module touch points:
  - `src/main.rs` (CLI wiring) and focused domain module(s) only.
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
- T-E3-21a
- T-E3-21b
- T-E3-21c
- Add at least:
  - one happy-path test
  - one failure-path test

## ADR Triage

- ADR required? (no):
- Rationale: keep in spec unless long-lived cross-feature decision exists.

## Docs Impact

- [x] docs/plan/spec/PKT-E3-21-work-item.md
- [ ] docs/plan/TESTING.md (if test mappings are added/changed)
- [ ] docs/architecture/decisions/ADR-*.md (if ADR required)
- [ ] docs/README.md (if user-facing behavior changed)

## Validation Commands

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`

## Execution Notes

- Commit split plan will be finalized in the issue `Execution Plan` comment during `execute`.
- Implementation details completed in this packet:
  - DID internal backend modules renamed to backend-oriented names (`backend_darwin`, `backend_stub`) with no runtime output change.
  - `register-agent` flow decomposed into focused helpers with centralized cleanup guard (`RegistrationCleanup`), preserving no-plaintext and partial-artifact cleanup invariants.
  - Config error mapping centralized via `RegisterAgentError::from_config_error(...)` to avoid ad-hoc mapping branches.
  - Canonical-name lookup consolidated in config layer via reusable canonical-name set helper.
  - Shared formatting/id helpers centralized in `src/util.rs` (`bytes_to_hex`, `generate_agent_id`).
  - Added regression coverage for invalid stored config agent names mapping to `ICE-309`.
