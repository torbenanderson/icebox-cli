# E1-02 Execution Spec

## Objective

- Deliver E1-02 (CLI scaffolding).
- Backlog contract: Set up `src/main.rs` with `clap`

## Problem

- Why this exists: implement the backlog contract in a way that is testable, deterministic, and easy to extend.

## Scope

- In scope:
  - Set up `src/main.rs` with `clap`
- Out of scope:
  - Unrelated backlog items outside E1-02
  - Cross-epic behavior changes not requested by E1-02

## Acceptance Criteria

- AC1: `src/main.rs` is scaffolded to use `clap` for top-level CLI parsing.
- AC2: Running `cargo run -- --help` succeeds and shows CLI help output.
- AC3: CLI help includes standard project metadata (repository link).
- AC4: Changes are validated with mapped tests.

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

## Test Mapping

- Linked tests from `docs/plan/TESTING.md`:
- `T-E1-02`: CLI scaffolding compiles with `clap`, `--help` output path is wired, and help metadata includes repository information.
- Scaffold-only validation mapping (no runtime feature code in scope):
  - verify `Cargo.toml` includes `clap` dependency and compiles
  - verify `src/main.rs` contains clap parser scaffolding
  - run `cargo check`
  - run `cargo run -- --help`
- Add at least:
  - one happy-path test
  - one failure-path test

## ADR Triage

- ADR required? (no):
- Rationale: keep in spec unless long-lived cross-feature decision exists.

## Docs Impact

- [x] docs/plan/spec/PKT-E1-02-work-item.md
- [x] docs/plan/TESTING.md (if test mappings are added/changed)
- [ ] docs/architecture/decisions/ADR-*.md (if ADR required)
- [ ] docs/README.md (if user-facing behavior changed)

## Validation Commands

- `cargo check`
- `cargo run -- --help`

## Execution Notes

- Commit split plan will be finalized in the issue `Execution Plan` comment during `execute`.

## As-Built (Delivered)

- `src/lib.rs` with clap `Parser`; `--help` wired; repository link in `after_help`.
- Tests: `tests/e2e/e1_02_cli_scaffolding.rs` (help exits 0, unknown flag exits 2).

---

*Last updated: 2026-03-03*
