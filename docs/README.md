# Documentation Index

Use this file as the entrypoint for project docs.

## Architecture

- [Architecture Index](architecture/)
- [Overview](architecture/overview.md)
- [Identity And Enclave](architecture/identity-and-enclave.md)
- [Vault And Integrity](architecture/vault-and-integrity.md)
- [Secret Management And Run](architecture/secret-management-and-run.md)
- [Brokered Credential Execution](architecture/brokered-credential-execution.md)
- [Security Model](architecture/security-model.md)
- [Errors And Diagnostics](architecture/errors-and-diagnostics.md)
- [Data Models And Layout](architecture/data-models-and-layout.md)
- [Platform And Distribution](architecture/platform-and-distribution.md)
- [Compatibility & Evolution](architecture/compatibility-and-evolution.md)
- [Technology Choices](architecture/technology-choices.md)
- [MVP Decision Lock](architecture/mvp-decision-lock.md)
- [Architecture Decision Log](architecture/decisions/)
- [ADR-0002 Dual-Branch Identity And Device Model](architecture/decisions/ADR-0002-dual-branch-identity-device-model.md)
- [Mobile App (Early Architecture)](architecture/mobile-app-early-architecture.md)
- [Rust Implementation Notes](architecture/rust-implementation.md)
- [Architecture Contracts](architecture/contracts/README.md)

## Planning

- [Planning Index](plan/)
- [Current State](plan/CURRENT_STATE.md)
- [Status Model](plan/STATUS_MODEL.md)
- [Roadmap](plan/ROADMAP.md)
- [Backlog](plan/BACKLOG.md)
- [Implementation Bootstrap](plan/IMPLEMENTATION_BOOTSTRAP.md)
- [CI Process](plan/CI.md)
- [Bootstrap Issues](plan/BOOTSTRAP_ISSUES.md)
- [Testing Plan](plan/TESTING.md)

## Maintenance

- [Maintenance Index](maintenance/)
- [Calendar](maintenance/CALENDAR.md)
- [Log](maintenance/LOG.md)
- [Dependencies](maintenance/DEPENDENCIES.md)
- [Refactor Backlog](maintenance/REFACTOR_BACKLOG.md)

## Guides

- [Backup & Recovery](guides/BACKUP.md)

## Process

- [Process Index](process/README.md)
- [Discussion Proposals](process/DISCUSSION_PROPOSALS.md)
- [Discussion Log](process/DISCUSSION_LOG.md)

## Reference

- [Versioning Policy](reference/VERSIONING.md)
- [API Documentation (Rustdoc)](reference/API_DOCS.md)
- [Error Codes (JSON)](reference/error-codes.json)
- [Documentation Governance](reference/DOCS_GOVERNANCE.md)

## Documentation Surfaces

- Docs book (mdBook): built from `docs/` using `book.toml`
- API docs (rustdoc): built from Rust `///` comments when `Cargo.toml` exists

### Local Build Commands

```bash
mdbook build
cargo doc --workspace --all-features --no-deps
```

## Source Of Truth

- Architecture behavior and contracts: `docs/architecture/`
- Architecture rationale and locked decisions: `docs/architecture/mvp-decision-lock.md`
- Planning/execution sequencing: `docs/plan/`
- User/operator workflows: `docs/guides/`
- Current machine-readable and policy artifacts: `docs/reference/`
- Planned persisted-artifact contracts: `docs/architecture/contracts/`
- Recurring maintenance: `docs/maintenance/`


---

*Last updated: 2026-03-18*
