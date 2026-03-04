# E3-01 Execution Spec

## Objective

- Deliver E3-01 (Vault creation).
- Backlog contract: `~/.icebox/identities/<name>/vault.enc` is created on first use as a JSON file of sealed blobs

## Problem

- Why this exists: implement the backlog contract in a way that is testable, deterministic, and easy to extend.

## Scope

- In scope:
  - `~/.icebox/identities/<name>/vault.enc` is created on first use as a versioned JSON envelope.
  - Envelope baseline for MVP create path:
    - top-level `format: "icebox.vault.legacy-v1"` (legacy marker for migration routing)
    - top-level `version: 1`
    - top-level `entries: []` (empty array on first create)
  - Keep envelope minimal for MVP: do not add optional identity self-description fields yet (for example `identity_pubkey`).
- Out of scope:
  - Unrelated backlog items outside E3-01
  - Cross-epic behavior changes not requested by E3-01

## Acceptance Criteria

- AC1: E3-01 behavior matches backlog description with versioned envelope baseline: first create writes `vault.enc` JSON containing `format`, `version: 1`, and `entries: []`.
- AC1a: `add` fails closed when no active agent is configured (`ICE-201` in current MVP mapping) and does not create vault artifacts.
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
- Recovery model note (MVP): losing both `~/.icebox` vault artifacts and the associated identity private key material means encrypted vault contents are non-recoverable by design.

## Test Mapping

- Linked tests from `docs/plan/TESTING.md`:
- T-E3-01
- Add at least:
  - one happy-path test
  - one failure-path test

## ADR Triage

- ADR required? (no):
- Rationale: keep in spec unless long-lived cross-feature decision exists.

## Docs Impact

- [x] docs/plan/spec/PKT-E3-01-work-item.md
- [ ] docs/plan/TESTING.md (if test mappings are added/changed)
- [ ] docs/architecture/decisions/ADR-*.md (if ADR required)
- [ ] docs/README.md (if user-facing behavior changed)

## Validation Commands

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`

## Execution Notes

- Commit split plan will be finalized in the issue `Execution Plan` comment during `execute`.
- Runtime implementation note: `save_vault` uses temp-file + rename (`vault.enc.tmp` -> `vault.enc`) to preserve atomic writes. This is intentionally carried forward from E3-11 into the E3-01/E3-02 execution unit to reduce corruption risk in the first shipped vault path.
- Test evidence note: E3-01 e2e coverage includes both missing `identity.pub` and missing `activeAgentId` failure paths.

---
*Last updated: 2026-03-03*
