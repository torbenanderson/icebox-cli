# Vault & Integrity

## Vault Model

- Per-agent `vault.enc` file.
- Per-agent `hmac.enc` file stores enclave-wrapped HMAC key material used for vault integrity checks.
- Independent sealed entries keyed by service.
- Each entry has immutable `entryId` for stable identity across updates/migrations.
- Versioned schema with monotonic `seq`.
- Recovery boundary (MVP): encrypted vault contents are non-recoverable without the corresponding identity private key material.

## Vault Envelope (v1)

The envelope allows format evolution without breaking old vaults. Machine-readable schema: [vault.schema.json](../reference/schemas/vault.schema.json).

MVP note:

- Current runtime write format is a minimal legacy envelope marker: `format: "icebox.vault.legacy-v1"` with `version` and `entries`.
- The full schema envelope shown below (`format: "icebox.vault"` plus `schemaVersion`, `seq`, `createdByVersion`, etc.) is the target contract for migration/hardening phases.
- Migration work should treat missing/legacy markers as deterministic upgrade inputs (see E3-20 planning notes).

**Top-level fields:**

| Field | Purpose |
|-------|---------|
| `format` | Constant `"icebox.vault"` — format identifier |
| `schemaVersion` | Schema contract version (integer, ≥1) |
| `version` | Envelope/data version for migrations |
| `seq` | Monotonic sequence for rollback detection (`ICE-203`) |
| `hmac` | Hex HMAC-SHA256 tag over vault body (excludes `hmac` field) |
| `entries` | Array of sealed secret entries |
| `createdByVersion` | CLI version that created this vault |
| `lastMigratedByVersion` | CLI version that last migrated (null if never) |

**Entry fields:**

| Field | Purpose |
|-------|---------|
| `entryId` | Immutable UUID/ULID — stable across renames/updates |
| `service` | Service label (e.g. `openai`, `aws`) — unique per vault |
| `sealedBlob` | Base64 ciphertext from `crypto_box_seal` |
| `created` | RFC3339 timestamp when entry was added |
| `updated` | RFC3339 or null — set on value change |

**Example (minimal):**

```json
{
  "format": "icebox.vault",
  "schemaVersion": 1,
  "version": 1,
  "seq": 0,
  "hmac": null,
  "entries": [],
  "createdByVersion": "0.1.0",
  "lastMigratedByVersion": null
}
```

Empty vault: `seq: 0`, `hmac: null`, `entries: []`. First secret adds one entry and sets `hmac` after HMAC key exists (E3-16).

Optional identity self-description (`identity_pubkey`, etc.) is deferred per [ADR-0003](decisions/ADR-0003-defer-vault-reseal-and-provenance.md).

## How Sealed Entries Work

Icebox does not create or store a separate long-term key per secret entry.

- Each identity has one long-term keypair used for vault sealing/unsealing operations.
- On each `crypto_box_seal` call, libsodium creates a fresh ephemeral keypair for that entry.
- The ephemeral public key is embedded in the sealed blob; the ephemeral private key is discarded after sealing.
- At unseal time, Icebox uses the identity private key plus the embedded ephemeral public key to derive the shared key and decrypt.

This gives per-entry cryptographic isolation while keeping operational key management simple: one identity private key unlocks all entries for that identity.

## Write Safety

- File lock during read-modify-write cycles.
- Atomic write via temp + rename.

## Validation Pipeline

- Load-time checks: parse, schema, entry structure, integrity checks, rollback detection.
- Unseal-time checks: AEAD verify and service binding.
- Integrity checks on security-critical paths depend on successful unwrap/use of `hmac.enc`.

### Vault Load Validation Pipeline

Implement one canonical function for load validation and keep checks centralized:

1. JSON parse (`ICE-201`)
2. Schema check (`ICE-202`)
3. Entry structure + uniqueness (`ICE-205`)
4. HMAC/integrity check (`ICE-204`)
5. Sequence rollback check (`ICE-203`)
6. AEAD verification at unseal time
7. Service-binding verification at unseal time

## Migration Contract

- Version upgrades are explicit and deterministic (`from_version` -> `to_version`).
- Only supported upgrade paths run automatically; unsupported versions fail with `ICE-202`.
- Migrations run before validation-dependent operations and write back atomically.
- Migration code must preserve unknown fields unless explicitly removed by the target schema.
- Every migration requires:
  - fixture-based round-trip tests (`vN` -> `vN+1`)
  - idempotency test (running migration twice is a no-op on target version)
  - rollback safety test for interrupted writes

## Integrity Signals

- `ICE-203`: rollback (`seq` monotonicity).
- `ICE-204`: vault integrity/HMAC mismatch.

## Related Docs

- `data-models-and-layout.md`
- `errors-and-diagnostics.md`


---

*Last updated: 2026-03-03*
