# ADR-0002: Dual-Branch Identity And Device Model

## Status

accepted

## Date

2026-02-24

## Context

Icebox needs both portability and strong local protection:

- Identity semantics (`did`, signing, vault identity roots) must remain portable across enrolled devices.
- Local at-rest protection must remain device-specific and hardware-aware.
- Platform backends differ (Secure Enclave, TPM/TEE, hardware tokens), but product behavior should stay logically consistent.

## Decision

Adopt a two-branch architecture:

- Identity branch (`K_identity`, portable): logical identity roots for DID/signing/vault behavior.
- Device branch (`K_device`, per-device): local wrapping/integrity keys and approval/session state.

Adopt explicit operation lanes:

- `local-enclave` lane for local backend operations (MVP first on macOS).
- `paired-remote-signer` lane for delegated operations (post-MVP).

Session/approval behavior is explicit (`ok`, `pending_approval`, `denied`, `expired`) and lease-based.

## Alternatives Considered

- Single-key local-only model with no portable identity branch.
- Portable key model with no per-device wrapping/integrity branch.

## Consequences

- Positive:
  - Preserves portability goals without collapsing local hardware boundaries.
  - Enables cross-platform backend evolution behind stable logical contracts.
  - Clarifies approval semantics for broker and mobile-assisted flows.
- Negative:
  - Higher schema and workflow complexity (lane/backend metadata, enrollment lifecycle).
  - Additional test matrix and reconciliation requirements across lanes/backends.
- Migration/compatibility impact:
  - Existing records remain valid.
  - Lane/backend fields are introduced as reserved/optional metadata first.
  - Unsupported lanes/backends fail safely and explicitly.

## Links

- Architecture docs:
  - `docs/architecture/identity-and-enclave.md`
  - `docs/architecture/security-model.md`
  - `docs/architecture/data-models-and-layout.md`
  - `docs/architecture/platform-and-distribution.md`
  - `docs/architecture/brokered-credential-execution.md`
- Backlog IDs:
  - `E2-01`, `E2-02`, `E2-03`, `E2-04`, `E2-11`, `E2-29`, `E2-31`
- Test IDs:
  - `T-E2-01`, `T-E2-02`, `T-E2-03`, `T-E2-04`, `T-E2-11`
- Related PR/commit:
  - pending


---
*Last updated: 2026-02-24*
