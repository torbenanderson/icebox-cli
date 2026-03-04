# E3-11 Execution Spec

## Objective

- Deliver E3-11 (Atomic vault writes).
- Backlog contract: Vault updates written to `vault.enc.tmp` then atomically renamed via `std::fs::rename`; prevents corruption on crash

## Problem

- Why this exists: implement the backlog contract in a way that is testable, deterministic, and easy to extend.

## Scope

- In scope:
  - Vault updates written to `vault.enc.tmp` then atomically renamed via `std::fs::rename`; prevents corruption on crash
- Out of scope:
  - Unrelated backlog items outside E3-11
  - Cross-epic behavior changes not requested by E3-11

## Acceptance Criteria

- AC1: E3-11 behavior matches backlog description: Vault updates written to `vault.enc.tmp` then atomically renamed via `std::fs::rename`; prevents corruption on crash
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
- T-E3-11
- Add at least:
  - one happy-path test
  - one failure-path test

## ADR Triage

- ADR required? (no):
- Rationale: keep in spec unless long-lived cross-feature decision exists.

## Docs Impact

- [x] docs/plan/spec/PKT-E3-11-work-item.md
- [ ] docs/plan/TESTING.md (if test mappings are added/changed)
- [ ] docs/architecture/decisions/ADR-*.md (if ADR required)
- [ ] docs/README.md (if user-facing behavior changed)

## Validation Commands

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`

## Execution Notes

- Commit split plan will be finalized in the issue `Execution Plan` comment during `execute`.
- Status note: Core atomic-write behavior has already been implemented in `src/vault.rs` during E3-01/E3-02 execution (`vault.enc.tmp` write + `std::fs::rename` replace). When executing E3-11 formally, focus is to validate/expand dedicated E3-11 test coverage and closeout evidence rather than re-implementing the write path.

---
*Last updated: 2026-03-03*
