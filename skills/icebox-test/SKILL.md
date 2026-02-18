---
name: icebox-test
description: Author runnable tests during execute so each backlog item ships with executable happy-path and failure-path coverage, not docs-only test mapping.
---

# Icebox Test

Use this skill alongside `icebox-execute` whenever implementation starts, or standalone via `build tests`.

## Goal

Convert planned test mappings into runnable tests in the same change set.

## Standalone Trigger

Use `build tests` for spec-scoped test authoring outside execute.

Preferred forms:

1. `build tests #<issue-id>`
2. `build tests E*`

Behavior:

1. Resolve packet/spec context from issue or backlog ID.
2. Add/update runnable tests under the test architecture conventions.
3. Sync `docs/plan/TESTING.md` mappings for any new or changed tests.

## Rules

1. For each executed backlog item, add or update runnable tests tied to that item.
2. Minimum coverage per item:
   - one happy-path test
   - one failure-path test
3. Prefer placement by scope:
   - module-level behavior: inline `#[cfg(test)]` unit tests in `src/**`
   - crate-boundary behavior: integration tests under `tests/integration_*.rs`
   - CLI/user-flow behavior: E2E tests under `tests/e2e_*.rs`
   - hardening/OS behavior: system or security tests under `tests/system_*.rs` or `tests/security_*.rs`
   - when using subfolders, keep top-level `tests/*.rs` entry files for Cargo discovery
4. Keep tests deterministic:
   - no network calls
   - no real secrets
   - temp files/dirs only
5. Update `docs/plan/TESTING.md` mappings when new test IDs or behaviors are introduced.

## Execute Coupling

When `execute #<id>` runs:

1. Read packet spec and mapped tests.
2. Implement code and test artifacts together.
3. Fail execute completion if tests are only mapped in docs but no runnable test artifact was added/updated.

## Output Requirements

Return:

1. Test artifact list (file paths + test names).
2. Mapping line(s) from backlog ID to test IDs.
3. Commands run and pass/fail summary.
4. Level classification used (`unit`, `integration`, `e2e`, `system/security`).
