# MVP Decision Lock

Locked implementation decisions for Phase 1.

## MVP Core (v0.1.0)

- No outbound network from `icebox` process.
- No non-interactive first-run prompt; fail with guidance to `register-agent`.
- `--agent` is one-shot targeting only; use `use-agent` to persist defaults.
- Seed/recovery/export/import are out of MVP.
- Vault validation and error mapping use `ICE-201/202/203/204/205` semantics.
- Subprocess execution is direct exec (no shell).
- Default env injection mode is enabled.
- `--unsafe-substitute` is out of MVP.
- `agentId` (UUID/ULID) is the immutable internal agent identifier; `name` remains a mutable UX label.
- Name-based CLI commands resolve to `agentId` internally.
- Legacy records missing `agentId` are backfilled once and persisted atomically (`activeAgent` -> `activeAgentId`).
- `manifest.json` `type` is enum-based: MVP supports `agent`; unknown types fail safely as unsupported.
- Operation authorization is capability-based (not `type`-based), using explicit capability flags.
- Internal implementation terminology is identity-first while MVP CLI remains agent-first.
- `manifest.json` reserves nullable forward-compat fields for standards-based identity and portability expansion and preserves unknown fields.
- `vault.enc` entries use immutable `entryId` in v1.
- Schema migrations follow explicit `from_version` -> `to_version` contracts with `ICE-202` on unsupported versions.
- Portable `.icebox-agent` format contract is defined now; import/export commands remain Phase 1.5.
- Compatibility/evolution guardrails (format markers, provenance metadata, canonical serialization, extension namespaces, support window, conformance fixtures) are defined now to reduce breaking changes later.

## Public Release Gate

- `v0.1.0` is an internal validation slice tag (not a public GA release).
- First public release starts at `v0.1.1` after minimum hardening gates pass:
  - Enclave-wrapped identity key path active for runtime operations.
  - Vault integrity checks (`ICE-201/202/203/204/205`) enforced on security-critical paths.
  - Sensitive runtime files/dirs enforce owner-only permissions (`0700` dirs, `0600` files).
  - `run` uses direct exec + allowlist-first env.
  - No outbound network and core dumps disabled.

### `v0.1.1` Security Gate (P0 Blockers)

- P0-1: No integrity downgrade path.
  - Missing/corrupt `hmac.enc` must fail closed for MVP-created agents (no warning-and-proceed path).
- P0-2: Rollback protection must be durable across process restarts.
  - Coordinated rollback scenarios (for example `vault.enc` + `hmac.enc`) must be detected by a persisted monotonic integrity anchor, not process-local cache only.
- P0-3: Local filesystem requirement is fail-closed by default on security-critical commands.
  - Non-local/synced filesystem detection cannot be warning-only in public release mode.
- P0-4: Runtime hardening baseline is enforced.
  - No outbound network from `icebox`, core dumps disabled, owner-only file modes, and allowlist-first env model.
- P0-5: Signed/notarized distribution and entitlement boundary are verified.
  - Public artifacts must pass notarization/Gatekeeper checks and runtime entitlement validation.

## Deferred To Post-MVP Hardening

- Selected hardening controls that are not required for MVP Core.
- These ship in the immediate post-MVP hardening release.
- Canonical sequencing details live in planning docs, not this lock file.

## Related Docs

- `README.md`
- `../plan/IMPLEMENTATION_BOOTSTRAP.md`
- `../plan/ROADMAP.md`


---

*Last updated: 2026-02-16*
