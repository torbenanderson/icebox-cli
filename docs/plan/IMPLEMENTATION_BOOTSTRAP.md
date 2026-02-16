# Icebox Implementation Bootstrap

> Execution checklist for MVP implementation, aligned to architecture docs, `BACKLOG.md`, and `TESTING.md`.
>
> **MVP Decision Lock:** See [../architecture/mvp-decision-lock.md](../architecture/mvp-decision-lock.md) for locked decisions. **Vertical slice:** See [BACKLOG.md#phase-1-vertical-slice-milestone](BACKLOG.md#phase-1-vertical-slice-milestone-sequencing-only) for delivery order.

## Scope

- Phase 1 MVP only (split into MVP Core v0.1.0 + MVP Hardening Pack v0.1.1)
- Thin vertical slice first: `register-agent` -> `add` -> `run` -> `remove` (see BACKLOG Phase 1 Vertical Slice)
- `v0.1.0` is internal validation only; first public GA starts at `v0.1.1` after hardening minimums
- No seed/recovery/export/import commands in MVP
- No DID commands in MVP (compatibility fields still persisted)

## Execution Guardrails (Complexity Control)

To avoid early over-engineering, keep one command path runnable at all times.

- Maintain an **always-green runnable slice**: `register-agent` -> `add` -> `run` -> `remove`
- Do not block this slice on enclave, HMAC, or advanced hardening features
- Add hardening in **incremental layers** after the slice is stable

### Runnable Slice Gate (Must Pass Before Hardening Layers)

- [ ] `register-agent <name>` creates minimum usable local identity/config artifacts
- [ ] `add <service> <secret>` persists a secret in vault storage
- [ ] `run <service> <command>` resolves secret and executes with direct exec (no shell)
- [ ] `remove <service>` deletes persisted secret cleanly
- [ ] One integration test covers the full flow end-to-end

## Milestone 0: Repository Scaffold

- [ ] Create `Cargo.toml` with crate metadata and pinned security dependencies
- [ ] Create `src/` layout:
  - [ ] `src/main.rs`
  - [ ] `src/lib.rs`
  - [ ] `src/error.rs`
  - [ ] `src/cli/mod.rs`
  - [ ] `src/config/mod.rs` (or `src/config.rs` if single-file; DESIGN uses `config.rs`, either is fine)
  - [ ] `src/agent/mod.rs`
  - [ ] `src/agent/enclave.rs`
  - [ ] `src/agent/enclave_darwin.rs`
  - [ ] `src/agent/enclave_stub.rs`
  - [ ] `src/agent/did.rs` (did:key derivation for manifest compatibility fields; full DID commands are Phase 1.5)
  - [ ] `src/vault/mod.rs`
  - [ ] `src/vault/store.rs`
  - [ ] `src/vault/crypto.rs`
  - [ ] `src/vault/validation.rs`
  - [ ] `src/runner/mod.rs`
- [ ] Add `tests/` directory with `integration.rs` and `security.rs`

> **Milestone ordering:** Milestones 0–5 complete the vertical slice. Milestones 6–7 add reconcile and hardening.

## Milestone 1: CLI + Error Surface

- [ ] Implement global flags: `--debug`, `--quiet`, `--agent`
- [ ] Implement commands:
  - [ ] `register-agent`
  - [ ] `use-agent`
  - [ ] `list-agents`
  - [ ] `add`
  - [ ] `list`
  - [ ] `remove`
  - [ ] `remove-agent`
  - [ ] `run`
  - [ ] `reconcile`
- [ ] Add CLI stubs for MVP helpers (full impl in later milestones):
  - [ ] `copy-services --from --to` (stub in M1; full decrypt→re-seal impl after M4 crypto)
  - [ ] `migrate-enclave <agent>` (stub in M1; full impl after enclave handling)
- [ ] Implement structured ICE error mapping at CLI boundary
- [ ] Wire `docs/reference/error-codes.json` validation test

## Compatibility Guardrails (Cross-Cutting)

- [ ] Add `format` + `schemaVersion` markers to all persisted artifacts
- [ ] Add `createdByVersion` + `lastMigratedByVersion` metadata and update rules
- [ ] Implement canonical serialization helpers used by all writers/exporters
- [ ] Preserve unknown fields and extension namespaces (`x_icebox_*`, `x_vendor_*`)
- [ ] Implement compatibility window enforcement (`N`, `N-1`, `N-2`, older fail `ICE-202`)
- [ ] Add conformance fixtures for manifest/vault/bundle as merge gates
- [ ] Add deterministic import validation checks (checksum, duplicates, unsupported required fields)
- [ ] Add JSON Schema files + fixtures for persisted artifacts and wire CI validation workflow
- [ ] Add forward-only ADR template/log usage for decision-impacting changes

## Milestone 2: Config + Agent Identity

- [ ] Implement `~/.icebox/config.json` read/write with atomic update
- [ ] Implement non-interactive first-run fail behavior
- [ ] Implement interactive first-run prompt behavior
- [ ] Ensure `--agent` remains one-shot targeting only (does not mutate `activeAgentId`)
- [ ] Implement agent name validation
- [ ] Implement `register-agent`:
  - [ ] Generate immutable `agentId` (UUID/ULID)
  - [ ] Generate Ed25519 keypair
  - [ ] Store `identity.pub` (`0xed01 || pubkey`)
  - [ ] Compute/store `did` and `pubkeyFingerprint` compatibility fields
  - [ ] Store `derivationVersion: null`
  - [ ] Store reserved nullable forward-compat fields (`keyAlgorithm`, `curve`, `didMethod`, `derivationScheme`, `coinType`, `network`, `keyPurposes`)
  - [ ] Preserve unknown `manifest.json` fields on read/write
  - [ ] Create manifest + agent directory + empty vault
- [ ] Store `agentId` in `manifest.json` and `config.json` `agents` registry entries
- [ ] Implement name -> `agentId` resolution for all name-based CLI commands
- [ ] Implement one-time legacy migration (`activeAgent` -> `activeAgentId`, missing `agentId` backfill) with atomic persistence
- [ ] Enforce `type` enum contract (`agent` supported in MVP; reserved values parsed but unsupported)
- [ ] Implement capability flags (`canHoldSecrets`, `canRunCommands`, `canSign`, `hasEnclaveBinding`) and gate operations by capabilities (not by `type`)
- [ ] Keep internal domain/service naming neutral (`identity`), while preserving agent-based CLI UX
- [ ] Implement per-agent Secure Enclave wrapping key and `key.enc`
- [ ] Implement hard fail on `enclaveAlgorithm` mismatch

## Milestone 3: Vault Core

- [ ] Implement `VaultStore` synchronous trait
- [ ] Implement file backend with:
  - [ ] `flock` lock file
  - [ ] atomic write via temp + rename
- [ ] Implement vault schema v1 with reserved `null` fields preserved
- [ ] Add immutable `entryId` to each vault entry; preserve across updates
- [ ] Implement service uniqueness enforcement at write path
- [ ] Implement `validate_vault_load()` single canonical pipeline
- [ ] Enforce service uniqueness in load validation
- [ ] Implement strict local filesystem mode (`--require-local-fs`)
- [ ] Implement migration registry (`from_version` -> `to_version`) with `ICE-202` on unsupported versions

## Milestone 4: Crypto + Integrity

- [ ] Implement sealed-box compatible encryption (`crypto_box_seal` wire format)
- [ ] Implement Ed25519 -> X25519 conversions using pinned crates/APIs
- [ ] Implement service-name binding in payload
- [ ] Implement `hmac.enc` generation at registration
- [ ] Implement HMAC on vault writes (`add`/`remove`)
- [ ] Implement HMAC verification on security-critical loads
- [ ] Implement `list --strict` integrity verification

## Milestone 5: Secure Run

- [ ] Implement no-shell command execution via `std::process::Command`
- [ ] Implement allowlist-first subprocess env
- [ ] Implement env injection default path
- [ ] Implement command provenance warning for untrusted/unknown source
- [ ] Implement per-run temp dir (`0700`)
- [ ] Implement stale temp sweep on every CLI invocation

## Milestone 6: Reconcile + Fail-Closed

- [ ] Implement drift detection:
  - [ ] orphaned directory
  - [ ] missing directory
  - [ ] DID mismatch
- [ ] Implement `icebox reconcile` interactive and `--yes`
- [ ] Enforce fail-closed for security-critical commands (`add`/`remove`/`run`) until reconciled

## Milestone 7: Security Hardening

- [ ] Enforce no outbound network from `icebox` process
- [ ] Disable core dumps (`RLIMIT_CORE=0`)
- [ ] Ensure no clipboard interactions
- [ ] Enforce owner-only file permissions (`0700` dirs, `0600` sensitive files) with fail-closed checks
- [ ] Add memory hygiene wrappers (`secrecy`, `zeroize`, `mlock`)
- [ ] Enclave throttling:
  - [ ] fixed cooldown
  - [ ] failure backoff
  - [ ] single-flight unwrap

## Hardening Layer Order (Post-Slice)

After the runnable slice gate is passing, apply hardening in this order:

1. Structured error-code coverage at CLI boundary
2. Atomic writes + file locking for config/vault
3. Owner-only filesystem permission enforcement (`0700` dirs, `0600` sensitive files)
4. Secure Enclave wrap/unwrap path
5. Vault integrity verification (`hmac.enc`, HMAC checks, rollback detection)
6. Runtime execution hardening (allowlist env, tempdir controls, provenance warnings)

## CI Gates (Must Pass)

- [ ] Unit + integration tests on macOS
- [ ] Non-enclave tests on Linux
- [ ] Crypto interop merge gates:
  - [ ] did:key vector tests
  - [ ] sealed-box libsodium round-trip tests
- [ ] Security baseline tests from `TESTING.md` vertical-slice + MVP rows

## Public `v0.1.1` Release Blockers

### P0 (Must Pass Before Public Release)

- [ ] P0-1 No integrity downgrade path (`E3-17`)
  - Missing/corrupt `hmac.enc` fails closed for MVP-created agents.
  - Required tests: `T-E3-17c` (updated fail-closed expectation), `T-SEC-17`, `T-SEC-18`
- [ ] P0-2 Durable rollback protection across restart (`E3-13`, `E3-16`, `E3-17`)
  - Coordinated rollback is detected after restart by persisted monotonic integrity state.
  - Required tests: `T-E3-13e`, `T-SEC-17`, `T-SEC-19` (updated durable expectation)
- [ ] P0-3 Local filesystem default fail-closed (`E3-14`)
  - Security-critical commands fail on non-local/synced filesystems by default.
  - Required tests: `T-E3-14`, `T-E3-14b`
- [ ] P0-4 Runtime hardening baseline (`E1-07`, `E1-08`, `E1-20`, `E5-04`)
  - Required tests: `T-E1-07`, `T-E1-08`, `T-E1-20`, `T-E5-04`, `T-E5-04b`, `T-SEC-08`
- [ ] P0-5 Signed/notarized and entitlement boundary (`E1-15`, `E1-17`, `E1-18`)
  - Required tests: `T-E1-17`, `T-E1-18`, `T-SEC-15`, `T-SEC-16`

### P1 (Immediate Follow-Ups, Not Public-Release Blockers)

- [ ] P1-1 Path model alignment (`agentId`-first storage) (`E2-11`, `E2-28`, `E2-32`)
- [ ] P1-2 `run` argv-first invocation model over shlex string parsing (`E5-02`)
- [ ] P1-3 Debug channel hardening (`stderr` + explicit redaction boundary) (`E1-10`, `E1-11`)
- [ ] P1-4 Schema strictness upgrade for critical identifiers/namespaces (`E1-21`, `E1-23`, `E1-28`)

## Explicitly Deferred (Do Not Implement in MVP)

- [ ] DID CLI commands (`icebox did`, DID doc generation, verification)
- [ ] Seed backup/recovery/export/import
- [ ] Portable `.icebox-agent` import/export commands (format contract is defined now; command implementation is Phase 1.5)
- [ ] Biometric toggle/config
- [ ] `--unsafe-substitute` placeholder arg mode
- [ ] Phase 2 policy enforcement templates/allowlists

## Post-MVP Hardening Notes

- Certain hardening controls are intentionally sequenced after MVP Core.
- Track these as follow-up implementation issues tied to the first post-MVP hardening release.


---

*Last updated: 2026-02-16*
