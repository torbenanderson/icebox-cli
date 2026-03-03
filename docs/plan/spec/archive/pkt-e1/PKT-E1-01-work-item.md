# E1-01 Execution Spec

## Objective

- Deliver E1-01 (Cargo init).
- Backlog contract: `cargo init` (icebox-cli crate)

## Problem

- Why this exists: implement the backlog contract in a way that is testable, deterministic, and easy to extend.

## Scope

- In scope:
  - `cargo init` (icebox-cli crate)
- Out of scope:
  - Unrelated backlog items outside E1-01
  - Cross-epic behavior changes not requested by E1-01

## Acceptance Criteria

- AC1: Running `cargo init` for `icebox-cli` yields a valid Rust binary crate scaffold with `Cargo.toml` and `src/main.rs`.
- AC2: Scaffold creation is non-interactive and reproducible for the same inputs.
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
- `T-E1-01`: Cargo scaffold validation for manifest/bin presence and invalid-manifest failure path.
- Scaffold-only validation mapping (no runtime feature code in scope):
  - verify `Cargo.toml` exists and package name is `icebox-cli`
  - verify `src/main.rs` exists
  - run `cargo check`
- Add at least:
  - one happy-path test
  - one failure-path test

## ADR Triage

- ADR required? (no):
- Rationale: keep in spec unless long-lived cross-feature decision exists.

## Docs Impact

- [x] docs/plan/spec/PKT-E1-01-work-item.md
- [x] docs/plan/TESTING.md (if test mappings are added/changed)
- [ ] docs/architecture/decisions/ADR-*.md (if ADR required)
- [ ] docs/README.md (if user-facing behavior changed)

## Validation Commands

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`

## Execution Notes

- Commit split plan will be finalized in the issue `Execution Plan` comment during `execute`.

## As-Built (Delivered)

- `Cargo.toml` with package name `icebox-cli`; `src/main.rs` thin entrypoint.
- Tests: `tests/integration/e1_01_cargo_init.rs` (scaffold manifest/main, missing-manifest failure).
