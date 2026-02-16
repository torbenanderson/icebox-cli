# JSON Schemas

Machine-validated schema contracts for persisted artifacts.

## Schemas

- `manifest.schema.json`
- `config.schema.json`
- `vault.schema.json`
- `bundle-manifest.schema.json`

## Example Fixtures

- `examples/manifest.example.json`
- `examples/config.example.json`
- `examples/vault.example.json`
- `examples/bundle-manifest.example.json`

## CI Validation

CI validates:

- Each schema against the JSON Schema metaschema.
- Each example fixture against its matching schema.

Workflow file:

- `.github/workflows/docs-schemas.yml`


---

*Last updated: 2026-02-16*
