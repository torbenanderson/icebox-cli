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
- `ICE-301`: agent not found.
- `ICE-302`: missing capability for requested broker operation.
- `ICE-303`: destination/action not allowlisted by broker policy.
- `ICE-304`: broker identity/attestation/authentication failure.
- `ICE-305`: unsafe raw-secret mode blocked by policy.
- `ICE-306`: identity setup failure.
- `ICE-307`: approval pending.
- `ICE-308`: approval session expired.
- `ICE-309`: invalid config.

## Related Docs

- `../reference/error-codes.json`
- `vault-and-integrity.md`


---

*Last updated: 2026-03-03*
