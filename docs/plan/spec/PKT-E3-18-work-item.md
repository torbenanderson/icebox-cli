# E3-18 Execution Spec

## Objective

- Deliver E3-18 (HMAC key recovery).
- Backlog contract: Phase 1.5. `recover-agent` generates a new `hmac.enc` (new random HMAC key, encrypted by the new enclave key). First vault write after recovery establishes HMAC baseline.

## Problem

- Why this exists: implement the backlog contract in a way that is testable, deterministic, and easy to extend.

## Scope

- In scope:
  - Phase 1.5. `recover-agent` generates a new `hmac.enc` (new random HMAC key, encrypted by the new enclave key). First vault write after recovery establishes HMAC baseline.
- Out of scope:
  - Unrelated backlog items outside E3-18
  - Cross-epic behavior changes not requested by E3-18

## Acceptance Criteria

- AC1: E3-18 behavior matches backlog description: Phase 1.5. `recover-agent` generates a new `hmac.enc` (new random HMAC key, encrypted by the new enclave key). First vault write after recovery establishes HMAC baseline.
- AC2: CLI output/errors are deterministic and user-safe.
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
- T-E3-18
- Add at least:
  - one happy-path test
  - one failure-path test

## ADR Triage

- ADR required? (no):
- Rationale: keep in spec unless long-lived cross-feature decision exists.

## Docs Impact

- [x] docs/plan/spec/PKT-E3-18-work-item.md
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
*Last updated: 2026-03-03*
