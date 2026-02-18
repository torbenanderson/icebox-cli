# E1-04 Execution Spec

## Objective

- Deliver E1-04 (CI pipeline).
- Backlog contract: GitHub Actions: macOS runner (full tests incl. enclave) + Linux runner (vault/crypto/DID/CLI tests only; enclave code excluded via `#[cfg]`). Run merge-blocking quality gates on push + pull_request, plus scheduled security audit.

## Problem

- Why this exists: implement the backlog contract in a way that is testable, deterministic, and easy to extend.

## Scope

- In scope:
  - E1-04 core (backlog contract): GitHub Actions on macOS + Linux running merge-blocking build/test/lint gates on push + pull_request (`cargo check`, `cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings`).
  - E1-04 enhancements (non-blocking for initial pass closeout):
    - scheduled CI security audit via `cargo audit` (weekly)
    - dependency update automation via Dependabot (weekly batched minor/patch)
    - non-blocking coverage trend signal via `cargo llvm-cov`
    - pull-request-only optional metadata checks (title/body/Refs linkage)
  - Post-scope addition (tracked in packet issue closeout):
    - Linux mutation testing via `cargo mutants` in a dedicated weekly workflow to cover Linux-only compiled paths (including `enclave_stub`).
- Out of scope:
  - Unrelated backlog items outside E1-04
  - Cross-epic behavior changes not requested by E1-04

## Acceptance Criteria

- AC1: E1-04 behavior matches backlog description: GitHub Actions: macOS runner (full tests incl. enclave) + Linux runner (vault/crypto/DID/CLI tests only; enclave code excluded via `#[cfg]`). Run merge-blocking quality gates on push + pull_request, plus scheduled security audit.
- AC2: CLI output/errors are deterministic and user-safe.
- AC3: Changes are validated with mapped tests.
- AC4 (enhancement): Security dependency audit is integrated into scheduled CI (`cargo audit`) and reports failures on known vulnerabilities.
- AC5 (enhancement): Dependabot is configured for Rust and GitHub Actions with weekly batched minor/patch updates.
- AC6 (enhancement): Coverage trend collection uses `cargo llvm-cov` and remains non-blocking in this initial pass.
- AC7 (enhancement): Trigger policy is explicit: push + pull_request run merge-blocking quality gates; pull_request-only metadata checks are optional; schedule runs `cargo audit`.
- AC8 (enhancement): CI artifacts/logs include quality outputs and coverage summary with a defined retention window.
- AC9 (post-scope addition): Linux CI runs `cargo mutants` in a dedicated weekly (scheduled) job so mutation coverage includes Linux-only compiled paths such as `enclave_stub` without slowing per-PR gates.

## CI Trigger Policy

- `push` + `pull_request`: run build/test/format/lint (`cargo check`, `cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings`) as merge-blocking checks.
- `pull_request` only: run optional PR metadata checks (title/body/reference hygiene) as non-blocking informational checks.
- `schedule` (weekly): run `cargo audit`; avoids blocking every push on advisory DB slowness while still surfacing dependency risk.
- `schedule` (weekly): run dedicated Linux mutation testing via `cargo mutants` in separate workflow.

## Runner Boundaries

- `macos-latest`: run full configured Rust quality/test suite, including macOS-gated code paths.
- `ubuntu-latest`: run the same quality/test suite but with `#[cfg(target_os = "macos")]` paths excluded by compilation target.
- Linux jobs must not attempt enclave-only runtime tests; macOS jobs are the canonical runner for enclave coverage.
- Linux mutation testing is the canonical mutation gate for `enclave_stub` and other Linux-compiled paths.

## Artifacts And Retention

- Upload CI artifacts needed for debugging:
  - on failure (merge-blocking jobs): failing test/log outputs
  - always (coverage trend job): coverage summary (`cargo llvm-cov --summary-only` output)
- Keep retention explicit in workflow config (default target: 90 days) and adjust only with team agreement.
- Coverage artifact is informational in E1-04 and must not fail the workflow.

## CI Implementation Plan

- Workflow/config touch points:
  - `.github/workflows/ci.yml`
  - `.github/workflows/ci-enhancements.yml`
  - `.github/workflows/security-audit.yml`
  - `.github/workflows/docs-site.yml` (exists today; touch only if workflow coupling requires updates)
  - `.github/workflows/docs-schemas.yml` (exists today; touch only if workflow coupling requires updates)
  - `.github/dependabot.yml`
  - `Cargo.toml` / `Cargo.lock` (only if CI tooling dependency updates are required)
- Implementation constraints:
  - keep merge-blocking and non-blocking checks clearly separated by job and trigger
  - use explicit workflow names/job names for stable branch protection rules
  - keep scheduled audit isolated from push/PR gates
  - keep Linux jobs free of enclave-only runtime expectations

## Security/Runtime Notes

- Keep secret-handling boundaries unchanged unless explicitly in scope.
- Preserve direct-exec/no-shell guarantees where relevant.
- Preserve user-safe default errors (no sensitive internals in normal mode).

## Test Mapping

- Linked tests from `docs/plan/TESTING.md`:
- `T-E1-04`: CI workflows validate push/PR gates on macOS and Linux with pass/fail signaling.
- Add at least:
  - happy path: merge-blocking CI jobs pass (`check`, `fmt`, `clippy -D warnings`, `test`) on configured runners; scheduled Linux mutation testing (`cargo mutants`), scheduled `cargo audit`, and non-blocking `llvm-cov` trend run as configured
  - failure path: one merge-blocking quality gate turns push/PR workflow red and blocks merge; scheduled mutation/audit failures are reported via their scheduled workflows

## ADR Triage

- ADR required? (no):
- Rationale: keep in spec unless long-lived cross-feature decision exists.

## Docs Impact

- [x] docs/plan/spec/PKT-E1-04-work-item.md
- [x] docs/plan/TESTING.md (if test mappings are added/changed)
- [ ] docs/architecture/decisions/ADR-*.md (if ADR required)
- [ ] docs/README.md (if user-facing behavior changed)

## Validation Commands

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- `cargo check`
- `cargo audit` (scheduled workflow)
- `cargo llvm-cov --workspace --all-features --summary-only` (non-blocking trend signal)
- `cargo mutants --all-targets --all-features` (Linux CI mutation gate)
- `cargo build` is optional as an explicit build-only gate; `cargo test` already compiles test targets.

## Execution Notes

- Commit split plan will be finalized in the issue `Execution Plan` comment during `execute`.

---

*Last updated: 2026-02-18*
