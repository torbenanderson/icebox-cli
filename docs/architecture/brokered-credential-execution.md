# Brokered Credential Execution

This page defines the post-MVP brokered execution contract where agents can use credential references without receiving long-lived credential values.

## Goal

1. Agent can discover credential availability and capabilities.
2. Agent cannot read raw long-lived credential values.
3. Broker resolves credential reference and executes approved operation.
4. Broker returns result data only; no credential material in response.

## Agent-Visible Credential Metadata

Agent-visible credential records must contain metadata only.

Example:

```json
{
  "stripe_api": {
    "type": "api_key",
    "provider": "stripe",
    "hint": "...4f2x",
    "capabilities": ["payments"]
  }
}
```

Constraints:

1. No `value` field is ever returned in metadata responses.
2. `hint` is non-sensitive (for example suffix or fingerprint fragment), used only for operator disambiguation.
3. `capabilities` drives policy checks for allowed operation types.

## Request/Execution Contract

1. Agent requests an operation referencing a credential ID.
2. Broker validates policy (identity, capability, action, destination, constraints).
3. Broker resolves secret material inside trusted boundary.
4. Broker performs operation using credential or delegated token.
5. Broker zeroizes transient sensitive buffers.
6. Broker returns sanitized operation result.

Example request shape:

```json
{
  "operation": "api_call",
  "credentialRef": "stripe_api",
  "target": "api.stripe.com",
  "action": "create_payment_intent",
  "payload": {
    "amount": 1000,
    "currency": "usd"
  }
}
```

Example result shape:

```json
{
  "ok": true,
  "statusCode": 200,
  "result": {
    "payment_intent_id": "pi_***"
  }
}
```

Constraints:

1. Response must never include plaintext secret values.
2. Error payloads must not include secret bytes or raw auth headers.

## Policy Requirements

1. Deny by default.
2. Allowlist by operation, destination, and capability.
3. Explicit unsafe raw-secret path is disabled by default and auditable when enabled.
4. Policy failures return deterministic, user-safe error codes.

## Relationship To MVP `run`

1. MVP `run` remains direct-exec and trusted-subprocess oriented.
2. Brokered execution is the canonical post-MVP path for untrusted/partially trusted agent integrations.
3. Migration should preserve explicit operator control over legacy unsafe modes.

## Related Backlog And ADR

1. `docs/plan/BACKLOG.md` (`E8-01`..`E8-05`, `E9-01`..`E9-03`)
2. `docs/plan/TESTING.md` (`T-E8-*`, `T-E9-*`, `T-SEC-21`..`T-SEC-24`)
3. `docs/architecture/decisions/ADR-0001-brokered-secret-execution.md`

---

*Last updated: 2026-02-20*
