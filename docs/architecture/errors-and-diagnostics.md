# Errors & Diagnostics

## Error Strategy

- Minimal safe messages by default.
- Structured `ICE-xxx` codes mapped by root cause.
- Debug mode can include additional internal context.

## Phasing

- MVP source of truth is runtime code mapping (typed errors/code map in `src/`).
- `docs/reference/error-codes.json` is the current machine-readable runtime registry for external consumers.
- Planned post-MVP codes can be described here before they are added to the runtime registry.

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
- `ICE-307`: duplicate agent name.
- `ICE-308`: invalid agent name.
- `ICE-309`: invalid config.
- `ICE-310`: duplicate config agent names.
- `ICE-311`: Secure Enclave operation failure.

## MVP Concession (Vault Codes)

- Current MVP runtime mapping intentionally collapses multiple vault/precondition failures to `ICE-201` for stability and lower implementation churn in the first vault slice.
- This currently includes parse/corruption plus setup/precondition failures in early vault operations (for example missing active agent or missing identity public-key artifact).
- This is temporary and should be treated as an MVP compatibility behavior, not the long-term code taxonomy.

## Planned Post-MVP Split

- Target: post-MVP hardening release (tracked in backlog).
- Planned direction:
  - `ICE-201`: parse/corruption and low-level vault read failures.
  - `ICE-202`: schema/version/unsupported-format failures.
  - `ICE-205`: entry-level structure/uniqueness validation failures.
  - `ICE-206` (planned): vault preconditions/missing dependencies (for example no active agent, missing identity artifacts).
- Implementation shape: typed vault validation/precondition errors in vault module(s), mapped to final `ICE-2xx` codes at CLI/runtime boundary.
- Add new machine-readable codes to `docs/reference/error-codes.json` only when they exist in runtime.

## Debug Detail Policy

- Default mode stays user-safe and minimal.
- `--debug` may expose underlying error class/cause (for example `MissingActiveAgent`) for diagnosis.
- `--debug` must still avoid leaking secret material; sensitive filesystem detail should remain redacted/minimized unless explicitly required for support workflows.

## Related Docs

- `../reference/error-codes.json`
- `vault-and-integrity.md`


---

*Last updated: 2026-03-18*
