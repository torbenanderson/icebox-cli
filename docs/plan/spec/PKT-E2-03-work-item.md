# E2-03 Execution Spec

## Objective

- Deliver E2-03 (Wrap Ed25519 key).
- Backlog contract: Encrypt the Ed25519 private key with the enclave P-256 key (`SecKeyCreateEncryptedData`); store as `key.enc`.

## Problem

- E2-02 creates the hardware-backed wrapping key, but key-protection is incomplete until identity private-key material is persisted only as wrapped ciphertext.
- This item defines the `local-enclave` lane persistence boundary (`key.enc`) and the deterministic failure behavior when wrapping cannot be completed.
- The output artifact must stay compatible with future lane expansion (paired/remote signer) without changing identity primary keys.

## Scope

- In scope:
  - Encrypt the Ed25519 private key with the enclave P-256 key (`SecKeyCreateEncryptedData`); store as `key.enc`.
  - Use the E2-02 device-branch (`K_device`) wrapping key in `local-enclave` lane; do not alter portable identity semantics.
  - Keep manifest compatibility contract coherent with wrapping artifacts (`enclaveKeyRef` linkage retained; no contradictory lane/backend metadata).
- Out of scope:
  - Unrelated backlog items outside E2-03
  - Cross-epic behavior changes not requested by E2-03

## Acceptance Criteria

- AC1: Ed25519 private key bytes are encrypted via enclave wrapping key and persisted as `key.enc` in `local-enclave` lane.
- AC2: `key.enc` is non-empty and parseable as encrypted blob format expected by unwrap path.
- AC3: Wrap path uses per-agent device-branch wrapping material (`K_device`) and does not expose private-key bytes through logs/API/files.
- AC4: Wrapping failures return deterministic structured runtime errors and non-zero exit.
- AC5: CLI output/errors are user-safe in default mode.
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

- Security goal in scope: ensure at-rest private-key persistence is ciphertext-only for `local-enclave`.
- Architecture alignment:
  - E2-03 consumes device-local wrapping capability and preserves portability boundaries from ADR-0002 (`K_identity` vs `K_device`).
- Artifact contract:
  - `enclave.keyref`: persisted label/reference only (not key bytes).
  - `identity.pub`: persisted Ed25519 public-key bytes.
  - `key.enc`: persisted wrapped private-key blob; treated as opaque storage format.
- Known deferred hardening notes (tracked by backlog):
  - Partial-artifact risk (for example `key.enc` created before all identity artifacts) is tolerated in E2-03 and tightened in E2-04.
  - Residual plaintext-memory windows from intermediate buffers are deferred to dedicated memory-hardening work in E3-07 (`secrecy` + `Zeroize`).
- Explicit non-goals for E2-03:
  - does not complete global "never plaintext on disk" verification controls (E2-04),
  - does not define paired/remote-signer transport protocol.
- Preserve user-safe default errors (no sensitive internals in normal mode).

## Test Mapping

- Linked tests from `docs/plan/TESTING.md`:
- T-E2-03
- Add at least:
  - one happy-path test
  - one failure-path test

## ADR Triage

- ADR required? (no):
- Rationale: keep in spec unless long-lived cross-feature decision exists.

## Docs Impact

- [x] docs/plan/spec/PKT-E2-03-work-item.md
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
