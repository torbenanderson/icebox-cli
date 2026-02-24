# Data Models & Layout

## Runtime Paths

- `~/.icebox/config.json`
- `~/.icebox/identities/<agent>/manifest.json`
- `~/.icebox/identities/<agent>/identity.pub`
- `~/.icebox/identities/<agent>/key.enc`
- `~/.icebox/identities/<agent>/hmac.enc`
- `~/.icebox/identities/<agent>/vault.enc`

## Required File Modes

- `~/.icebox/` and `~/.icebox/identities/<agent>/` must be `0700`.
- Sensitive files (`config.json`, `manifest.json`, `identity.pub`, `key.enc`, `vault.enc`, `hmac.enc`) must be `0600`.
- CLI must validate these modes at startup for security-critical operations and fail closed when unsafe.

## Schemas

- `manifest.json` stores identity metadata and compatibility anchors.
- `config.json` stores `activeAgentId` and agent registry entries keyed by `agentId`.
- `vault.enc` stores versioned sealed entries and integrity fields.

### `manifest.json` (v1 contract)

- Core required fields stay stable in v1 (`version`, `agentId`, `type`, `name`, `did`, `pubkeyFingerprint`, `enclaveKeyRef`, timestamps).
- `type` enum contract:
  - MVP supported: `agent`
  - Reserved: `human`, `robot`, `service`, `algorithm`
- Reserved nullable forward-compat fields must exist in v1:
  - `keyAlgorithm`, `curve`, `didMethod`, `derivationScheme`, `coinType`, `network`, `keyPurposes`
- Reserved device/lane compatibility fields for expansion:
  - `wrappingScheme`
  - `backendClass`
  - `identityLane`
- Unknown fields must be preserved across read/write cycles.
- Unknown `type` must fail safely as unsupported until explicitly implemented.

### Generic Identity Schema Contract

- Keep common identity fields stable across all future identity kinds:
  - `agentId`, `type`, `name`, `did`, `pubkeyFingerprint`, timestamps, format/version/provenance fields
- Type-specific data lives in extension fields/capability flags, not by changing common field semantics.
- Capability flags are explicit booleans and checked per operation:
  - `canHoldSecrets`, `canRunCommands`, `canSign`, `hasEnclaveBinding`

### `config.json` (v1 contract)

- Primary selector is `activeAgentId` (not agent name).
- `agents` entries include at least: `agentId`, `name`, `did`.
- Device enrollment metadata may expand `agents` entries (or adjacent arrays) with backend/class references while preserving existing keys.
- Name-based CLI commands resolve `name` -> `agentId` before vault/identity operations.
- Backward compatibility:
  - If `agentId` is missing in legacy config or manifest records, generate one UUID/ULID once.
  - Persist the generated `agentId` to both `config.json` and `manifest.json` atomically.
  - If legacy `activeAgent` exists, map it to `activeAgentId` during upgrade.

### `vault.enc` entry contract (v1)

- Each entry includes immutable `entryId` (UUID/ULID string) and mutable `service` label.
- `entryId` is unique per vault and never changes across service renames or value updates.

### Portable Agent Bundle Contract (`.icebox-agent`, Phase 1.5+)

- Archive format is versioned and deterministic (stable file ordering).
- Includes: `manifest.json`, `identity.pub`, `vault.enc`, `bundle.manifest.json`.
- Excludes device-bound material: `key.enc`, `hmac.enc`.
- `bundle.manifest.json` records per-file SHA-256 checksums and bundle format version.

## Project Structure

- Thin binary in `main.rs`.
- Library-first modules under `src/`.
- Platform-specific enclave implementation separated by `#[cfg]`.

## Related Docs

- `identity-and-enclave.md`
- `vault-and-integrity.md`


---

*Last updated: 2026-02-24*
