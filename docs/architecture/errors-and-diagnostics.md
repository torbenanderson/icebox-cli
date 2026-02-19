# Errors & Diagnostics

## Error Strategy

- Minimal safe messages by default.
- Structured `ICE-xxx` codes mapped by root cause.
- Debug mode can include additional internal context.

## Phasing

- MVP source of truth is runtime code mapping (typed errors/code map in `src/`).
- `docs/reference/error-codes.json` is introduced later when external consumers (docs/support/tooling) need a machine-readable artifact.
- When JSON is introduced, add a sync test between runtime code mapping and registry before any code generation step.

## Key Codes

- `ICE-201`: vault parse/corruption failures (invalid JSON/truncated payloads).
- `ICE-202`: unsupported or invalid schema/version.
- `ICE-205`: vault entry-structure/validation failures (entry shape/uniqueness violations).
- `ICE-203`: rollback (`seq` stale).
- `ICE-204`: integrity/HMAC mismatch.

## Related Docs

- `../reference/error-codes.json`
- `vault-and-integrity.md`


---

*Last updated: 2026-02-19*
