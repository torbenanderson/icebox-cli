# E1-03 Execution Spec

## Objective

- Deliver E1-03 (Project structure).
- Backlog contract: Create `src/` modules: `agent`, `config`, `vault`, `runner`, `did`. Prefer neutral `identity` naming in internal domain services/types even if MVP CLI remains `agent`-named. Enclave code isolated to `enclave_darwin.rs` (`#[cfg(target_os = "macos")]`, raw FFI) with `enclave_stub.rs` (`#[cfg(not(target_os = "macos"))]`) for non-macOS builds.

## Problem

- Why this exists: implement the backlog contract in a way that is testable, deterministic, and easy to extend.

## Scope

- In scope:
  - Create `src/` modules: `agent`, `config`, `vault`, `runner`, `did`. Prefer neutral `identity` naming in internal domain services/types even if MVP CLI remains `agent`-named. Enclave code isolated to `enclave_darwin.rs` (`#[cfg(target_os = "macos")]`, raw FFI) with `enclave_stub.rs` (`#[cfg(not(target_os = "macos"))]`) for non-macOS builds.
- Out of scope:
  - Unrelated backlog items outside E1-03
  - Cross-epic behavior changes not requested by E1-03

## Acceptance Criteria

- AC1: E1-03 behavior matches backlog description: Create `src/` modules: `agent`, `config`, `vault`, `runner`, `did`. Prefer neutral `identity` naming in internal domain services/types even if MVP CLI remains `agent`-named. Enclave code isolated to `enclave_darwin.rs` (`#[cfg(target_os = "macos")]`, raw FFI) with `enclave_stub.rs` (`#[cfg(not(target_os = "macos"))]`) for non-macOS builds.
- AC2: CLI output/errors are deterministic and user-safe.
- AC3: Changes are validated with mapped tests.

## Rust Implementation Plan

- Crate/module touch points:
  - `src/main.rs` (thin binary entrypoint)
  - `src/lib.rs` (CLI parsing/wiring + module exports)
  - `src/agent.rs`, `src/config.rs`, `src/vault.rs`, `src/runner.rs`, `src/did.rs` (scaffold modules)
  - `src/did/enclave_darwin.rs` and `src/did/enclave_stub.rs` for platform-gated enclave split
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
- `T-E1-03`: project structure modules and platform-gated enclave split compile and load as specified.
- Add at least:
  - happy path: modules compile and expose scaffold types/functions
  - failure path: empty identity name validation rejects invalid input

## ADR Triage

- ADR required? (no):
- Rationale: keep in spec unless long-lived cross-feature decision exists.

## As-Built (Delivered)

- Modules: `src/agent.rs`, `src/config.rs`, `src/vault.rs`, `src/runner.rs`, `src/did.rs`.
- Enclave split: `src/did/enclave_darwin.rs` (macOS) and `src/did/enclave_stub.rs` (non-macOS); scaffold returns backend name only (no FFI in E1-03). Security.framework integration delivered in E2-02 via `src/enclave.rs`.
- Tests: `tests/integration/e1_03_project_structure.rs` (modules compile, empty-name rejection).

## Deferred Design Notes

- Enclave backend placement is intentionally scoped under `did` for MVP (`src/did.rs` + platform-gated enclave files).
- Trigger to split into shared `keystore`/`crypto` module:
  - enclave-backed key operations are required by `vault` and/or `runner`, or
  - enclave/policy logic starts duplicating across non-DID domains.
- When triggered, keep `did` domain-focused (DID format/derivation/validation) and move platform key backend concerns to shared infrastructure.

## Docs Impact

- [x] docs/plan/spec/PKT-E1-03-work-item.md
- [x] docs/plan/TESTING.md (if test mappings are added/changed)
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
