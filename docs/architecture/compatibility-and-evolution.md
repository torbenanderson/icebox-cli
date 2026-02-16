# Compatibility & Evolution

This document defines non-negotiable compatibility guardrails to reduce breaking changes as identity and portability features expand.

## Core Guarantees

1. Every persisted artifact declares `format` and `schemaVersion`.
2. Unknown fields are preserved unless a migration explicitly removes them.
3. Canonical serialization is stable across implementations.
4. Extension namespaces are reserved and non-conflicting.
5. Creation/migration provenance is persisted in metadata.
6. Compatibility support window is explicit and documented.
7. Import/export validation is deterministic and fail-fast.
8. Algorithm negotiation uses explicit states (preferred/allowed/deprecated).
9. Deprecations follow a documented lifecycle.
10. Conformance fixtures are treated as merge gates.

## Required Metadata

- `format`: logical artifact type (`icebox.manifest`, `icebox.vault`, `icebox.bundle`)
- `schemaVersion`: integer schema version for that artifact
- `createdByVersion`: Icebox version that created the artifact
- `lastMigratedByVersion`: Icebox version that last migrated it (`null` if never migrated)

## Canonical Serialization Rules

- UTF-8 JSON only.
- Timestamps in UTC RFC3339.
- Hashes lowercase hex.
- Binary payloads base64 with a single chosen alphabet (documented once and reused).
- Deterministic field ordering in exported artifacts.

## Extension Namespace Policy

- Internal experiments: `x_icebox_*`
- Vendor/integration extensions: `x_vendor_*`
- Unknown extension fields must round-trip untouched.

## Compatibility Support Window

- `N` (current): read/write
- `N-1`: read/write
- `N-2`: read-only with migration path warning
- `< N-2`: unsupported (`ICE-202`)

## Algorithm Negotiation Policy

- Every algorithmic field must support:
  - `preferred`
  - `allowed`
  - `deprecated`
- Deprecated algorithms remain readable for at least one full minor cycle before removal.

## Deterministic Import/Export Validation

- Reject on checksum mismatch.
- Reject on unsupported required fields/algorithms.
- Reject duplicate IDs (`agentId`, `entryId`) within a bundle.
- Reject conflicting agent records unless explicit recovery flow is selected.

## Deprecation Lifecycle

1. Introduced
2. Soft-deprecated (warning only)
3. Hard-deprecated (requires explicit override)
4. Removed

Each stage must specify minimum release counts before advancing.

## Related Docs

- `data-models-and-layout.md`
- `vault-and-integrity.md`
- `../plan/BACKLOG.md`
- `../plan/TESTING.md`


---

*Last updated: 2026-02-16*
