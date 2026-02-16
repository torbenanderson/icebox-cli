# Vault & Integrity

## Vault Model

- Per-agent `vault.enc` file.
- Per-agent `hmac.enc` file stores enclave-wrapped HMAC key material used for vault integrity checks.
- Independent sealed entries keyed by service.
- Each entry has immutable `entryId` for stable identity across updates/migrations.
- Versioned schema with monotonic `seq`.

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

*Last updated: 2026-02-16*
