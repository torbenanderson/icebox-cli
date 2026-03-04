# E3-29 Execution Spec

## Objective

- Deliver E3-29 (Vault locking/error-path refactor cleanup).
- Backlog contract: Refactor vault lock/error-path internals after E3 core execution: centralize lock lifecycle helper boundaries and normalize lock/open/unlock error mapping while preserving E3-10/11/12 behavior and test outcomes

## Problem

- Why this exists: current E3 lock and error-path logic works, but maintainability risk increases as validation and integrity layers are added. This packet tightens internal structure without expanding user-visible scope.
- Known bug target 1: vault lock cleanup currently risks masking the primary action error when `flock(Unlock)` also fails.
- Known bug target 2: enclave error handling currently uses hardcoded OSStatus `-26276` magic number; readability/maintainability improve with a named constant.
- Refactor target 3: `resolve_icebox_home` is duplicated in `agent.rs` and `vault.rs` with different error types, creating drift risk.
- Refactor target 4: `config::load_or_default_with_repair` is invoked multiple times in register flows (`has_agent_name` + append path), increasing I/O and subtle state/race risk.
- Known gap target 5: `add` currently accepts secret value as `String` and passes it through multiple layers; plaintext memory lifetime hardening (`secrecy::Secret`/`Zeroize`) is deferred to E3-07.
- Refactor target 6: `runner::RunRequest` scaffolding is currently unused by runtime flows; keep module boundary but remove dead request API until `run` is implemented.

## Scope

- In scope:
  - Refactor vault lock lifecycle internals to keep lock acquire/hold/release flow explicit and easier to audit.
  - Normalize lock/open/unlock error mapping paths for deterministic behavior.
  - Fix error-propagation ordering so action errors are preserved even when unlock fails.
  - Replace enclave OSStatus magic number usage with a named constant (or framework constant when available) plus short meaning note.
  - Consolidate `resolve_icebox_home` into shared path utility to avoid duplicated environment/home resolution logic.
  - Reduce repeated config load/repair calls in register flow by using a single repaired config context per operation where practical.
  - Remove or stub unused runner request scaffolding while preserving `runner` module structure for later E4 wiring.
  - Preserve current runtime semantics and user-visible outputs for E3-10/11/12 paths.
- Out of scope:
  - New user-facing vault commands.
  - New error-code taxonomy changes outside mapped packet scope.
  - New cryptographic behavior.
  - Secret-memory hardening changes (`secrecy::Secret`, `Zeroize`, buffer pinning), which remain tracked under E3-07.

## Acceptance Criteria

- AC1: E3-29 refactor preserves current lock semantics and runtime behavior for concurrent add and lock-open failure paths.
- AC2: Error behavior remains deterministic and user-safe after refactor.
- AC2a: If vault action and unlock both fail, the returned error preserves the original action failure (unlock handled as best-effort or secondary diagnostic, but does not replace primary cause).
- AC2b: Enclave `-26276` handling no longer uses bare magic number in code; named constant (or equivalent) is used with clear intent.
- AC2c: Home-path resolution behavior is defined in one shared utility (no duplicated resolver logic across agent/vault modules).
- AC2d: Register flow avoids redundant config load/repair passes for duplicate-check + append path (single repaired config context or equivalent), while preserving current user-visible behavior.
- AC2e: Runner module remains present, but unused `RunRequest` scaffolding is removed/stubbed so no dead request API is exposed before `run` command implementation.
- AC3: Changes are validated with mapped tests and non-regression checks.

## Rust Implementation Plan

- Crate/module touch points:
  - `src/vault.rs`
  - `src/enclave.rs`
  - `src/agent.rs`
  - `src/config.rs`
  - `src/util.rs`
  - `src/runner.rs`
  - `tests/e2e/e3_12_file_locking.rs`
- Keep interfaces explicit:
  - prefer small pure helpers where possible.
  - avoid hidden global state.
- Error handling:
  - keep deterministic `Result<T, E>` paths.
  - avoid `unwrap()` / `expect()` in non-test code.
- I/O behavior:
  - preserve existing lock and atomic-write semantics.

## Security/Runtime Notes

- No reduction in fail-closed behavior.
- No change to secret-handling boundaries.
- Preserve user-safe default error output.

## Test Mapping

- Linked tests from `docs/plan/TESTING.md`:
- T-E3-29
- Add at least:
  - one happy-path non-regression test
  - one failure-path non-regression test

## ADR Triage

- ADR required? (no):
- Rationale: internal refactor with no new cross-feature architecture commitment.

## Docs Impact

- [x] docs/plan/spec/PKT-E3-29-work-item.md
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
*Last updated: 2026-03-04*
