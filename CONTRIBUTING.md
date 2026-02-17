# Contributing to Icebox

Thanks for contributing.

## Current Project State

- Icebox is currently in planning/docs-first stage.
- Much of the Rust implementation is not in `src/` yet.
- Architecture and planning docs are the implementation contract for MVP.

Start here:

- `docs/README.md`
- `docs/architecture/README.md`
- `docs/plan/README.md`
- `docs/reference/DOCS_GOVERNANCE.md`

## Ways to Contribute

- Clarify architecture and security docs.
- Improve backlog/test mapping consistency.
- Add schemas, fixtures, and docs validation improvements.
- Implement MVP backlog items once scoped in issues/PRs.

## Contribution Workflow

1. Fork the repo on GitHub
2. Clone your fork locally
3. Create a feature branch (`git checkout -b my-feature`)
4. Make your changes and commit
5. Push to your fork (`git push origin my-feature`)
6. Open a Pull Request to `main`
7. Address review feedback
8. Maintainer merges

## Before You Open a PR

1. Check the relevant architecture and planning docs first.
2. Keep changes scoped to one concern (for example one backlog item or one docs topic).
3. Update linked docs when behavior/contracts change.
4. If persisted artifact behavior changes, update schemas and fixtures.

## Development and Test Expectations

- Preferred local platform for full-flow testing: macOS.
- Linux contributors can work on non-enclave paths (CLI/config/vault/crypto/schemas/docs).
- Keep CI green:
  - `cargo fmt --check`
  - `cargo clippy -- -D warnings`
  - `cargo test`

If code does not exist yet for a planned area, contribute docs/tests/contracts first.

## PR Guidelines

1. Use a clear title with subsystem scope, for example:
   - `docs: expand security model threat assumptions`
   - `plan: align rollback tests with release gate`
2. Describe:
   - what changed,
   - why it changed,
   - which docs/backlog/tests are affected.
3. Include file references in the PR body for decision-impacting changes.
4. Keep security-sensitive changes explicit; avoid hidden behavior changes.

## Security-Sensitive Changes

For changes affecting crypto, key handling, vault integrity, `run`, or permissions:

- call out threat-model impact in the PR description,
- list added/updated tests,
- include fail-closed behavior expectations.

For vulnerability reports, do not open a public issue. See `SECURITY.md`.
