# Documentation Index

Use this file as the entrypoint for project docs.

## Architecture

- [Architecture Index](architecture/)
- [Compatibility & Evolution](architecture/compatibility-and-evolution.md)
- [Architecture Decision Log](architecture/decisions/)
- [Rust Implementation Notes](architecture/rust-implementation.md)

## Planning

- [Planning Index](plan/)
- [Roadmap](plan/ROADMAP.md)
- [Backlog](plan/BACKLOG.md)
- [Implementation Bootstrap](plan/IMPLEMENTATION_BOOTSTRAP.md)
- [CI Process](plan/CI.md)
- [Testing Plan](plan/TESTING.md)

## Guides

- [Backup & Recovery](guides/BACKUP.md)

## Process

- [Merge Message Template](process/MERGE_MESSAGE_TEMPLATE.md)

## Reference

- [Versioning Policy](reference/VERSIONING.md)
- [API Documentation (Rustdoc)](reference/API_DOCS.md)
- [Error Codes (JSON)](reference/error-codes.json)
- [JSON Schemas](reference/schemas/)
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
- Machine-readable and policy artifacts: `docs/reference/`


---

*Last updated: 2026-02-19*
