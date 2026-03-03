# E2-11 Execution Spec

## Objective

- Deliver E2-11 (Active agent tracking).
- Backlog contract: `~/.icebox/config.json` `activeAgentId` tracks the active agent. `agents` array is the registry of all known agents (`agentId` + `name` + DID). Registration appends, removal deletes, rotation updates DID. `--agent` is one-shot targeting only and does not mutate `activeAgentId`.

## Problem

- Identity selection must be explicit and stable across commands, lanes, and devices.
- Without deterministic active-agent tracking, one-shot lane/device operations can silently target the wrong identity.
- This item defines the stable registry/selector contract while leaving lane-specific backend behavior to dedicated items.
- `config.json` is the active selector source-of-truth; corruption/drift must fail safely instead of selecting a wrong identity.

## Scope

- In scope:
  - `~/.icebox/config.json` `activeAgentId` tracks the active agent. `agents` array is the registry of all known agents (`agentId` + `name` + DID). Registration appends, removal deletes, rotation updates DID. `--agent` is one-shot targeting only and does not mutate `activeAgentId`.
  - Load-time integrity checks for selector state:
    - if `activeAgentId` is missing from `agents`, clear it, persist atomically, and require explicit re-selection (`use-agent`/`register-agent`).
    - if registry contains duplicate names, fail closed with deterministic error (no implicit winner).
  - Current naming policy: agent `name` is treated as immutable until explicit rename flow exists.
- Out of scope:
  - Unrelated backlog items outside E2-11
  - Cross-epic behavior changes not requested by E2-11

## Acceptance Criteria

- AC1: `config.json` tracks `activeAgentId` and `agents` registry keyed by `agentId` for deterministic identity selection.
- AC2: `--agent` remains one-shot and never mutates `activeAgentId`.
- AC3: On load/use, stale `activeAgentId` (not present in `agents`) is cleared and persisted atomically; user is required to reselect.
- AC4: Registry duplicate-name state fails closed with deterministic error.
- AC5: Lane/backend metadata additions must not break existing `activeAgentId` selection semantics.
- AC6: CLI output/errors are deterministic and user-safe.
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

- Security goal in scope: prevent accidental cross-identity operations due to selector drift.
- Risk handling in scope:
  - `config.json` corruption/drift is treated as selector-integrity issue; no silent fallback selection.
  - Orphan/mismatch repair UX remains E2-26 (`reconcile`), but E2-11 read path must remain fail-closed and deterministic.
- Explicit non-goals for E2-11:
  - does not implement complete device enrollment protocol,
  - does not define approval/session lease policy enforcement.
- Preserve user-safe default errors (no sensitive internals in normal mode).

## Test Mapping

- Linked tests from `docs/plan/TESTING.md`:
- T-E2-11a
- T-E2-11b
- Add at least:
  - one happy-path test
  - one failure-path test

## ADR Triage

- ADR required? (no):
- Rationale: keep in spec unless long-lived cross-feature decision exists.

## Docs Impact

- [x] docs/plan/spec/PKT-E2-11-work-item.md
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
