# CI Process

This document defines the repository CI execution model for E1-04.

## E1-04 Scope Split

- Core (merge-blocking baseline)
  - `push` + `pull_request` quality gates: `cargo check`, `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`
- Enhancements (initially non-blocking)
  - Weekly scheduled `cargo audit`
  - Dependabot updates for Rust + GitHub Actions (weekly, batched minor/patch)
  - Coverage trend reporting via `cargo llvm-cov --summary-only`
  - Optional pull-request metadata checks (title/body/reference hygiene)

## Trigger Matrix

- `push` + `pull_request`
  - Merge-blocking quality gates:
    - `cargo check`
    - `cargo fmt --check`
    - `cargo clippy -- -D warnings`
    - `cargo test`
- `pull_request` only
  - Optional non-blocking metadata checks (PR title/body/reference hygiene)
- `schedule` (weekly)
  - Security audit:
    - `cargo audit`

## Runner Boundaries

- `macos-latest`
  - Full configured Rust quality/test suite, including macOS-gated code paths
- `ubuntu-latest`
  - Same merge-blocking quality/test suite with `#[cfg(target_os = "macos")]` code excluded by target compilation
- Linux jobs must not attempt enclave-only runtime tests.

## Blocking Policy

- Merge-blocking checks run on `push` and `pull_request`.
- Scheduled security audit is reported separately and does not block `push`/`pull_request`.
- Coverage trend reporting is informational and non-blocking in this initial pass.

## Artifacts And Retention

- Upload failure diagnostics for merge-blocking jobs when they fail.
- Upload coverage summary artifact for trend tracking.
- Artifact retention target: 90 days.

## Failure Handling

- Merge-blocking job failure: fix or revert before merge.
- Scheduled audit failure: triage advisory severity, create backlog follow-up, and patch/update dependency as needed.
- Coverage movement: track trend; do not fail pipeline solely on coverage in E1-04.

## Related Files

- `.github/workflows/ci.yml`
- `.github/workflows/ci-enhancements.yml`
- `.github/workflows/security-audit.yml`
- `.github/workflows/docs-site.yml`
- `.github/workflows/docs-schemas.yml`
- `.github/dependabot.yml`


---

*Last updated: 2026-02-18*
