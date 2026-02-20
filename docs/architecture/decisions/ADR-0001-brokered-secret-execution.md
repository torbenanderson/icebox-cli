# ADR-0001 Brokered Secret Execution Boundary

- Status: proposed
- Date: 2026-02-20

## Context

Current MVP `run` design injects secret material into subprocess context. This is acceptable only for trusted-by-user subprocess usage and remains a documented residual risk for exfiltration when execution targets are untrusted.

Future socket/skill integrations introduce higher-risk execution surfaces where untrusted or partially trusted clients may request operations. In that model, long-lived plaintext credential exposure to client code is an unacceptable default.

## Decision

Adopt brokered secret execution as the default post-MVP trust boundary:

1. Untrusted clients request approved operations; they do not receive long-lived plaintext credentials.
2. Operation authorization is deny-by-default and policy gated by capability + action + destination constraints.
3. Provider-facing auth should prefer short-lived delegated credentials scoped by audience, TTL, and operation intent.
4. Raw secret injection is quarantined as explicit unsafe mode and is disabled by default.

## Consequences

Positive:

1. Reduces credential exfiltration blast radius for agent/skill integrations.
2. Moves security control plane from warning-only posture to enforceable policy.
3. Creates a clear foundation for auditable least-privilege execution.

Tradeoffs:

1. Requires broker/proxy interfaces and policy management UX.
2. Increases integration complexity for clients that currently expect direct secret access.
3. Some providers may not support delegated-token exchange and will need controlled fallback behavior.

## Related Backlog And Tests

- Backlog:
  - `E8-01`, `E8-02`, `E8-03`, `E8-04`, `E8-05`
  - `E9-01`, `E9-02`, `E9-03`
- Tests:
  - `T-E8-01` through `T-E8-05`
  - `T-E9-01` through `T-E9-03`
  - `T-SEC-21` through `T-SEC-24`

## Related Docs

- `docs/architecture/security-model.md`
- `docs/architecture/secret-management-and-run.md`
- `docs/plan/BACKLOG.md`
- `docs/plan/TESTING.md`

