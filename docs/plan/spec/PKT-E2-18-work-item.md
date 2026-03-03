# E2-18 Execution Spec

## Objective

- Deliver E2-18 (Agent name validation).
- Backlog contract: Agent names validated: lowercase letters, numbers, hyphens, 3-32 characters

## Problem

- Name validation must be deterministic across all name-entry points (`register-agent`, `use-agent`, `--agent`).
- Inconsistent parsing rules create ambiguity and weaken duplicate-guard guarantees.

## Scope

- In scope:
  - Agent names validated: lowercase letters, numbers, hyphens, 3-32 characters
  - Define a single canonical parser/validator used by all agent-name entry points.
  - Validation must align with E2-09 duplicate checks to avoid case/format mismatch.
- Out of scope:
  - Unrelated backlog items outside E2-18
  - Cross-epic behavior changes not requested by E2-18

## Acceptance Criteria

- AC1: Agent names are accepted only when they match `[a-z0-9-]{3,32}`.
- AC2: Invalid names (uppercase, special chars, short/long values, empty/whitespace) fail with deterministic user-safe error.
- AC3: Same validation rules are applied consistently across `register-agent`, `use-agent`, and `--agent`.
- AC4: Validation behavior aligns with E2-09 duplicate checks.
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

- Security goal in scope: eliminate identity-name ambiguity and parser drift across command surfaces.
- Keep secret-handling boundaries unchanged unless explicitly in scope.
- Preserve user-safe default errors (no sensitive internals in normal mode).

## Test Mapping

- Linked tests from `docs/plan/TESTING.md`:
- T-E2-18
- Add at least:
  - one happy-path test
  - one failure-path test

## ADR Triage

- ADR required? (no):
- Rationale: keep in spec unless long-lived cross-feature decision exists.

## Docs Impact

- [x] docs/plan/spec/PKT-E2-18-work-item.md
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
