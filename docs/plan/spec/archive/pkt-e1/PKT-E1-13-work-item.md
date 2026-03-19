# E1-13 Execution Spec

## Objective

- Deliver E1-13 (Structured error codes).
- Backlog contract: Error codes (`ICE-1xx`, `ICE-2xx`, etc.) categorized by root cause (auth, vault, agent, secret, exec, enclave, input). Safe for support tickets without exposing internals. Codes never reused.

## Problem

- Why this exists: implement the backlog contract in a way that is testable, deterministic, and easy to extend.

## Scope

- In scope:
  - Error codes (`ICE-1xx`, `ICE-2xx`, etc.) categorized by root cause (auth, vault, agent, secret, exec, enclave, input). Safe for support tickets without exposing internals. Codes never reused.
- Out of scope:
  - Unrelated backlog items outside E1-13
  - Cross-epic behavior changes not requested by E1-13

## Acceptance Criteria

- AC1: E1-13 behavior matches backlog description: Error codes (`ICE-1xx`, `ICE-2xx`, etc.) categorized by root cause (auth, vault, agent, secret, exec, enclave, input). Safe for support tickets without exposing internals. Codes never reused.
- AC2: CLI output/errors are deterministic and user-safe.
- AC3: Changes are validated with mapped tests.

## Rust Implementation Plan

- Crate/module touch points:
  - `src/lib.rs` (CLI boundary mapping), `src/main.rs` (thin entrypoint), and `src/error.rs` (typed ICE code registry/mapping).
- MVP source-of-truth policy:
  - runtime truth lives in code (`src/error.rs`) as typed enum/constants plus deterministic code/message mapping.
  - do not introduce JSON-driven code generation in MVP.
- Docs authority policy:
  - docs remain lightweight reference material and are not authoritative for runtime behavior.
  - runtime mapping in code is authoritative during MVP; machine-readable docs artifacts are a later-phase concern.
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
- T-E1-13
- Add at least:
  - one happy-path test
  - one failure-path test

## ADR Triage

- ADR required? (no):
- Rationale: keep in spec unless long-lived cross-feature decision exists.

## Docs Impact

- [x] docs/plan/spec/PKT-E1-13-work-item.md
- [ ] docs/plan/TESTING.md (if test mappings are added/changed)
- [ ] docs/architecture/decisions/ADR-*.md (if ADR required)
- [ ] docs/README.md (if user-facing behavior changed)

## Validation Commands

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`

## Execution Notes

- Commit split plan will be finalized in the issue `Execution Plan` comment during `execute`.

## As-Built (Delivered)

- `src/error.rs`: `IceErrorCode` enum (InputValidation → ICE-701, IdentitySetup → ICE-306); `format_cli_error`, `format_runtime_error` with debug-toggle detail; `map_clap_error`. Default mode hides internal detail; `--debug` exposes it.
- `docs/reference/error-codes.json`: machine-readable runtime registry aligned with the implemented `IceErrorCode` set.
- Tests: `tests/e2e/e1_13_structured_error_codes.rs` (default emits ICE-701 without detail; debug emits code + detail).

---

*Last updated: 2026-03-18*
