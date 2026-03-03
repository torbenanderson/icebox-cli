# E2-02 Execution Spec

## Objective

- Deliver E2-02 (Enclave wrapping key).
- Backlog contract: Create a P-256 key inside the Secure Enclave (non-exportable, per-agent); used to encrypt the Ed25519 private key.

## Problem

- E2-01 generates an Ed25519 identity keypair, but private-key protection is still incomplete until the wrapping key is hardware-backed and non-exportable.
- Without a Secure Enclave wrapping key, filesystem compromise or process-level exfiltration paths can more easily recover long-lived key material.
- This item exists to establish a per-agent hardware trust anchor for wrapping operations in `local-enclave` lane before E2-03 writes encrypted key blobs.

## Scope

- In scope:
  - Create a P-256 key inside the Secure Enclave (non-exportable, per-agent); used to encrypt the Ed25519 private key.
  - Treat this wrapping key as device-branch (`K_device`) material in `local-enclave` lane (not portable identity-branch key material).
  - Persist only stable key references/metadata needed by follow-on steps (for example `enclaveKeyRef`) and keep lane/backend compatibility fields ready for expansion (`identityLane`, `backendClass`, `wrappingScheme`).
- Out of scope:
  - Unrelated backlog items outside E2-02
  - Cross-epic behavior changes not requested by E2-02

## Acceptance Criteria

- AC1: `register-agent` creates a per-agent P-256 wrapping key in Secure Enclave-backed storage for `local-enclave` lane.
- AC2: Wrapping key material is non-exportable (no private key bytes are written to disk or returned through runtime/public API).
- AC3: Wrapping key semantics follow dual-branch model: key is `K_device`-scoped and not treated as portable `K_identity` material.
- AC4: Manifest/identity metadata remains lane-aware and ready for future backend/lane expansion (`enclaveKeyRef` present, reserved lane/backend fields not violated).
- AC5: CLI output/errors are deterministic and user-safe.
- AC6: Failure to create or access the wrapping key returns a deterministic structured runtime error and non-zero exit.
- AC7: Changes are validated with mapped tests.

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
- Architecture alignment:
  - `K_identity` remains the logical portable identity branch.
  - E2-02 creates only device-local `K_device` wrapping material for `local-enclave`.
- Real Secure Enclave prerequisites (runtime verification):
  - Supported Mac hardware (Apple Silicon or Intel Mac with T2/Touch Bar class enclave support).
  - Code-signed binary with required entitlements (at minimum `keychain-access-groups`; hardened runtime expected for release).
  - Run from normal user Terminal session (not root/system daemon context).
- Diagnostic mapping requirement:
  - `OSStatus -26276` should map to a clear actionable debug detail indicating hardware/signing/entitlement verification steps.
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
- [x] docs/plan/TESTING.md (if test mappings are added/changed)
- [ ] docs/architecture/decisions/ADR-*.md (if ADR required)
- [ ] docs/README.md (if user-facing behavior changed)

## Validation Commands

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- `scripts/verify_secure_enclave_prereqs.sh target/release/icebox-cli` (real-device prereq check)

## Execution Notes

- Commit split plan will be finalized in the issue `Execution Plan` comment during `execute`.

---
*Last updated: 2026-03-03*
