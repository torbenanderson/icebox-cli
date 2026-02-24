# E2-02 Execution Spec

## Objective

- Deliver E2-02 (Enclave wrapping key).
- Backlog contract: Create a P-256 key inside the Secure Enclave (non-exportable, per-agent); used to encrypt the Ed25519 private key

## Problem

- E2-01 generates an Ed25519 identity keypair, but private-key protection is still incomplete until the wrapping key is hardware-backed and non-exportable.
- Without a Secure Enclave wrapping key, filesystem compromise or process-level exfiltration paths can more easily recover long-lived key material.
- This item exists to establish a per-agent hardware trust anchor for wrapping operations before E2-03 writes encrypted key blobs.

## Scope

- In scope:
  - Create a P-256 key inside the Secure Enclave (non-exportable, per-agent); used to encrypt the Ed25519 private key
- Out of scope:
  - Unrelated backlog items outside E2-02
  - Cross-epic behavior changes not requested by E2-02

## Acceptance Criteria

- AC1: `register-agent` creates a per-agent P-256 wrapping key in Secure Enclave-backed storage.
- AC2: Wrapping key material is non-exportable (no private key bytes are written to disk or returned through runtime/public API).
- AC3: CLI output/errors are deterministic and user-safe.
- AC4: Failure to create or access the wrapping key returns a deterministic structured runtime error and non-zero exit.
- AC5: Changes are validated with mapped tests.

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

- Security goal in scope: enforce hardware-backed, non-exportable wrapping-key creation per agent.
- Explicit non-goals for E2-02:
  - does not yet persist the wrapped Ed25519 private key blob (E2-03),
  - does not yet complete no-plaintext-on-disk enforcement across the full flow (E2-04),
  - does not change command-execution trust boundaries.
- Preserve user-safe default errors (no sensitive internals in normal mode).

## Test Mapping

- Linked tests from `docs/plan/TESTING.md`:
- T-E2-02
- Add at least:
  - one happy-path test
  - one failure-path test

## ADR Triage

- ADR required? (no):
- Rationale: keep in spec unless long-lived cross-feature decision exists.

## Docs Impact

- [x] docs/plan/spec/PKT-E2-02-work-item.md
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
*Last updated: 2026-02-24*
