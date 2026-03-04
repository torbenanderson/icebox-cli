# ADR-0003: Defer Automatic Vault Reseal And Write Provenance In MVP

## Status

`accepted`

## Date

2026-03-03

## Context

- MVP scope prioritizes a minimal, deterministic encrypted vault path (`register-agent` -> `add` -> `run` -> `remove`) with low implementation risk.
- Key-rotation and provenance features introduce additional migration/compatibility complexity:
  - automatic reseal orchestration when identity keys change,
  - provenance metadata/signature semantics,
  - recovery/rollback behavior under partial failures.
- Current vault crypto path uses anonymous-sender sealed-box (`crypto_box_seal`), which is acceptable for local self-managed secret storage in MVP.

## Decision

- For MVP, do **not** implement automatic vault reseal on key rotation.
- For MVP, do **not** add authenticated write provenance/signature layer to sealed entries.
- Keep the vault envelope minimal and versioned (`version: 1` + `secrets`) and defer optional identity self-description metadata.
- Track follow-up work explicitly in backlog:
  - key rotation/reseal remains post-MVP (`E2-22`),
  - envelope metadata/provenance tracked in deferred vault backlog (`E3-22`, `E3-23`).

## Alternatives Considered

- Implement automatic reseal now in MVP.
- Add sender-authenticated provenance metadata now in MVP.
- Add optional `identity_pubkey` and other self-description metadata fields immediately.

## Consequences

- Positive:
  - keeps MVP simpler and easier to validate end-to-end.
  - preserves stable `crypto_box_seal` interoperability without introducing provenance policy coupling.
  - avoids high-risk migration logic before baseline vault behavior is shipped.
- Negative:
  - key rotation cannot transparently preserve existing sealed entries yet.
  - no cryptographic authorship/provenance trail for writes in MVP.
  - backup/recovery UX remains strict: encrypted vault data without matching identity private key material is non-recoverable.
- Migration/compatibility impact:
  - deferred features must be introduced with explicit schema/version migration and idempotent reseal flows.

## Links

- Architecture docs:
  - `../vault-and-integrity.md`
  - `../identity-and-enclave.md`
- Backlog IDs:
  - `E2-22`, `E3-22`, `E3-23`
- Test IDs:
  - `T-E2-22` (future), `T-E3-22` (future), `T-E3-23` (future)
- Related PR/commit:
  - `n/a`


---
*Last updated: 2026-03-03*
