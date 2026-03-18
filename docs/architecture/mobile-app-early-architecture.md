# Mobile App (Early Architecture)

## Purpose

Define the earliest architecture contract for a future Flutter mobile app so backlog slicing can proceed against stable boundaries.

## Mental Model

Treat the mobile app as a remote control for Icebox, not as the vault/key holder.

## Scope (Current Stage)

- Mobile app is a thin client to Icebox broker/CLI-host services.
- Mobile does not become a key-management authority in MVP.
- Mobile does not directly decrypt vault secrets in MVP.

## Non-Goals (Current Stage)

- No on-device plaintext secret viewing UX.
- No direct Secure Enclave/keychain cryptography in mobile app logic.
- No offline-first signing/decryption flows.

## Trust Boundaries

- Mobile app:
  - Untrusted for long-term secret material.
  - Trusted only for authenticated user intent, approvals, and control-plane actions.
- Local broker/host runtime:
  - Trusted boundary for policy checks, vault access control, and command execution orchestration.
- Identity device/enclave host:
  - Trusted boundary for protected key operations.
  - This is not automatically the phone. In current MVP direction, it is typically the Mac/host running Icebox.

## Pairing And Sign-In

- Pairing:
  - First-time enrollment that establishes trust between the mobile app and Icebox host.
  - Creates a revocable device relationship.
- Sign-in:
  - Routine authentication for an already paired device.
  - Grants a short-lived session/lease to perform allowed actions.

## Why Pair/Sign-In Exists

- Ensure only authorized devices can issue mobile control actions.
- Ensure actions are tied to an authenticated user/session.
- Prevent unpaired devices from triggering broker operations.

## Allowed Mobile Operations (MVP-Aligned)

- Pair/authenticate with broker.
- Read non-sensitive status:
  - active agent
  - registered agent list
  - service inventory names
  - health/diagnostic state
- Submit control-plane actions:
  - request add/update/remove operations via broker APIs
  - approve/deny protected operations (when approval flow is enabled)

## Disallowed Mobile Operations (MVP-Aligned)

- Request or display raw secret plaintext values.
- Display partial secret previews/masked-value snippets.
- Export private key material.
- Bypass broker policy or enclave-gated operations.

## API Contract Expectations

- Contract phases:
  - Phase M0 (pre-broker, optional): mobile may use a minimal local host API for read-only status surfaces (agent list, active agent, service-name inventory, health).
  - Phase M1 (brokered): mobile integrates against brokered request/response contracts for control-plane operations (approve/deny, add/update/remove, protected actions).
- Brokered-only rule applies to mutating/protected operations.
- Error handling must use stable `ICE-*` codes and safe default messages.
- Unknown operation/version responses must fail safely (no silent fallback).

## Pairing/Auth Model (Early Contract)

- Device pairing is explicit and revocable.
- Session tokens/leases are short-lived and scoped.
- Sensitive operations require fresh authorization state from broker policy.

## Data Handling Contract

- Mobile stores minimal metadata only (agent labels, service names, diagnostics).
- No durable storage of secret plaintext.
- Logs/analytics must never include secret values.

## Deferred Capabilities

- Push-driven approvals (this is the threshold where mobile approval UX becomes truly native for brokered execution flows).
- Offline queue/replay model.
- Stronger device attestation.
- Expanded provenance/audit metadata on mobile actions.

## Backlog Gating

Start mobile backlog execution after core CLI/broker contracts stabilize (post-E5 baseline), then add mobile packets against this contract.

- Practical sequencing:
  - If broker (E8) is not yet available, only run Phase M0 read-only packet scope.
  - Start Phase M1 control-plane packet scope only when broker contract is available.

## OpenClaw/Gateway Integration Note

- Mobile should not connect to both Icebox broker and OpenClaw control endpoints.
- Preferred model:
  - mobile -> Icebox broker (single control plane)
  - Icebox broker -> OpenClaw/Gateway (downstream execution/secret-consumer plane)

## OpenClaw Secrets Feature Mapping And User Flow

- OpenClaw secrets features (for example secret-reference/proxy-based runtime resolution) remain valid as downstream runtime plumbing.
- Icebox is the protected secret authority in the migrated scope; OpenClaw consumes resolved values/refs from brokered flows.
- User flow (target state):
  1. User pairs/signs in on mobile.
  2. User selects agent/service action in mobile.
  3. Mobile submits request to Icebox broker.
  4. Icebox enforces policy/approval and resolves secret material.
  5. Icebox invokes downstream OpenClaw execution path using brokered credentials/refs.
  6. Mobile receives status/result metadata only (no plaintext secret payloads).

## OpenClaw OSS Contribution Strategy

- Credibility-first sequence:
  1. Start with a docs/integration PR (no full mobile app dependency).
  2. Follow with small additive code PRs once contracts are validated.
- Positioning in upstream docs/PRs must be explicit:
  - Icebox is an optional external authority mode for stricter policy/approval flows.
  - OpenClaw native secrets flow remains valid and supported.
  - Integration is not "mobile directly replacing OpenClaw secrets"; it is brokered authority + downstream runtime consumption.

## OpenClaw Onboarding Mode (Future)

- When integration matures, onboarding should expose a clear mode choice:
  - OpenClaw native secrets
  - External authority (Icebox)
- This selection should be introduced only after integration contracts and migration guidance are stable.

## Candidate OpenClaw Code PR Scope (Post-Docs)

- Small, additive, flag-gated changes only in early phases:
  - external-authority resolver/provider hook
  - improved `pending_approval` / external-approval runtime messaging
  - migration/drift reporting helpers to reduce dual source-of-truth risk

## Source-Of-Truth And Migration Note

- Without migration, teams risk dual source-of-truth (`legacy gateway secrets` + `Icebox`).
- For rollout, define an explicit authority policy per service (`icebox` or `legacy`) and cut over to one authority.
- Minimal migration/cutover flow:
  - inventory existing gateway secret refs
  - dry-run diff report (`legacy-only`, `icebox-only`, `both-different`)
  - one-way import into Icebox for chosen scope
  - freeze new writes to legacy source for migrated scope
  - switch runtime reads to Icebox-first for migrated scope

## Strategic Adoption Note

- Adoption risk:
  - If OpenClaw gateway/runtime secret resolution already meets a team or solo developer need, a standalone mobile app may be perceived as optional friction.
- Strategic differentiator:
  - Mobile value is approval/authorization UX for protected operations (especially `pending_approval`/strict-mode paths), not secret transport/storage.
- Validation gate before full Flutter investment:
  1. Observe meaningful real-world volume of broker `pending_approval` events.
  2. Confirm team/compliance demand for explicit mobile approvals.
  3. Confirm operational need for off-device inventory/rotation flows.
- Recommended staged rollout:
  - Stage 1: thin approval surface (for example push-style approval integration) to validate demand.
  - Stage 2: full Flutter app once approval loop value is demonstrated.

## Introduction Slice

- Recommended start point: after E5 baseline completion and stabilization.
- Suggested first mobile packet scope:
  - pairing/sign-in
  - registered agent list + active agent indicator
  - selected-agent service-name inventory (no secret values)
  - basic connectivity/health diagnostics
- Control-plane writes/approvals are explicitly deferred to broker-ready packets.

## Identity Contract Dependencies

- Pairing/session model should align with:
  - `E2-34` device enrollment bindings
  - `E2-35` approval/session states
- Packet authors should treat those as upstream identity contracts for mobile auth/session behavior.

## Related Docs

- [Brokered Credential Execution](brokered-credential-execution.md)
- [Security Model](security-model.md)
- [Errors And Diagnostics](errors-and-diagnostics.md)
- [Identity And Enclave](identity-and-enclave.md)
- [Backlog (E2-34/E2-35)](../plan/BACKLOG.md)

---
*Last updated: 2026-03-04*
