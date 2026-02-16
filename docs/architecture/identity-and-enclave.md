# Identity & Enclave

## Identity Model

- One agent identity record per `agentId` (UUID/ULID).
- `name` is a mutable user label; it is not the internal primary key.
- Ed25519 keypair per agent.
- Public key stored as multicodec-prefixed bytes in `identity.pub`.
- DID compatibility fields (`did`, `pubkeyFingerprint`) persisted for forward compatibility.
- CLI UX remains name-based (`use-agent <name>`), with internal resolution to `agentId`.

## Identity Type Contract

- `type` is an explicit identity-kind enum field in `manifest.json`.
- MVP supported value: `agent`.
- Reserved values for forward-compatibility: `human`, `robot`, `service`, `algorithm`.
- Unknown `type` values must fail safely with a clear unsupported-type error (no implicit fallback behavior).

## Capability-First Behavior Contract

- Runtime authorization/behavior checks must be capability-based, not hardcoded by `type`.
- Capabilities are explicit booleans in metadata and validated per operation:
  - `canHoldSecrets`
  - `canRunCommands`
  - `canSign`
  - `hasEnclaveBinding`
- MVP `agent` manifests set these capability flags explicitly.

## Internal Naming Contract

- CLI/user-facing commands keep `agent` terminology in MVP.
- Internal structs/modules/services should use neutral `identity` terminology to reduce future refactors.
- Resolution flow remains: CLI agent name -> internal `agentId` -> identity record/capabilities.

## Manifest Forward-Compatibility Contract

- `manifest.json` is versioned and must preserve unknown fields on read/write.
- MVP writes stable core fields plus nullable forward-compat fields.
- Forward-compat nullable fields reserved in MVP:
  - `keyAlgorithm`
  - `curve`
  - `didMethod`
  - `derivationScheme`
  - `coinType`
  - `network`
  - `keyPurposes`
- Future identity, portability, and interoperability features populate these fields without breaking old manifests.

## Enclave Wrapping Model

- Ed25519 private key is generated in software memory.
- Private key is wrapped by Secure Enclave P-256 key and stored as `key.enc`.
- Unwrap occurs only at operation time; buffers are wiped after use.

## Registration-Time Integrity Material

- During `register-agent`, Icebox also generates a random HMAC key for vault integrity checks.
- The HMAC key is enclave-wrapped and stored as `hmac.enc` in the agent directory.
- Plaintext HMAC key material is wiped from memory after wrapping.
- `hmac.enc` is required for integrity verification on security-critical vault operations in MVP.

## Multi-Agent Isolation

- Per-agent directories and vaults under `~/.icebox/identities/<name>/`.
- Config tracks `activeAgentId` and an `agents` registry keyed by `agentId`.
- Backward compatibility: legacy records missing `agentId` are auto-populated once and persisted.

## Related Docs

- `data-models-and-layout.md`
- `mvp-decision-lock.md`


---

*Last updated: 2026-02-16*
