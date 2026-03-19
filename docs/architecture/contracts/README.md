# Architecture Contracts

This folder holds planned and target-state machine-readable contracts for Icebox persisted artifacts.

These files are architecture-level design artifacts, not the current runtime persistence surface.

## Contents

- `manifest.schema.json`
- `config.schema.json`
- `vault.schema.json`
- `bundle-manifest.schema.json`
- `examples/`

## Usage

- Use these schemas when discussing or validating target contract design.
- Do not treat them as the current runtime truth unless the implementation and current-state docs explicitly say so.
- Current authoritative machine-readable runtime artifacts live under `docs/reference/`.

## Validation

CI validates:

- Each schema against the JSON Schema metaschema.
- Each example fixture against its matching schema.

Workflow file:

- `.github/workflows/docs-schemas.yml`


---

*Last updated: 2026-03-18*
