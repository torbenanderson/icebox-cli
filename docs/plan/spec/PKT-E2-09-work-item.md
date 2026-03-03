# E2-09 Execution Spec

## Objective

- Deliver E2-09 (Duplicate guard).
- Backlog contract: Registering an agent name that already exists in the `agents` registry returns a clear error

## Problem

- Name collisions in the registry can make `use-agent <name>` and `--agent <name>` ambiguous.
- Schema alone does not guarantee unique names; runtime validation must enforce uniqueness.

## Scope

- In scope:
  - Registering an agent name that already exists in the `agents` registry returns a clear error
  - Enforce duplicate-name guard at registration boundary and config-load validation boundary.
  - Duplicate detection is deterministic and case/format aligned with E2-18 canonical name parser.
  - Duplicate checks must reuse `IdentityName::parse` (or a single shared equivalent) and compare canonical stored forms; do not implement a parallel parser.
- Out of scope:
  - Unrelated backlog items outside E2-09
  - Cross-epic behavior changes not requested by E2-09

## Acceptance Criteria

- AC1: Registering an agent name that already exists in `agents` fails with deterministic clear error (no overwrite).
- AC2: Config-load validation fails closed when `agents` already contains duplicate names.
- AC3: Duplicate checks use canonical output from `IdentityName::parse` (or single shared equivalent), not duplicated validation logic.
- AC4: CLI output/errors are deterministic and user-safe.
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

- Security goal in scope: prevent ambiguous identity selection caused by registry collisions.
- Keep secret-handling boundaries unchanged unless explicitly in scope.
- Preserve user-safe default errors (no sensitive internals in normal mode).

## Test Mapping

- Linked tests from `docs/plan/TESTING.md`:
- T-E2-09
- Add at least:
  - one happy-path test
  - one failure-path test

## ADR Triage

- ADR required? (no):
- Rationale: keep in spec unless long-lived cross-feature decision exists.

## Docs Impact

- [x] docs/plan/spec/PKT-E2-09-work-item.md
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
