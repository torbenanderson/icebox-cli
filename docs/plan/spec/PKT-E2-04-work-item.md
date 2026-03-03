# E2-04 Execution Spec

## Objective

- Deliver E2-04 (No plaintext key on disk).
- Backlog contract: Ed25519 private key never written to disk in plaintext; only the enclave-wrapped `key.enc` blob exists.

## Problem

- E2-03 adds encrypted persistence, but security posture is incomplete without explicit guarantees that plaintext Ed25519 key bytes are never written to disk.
- This item enforces disk-write boundaries for `local-enclave` lane and defines verification/failure behavior for unsafe write paths.

## Scope

- In scope:
  - Ed25519 private key never written to disk in plaintext; only the enclave-wrapped `key.enc` blob exists.
  - Enforce the disk boundary under dual-branch model: portable identity branch (`K_identity`) is never persisted as plaintext private key material; device-branch (`K_device`) wrapping artifacts remain non-exportable.
  - Verify local-enclave artifact expectations (`key.enc` on disk, no plaintext private key spill files, deterministic fail-closed behavior on unsafe paths).
- Out of scope:
  - Unrelated backlog items outside E2-04
  - Cross-epic behavior changes not requested by E2-04

## Acceptance Criteria

- AC1: Registration/wrapping flow never writes plaintext Ed25519 private-key bytes to disk in `local-enclave` lane.
- AC2: Only wrapped blob artifacts (`key.enc`) are persisted for identity private-key material.
- AC3: Dual-branch boundary remains intact: no implementation path treats device wrapping artifacts as portable identity private-key export.
- AC4: Unsafe persistence paths fail closed with deterministic runtime errors.
- AC5: CLI output/errors are deterministic and user-safe.
- AC6: Changes are validated with mapped tests.

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

- Security goal in scope: verify and enforce no-plaintext-on-disk contract for identity private key material.
- Architecture alignment:
  - E2-04 protects `K_identity` private material from disk plaintext exposure while preserving `K_device` non-exportable semantics for wrapping.
- Artifact contract under this hardening step:
  - `enclave.keyref`: persisted key label/reference metadata only.
  - `identity.pub`: persisted Ed25519 public-key bytes only.
  - `key.enc`: only persisted private-key artifact; plaintext Ed25519 private bytes must not be written to disk.
- Explicit non-goals for E2-04:
  - does not eliminate runtime in-memory plaintext windows during unwrap/use,
  - does not replace approval/session controls handled by broker policy lanes.
- Preserve user-safe default errors (no sensitive internals in normal mode).

## Test Mapping

- Linked tests from `docs/plan/TESTING.md`:
- T-E2-04
- Add at least:
  - one happy-path test
  - one failure-path test

## ADR Triage

- ADR required? (no):
- Rationale: keep in spec unless long-lived cross-feature decision exists.

## Docs Impact

- [x] docs/plan/spec/PKT-E2-04-work-item.md
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
