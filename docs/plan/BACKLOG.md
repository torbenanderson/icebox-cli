# Icebox Backlog

> Use cases and tasks mapped to [ROADMAP.md](ROADMAP.md) epics.
>
> All items are pending unless marked otherwise. Track progress in issues/PRs.

## Phase 1 Vertical Slice Milestone (Sequencing Only)

> Early milestone for implementation velocity. This is a delivery order marker only; it does not add/remove scope or change phase ownership.
> Backlog IDs are stable references and may be non-contiguous; gaps do not imply missing scope in this sequence.

- E1-01 Cargo init
- E1-02 CLI scaffolding
- E1-03 Project structure
- E1-04 CI pipeline (initial pass)
- E1-07 Disable core dumps
- E1-13 Structured error codes (initial mapping)
- E2-01 Generate keypair
- E2-02 Enclave wrapping key
- E2-03 Wrap Ed25519 key
- E2-04 No plaintext key on disk
- E2-11 Active agent tracking (minimum usable config)
- E2-09 Duplicate guard
- E2-18 Agent name validation
- E3-21 Identity/config refactor baseline
- E3-01 + E3-02 Vault creation + sealed-box encryption (combined execution unit)
- E3-05 Empty vault
- E3-10 Vault version field
- E3-11 Atomic vault writes
- E3-12 File locking
- E4-01 Add secret
- E4-02 List secrets
- E4-04 Remove secret
- E4-06 Service name validation
- E5-01 Basic run
- E5-02 Direct exec (no shell)
- E5-03 Env var injection (default)
- E5-05 Output passthrough
- E5-06 Exit code forwarding
- E5-07 Missing secret error
- E5-10 `--agent` flag

### MVP Core vs Post-MVP Hardening (Sequencing Only)

> Scope is unchanged; this is a release slicing decision to reduce delivery risk.

- **MVP Core:** prioritize the always-runnable `register-agent` → `add` → `run` → `remove` path.
- **Release boundary:** `v0.1.0` is internal validation only; first public GA starts at `v0.1.1` after minimum hardening gates.
- **Post-MVP Hardening:** detailed deferred controls are tracked in implementation planning docs.

### Post-Slice Hardening Layering (Sequencing Only)

> Canonical order is maintained in [Implementation Bootstrap](IMPLEMENTATION_BOOTSTRAP.md#hardening-layer-order-post-slice).

---

## E1 -- Project Bootstrap

| ID | Use Case | Description |
|---|---|---|
| E1-01 | Cargo init | `cargo init` (icebox-cli crate) |
| E1-02 | CLI scaffolding | Set up `src/main.rs` with `clap` |
| E1-03 | Project structure | Create `src/` modules: `agent`, `config`, `vault`, `runner`, `did`. Prefer neutral `identity` naming in internal domain services/types even if MVP CLI remains `agent`-named. Enclave code isolated to `enclave_darwin.rs` (`#[cfg(target_os = "macos")]`, raw FFI) with `enclave_stub.rs` (`#[cfg(not(target_os = "macos"))]`) for non-macOS builds. |
| E1-04 | CI pipeline | GitHub Actions: macOS runner (full tests incl. enclave) + Linux runner (vault/crypto/DID/CLI tests only; enclave code excluded via `#[cfg]`). Build, test, lint on push. |
| E1-05 | Makefile | Build, test, install targets |
| E1-06 | `--version` flag | Print version, commit hash, build date |
| E1-07 | Disable core dumps | Set `RLIMIT_CORE = 0` at process start to prevent secret leakage via crash dumps |
| E1-08 | No network | Enforce zero outbound network calls from the `icebox` process in v1; verify in tests |
| E1-09 | No clipboard | Icebox never copies secrets to the system clipboard |
| E1-10 | Error messaging (default) | Minimal, non-technical error messages by default; no internal paths, keys, or crypto details |
| E1-11 | `--debug` flag | `--debug` on any command outputs detailed internals (paths, crypto codes, traces) to stdout only |
| E1-12 | `--quiet` flag | `--quiet` suppresses all non-essential output (for scripting / automation) |
| E1-13 | Structured error codes | Error codes (`ICE-1xx`, `ICE-2xx`, etc.) categorized by root cause (auth, vault, agent, secret, exec, enclave, input). Safe for support tickets without exposing internals. Codes never reused. MVP implementation source of truth is runtime code mapping; keep JSON/codegen out of MVP scope. |
| E1-14 | Test scaffolding | Set up `tests/` directory, test helpers (temp enclave keys, temp `~/.icebox/`), CI test runner. See [TESTING.md](TESTING.md) |
| E1-15 | Release pipeline | GitHub Actions: `cargo build --release` / `cargo-dist`, **Developer ID signed + notarized** macOS binary, `cargo install` compatible (unsigned, dev only), GitHub Releases page. Homebrew pre-built bottles follow after first stable release. Include tag-triggered release automation (`v*`) that runs validation, builds release artifacts, publishes checksums, and uploads assets to GitHub Releases. |
| E1-16 | Install docs | Finalize install instructions in README (`cargo install` + binary download). Document that `cargo install` produces an unsigned binary with limited enclave access (dev only). |
| E1-17 | Entitlements file | Create project entitlements file (for example `icebox-cli.entitlements`) with `com.apple.security.smartcard`, `com.apple.keychain-access-groups`, hardened runtime. Embedded during `codesign` step in release pipeline. |
| E1-18 | Notarization | Integrate `xcrun notarytool` into release pipeline. Binary submitted to Apple for notarization after signing; stapled ticket attached to the distributed binary. |
| E1-19 | Error code artifact | Publish machine-readable error-code registry at `docs/reference/error-codes.json` when external consumers need it; add a sync test with runtime code mapping before introducing code generation |
| E1-20 | File permission baseline | Enforce owner-only filesystem modes for runtime artifacts: `~/.icebox/` + agent dirs `0700`; sensitive files (`config.json`, `manifest.json`, `identity.pub`, `key.enc`, `vault.enc`, `hmac.enc`) `0600`. Validate and fail closed on mismatch for security-critical operations. |
| E1-21 | Artifact type/version markers | Add `format` + `schemaVersion` to `manifest.json`, `vault.enc`, and `.icebox-agent` bundle metadata to prevent ambiguous migrations |
| E1-22 | Canonical serialization contract | Enforce canonical JSON serialization rules (UTC RFC3339 timestamps, lowercase hex, fixed base64 variant, deterministic export ordering) |
| E1-23 | Extension namespace policy | Reserve `x_icebox_*` and `x_vendor_*` namespaces; preserve unknown extension fields across read/write |
| E1-24 | Compatibility support window | Document and enforce support policy (`N`, `N-1` read/write; `N-2` read-only; older unsupported with `ICE-202`) |
| E1-25 | Artifact provenance metadata | Persist `createdByVersion` and `lastMigratedByVersion` on persisted artifacts |
| E1-26 | Deprecation lifecycle policy | Implement and document introduced -> soft-deprecated -> hard-deprecated -> removed lifecycle with minimum release counts |
| E1-27 | Conformance fixtures | Add golden fixtures for manifest/vault/bundle round-trips as merge gates across implementations |
| E1-28 | JSON Schema CI gate | Add machine-validated JSON Schemas for manifest/config/vault/bundle-manifest and enforce schema + fixture validation in CI |
| E1-29 | ADR process (forward-only) | Maintain lightweight architecture decision records under `docs/architecture/decisions/` for new decision-impacting changes (no historical backfill) |
| E1-30 | Release enclave-signing verification gate | During release candidate validation, verify the distributed binary is signed with hardened runtime + required entitlements and passes real `register-agent` Secure Enclave key-creation flow on supported macOS hardware. Block public release on failure. |
| E1-31 | Explicit developer software-backend mode (deferred) | Post-`v0.1.1`, add an explicit opt-in insecure/developer mode for unsupported hardware (for example `--developer-mode` / `--insecure-software-backend`) with fail-safe default disabled, prominent warnings, distinct capability/security labeling, and no enclave-grade security claims. |
| E1-32 | Docs site publishing | Two-step approach: (1) **Before MVP go-live**: Deploy mdBook + rustdoc to GitHub Pages via GitHub Actions (`peaceiris/actions-gh-pages`); configure repo Settings → Pages → GitHub Actions. (2) **Post-MVP** (optional): Add custom domain via CNAME if desired. |

## E2 -- Agent Identity

> E2 supports two execution lanes: `local-enclave` (MVP-first) and `paired-remote-signer` (post-MVP). The logical identity contract stays the same across lanes; backend/device scheme differs.

| ID | Use Case | Description |
|---|---|---|
| E2-01 | Generate keypair | As a user, I can run `icebox register-agent claw` to create an Ed25519 keypair and `~/.icebox/identities/claw/` directory (local lane baseline) |
| E2-02 | Enclave wrapping key | Create a P-256 key inside the Secure Enclave (non-exportable, per-agent); used to encrypt the Ed25519 private key in `local-enclave` lane |
| E2-03 | Wrap Ed25519 key | Encrypt the Ed25519 private key with the enclave P-256 key (`SecKeyCreateEncryptedData`); store as `key.enc` in `local-enclave` lane. Partial-artifact risk (for example `key.enc` written before all identity artifacts) is accepted in this step and tightened in E2-04 hardening. |
| E2-04 | No plaintext key on disk | Ed25519 private key never written to disk in plaintext; only the enclave-wrapped `key.enc` blob exists in `local-enclave` lane. Includes hardening/cleanup expectations for unsafe or partial persistence paths from earlier steps. |
| E2-05 | Multicodec + DID compatibility | Public key stored as `identity.pub` in multicodec-prefixed binary format (`[0xed, 0x01] \|\| 32-byte Ed25519 pubkey`, 34 bytes total). `did:key` value and `pubkeyFingerprint` are stored in `manifest.json` as compatibility anchors; DID-facing commands remain Phase 1.5. Migration contract: accept legacy 32-byte raw `identity.pub` during transition, write 34-byte multicodec format going forward, and keep migration idempotent/deterministic. |
| E2-06 | Agent manifest | `manifest.json` stores versioned identity metadata: `version`, immutable `agentId` (UUID/ULID), `type` ("agent"), `name`, `did` (compatibility anchor), `parent` (null in v1), `created`, `pubkeyFingerprint`, `enclaveKeyRef`, `derivationVersion` (`null` in MVP), plus reserved nullable forward-compat fields (`keyAlgorithm`, `curve`, `didMethod`, `derivationScheme`, `coinType`, `network`, `keyPurposes`). Unknown fields are preserved across read/write. |
| E2-07 | Agent listing | `icebox list-agents` reads from `config.json` `agents` registry (`agentId`, `name`, `did`). No filesystem scan. Shows name, DID, and active status. Orphaned directories (not in registry) listed separately with a warning. |
| E2-08 | Agent removal | `icebox remove-agent <name>` removes agent directory + enclave wrapping key + removes entry from `config.json` `agents` array. If the removed agent was active, clears `activeAgentId`. |
| E2-09 | Duplicate guard | Registering an agent name that already exists in the `agents` registry returns a clear error |
| E2-10 | First-run prompt | If `agents` array is empty (or `config.json` doesn't exist), Icebox prompts interactively for an agent name (validates: lowercase, 3-32 chars, letters/numbers/hyphens). In non-interactive mode, fail with guidance to run `register-agent` |
| E2-11 | Active agent tracking | `~/.icebox/config.json` `activeAgentId` tracks the active agent. `agents` array is the registry of all known agents (`agentId` + `name` + DID). Registration appends, removal deletes, rotation updates DID. `--agent` is one-shot targeting only and does not mutate `activeAgentId`. |
| E2-13 | Use active agent | `icebox use-agent <name>` resolves name -> `agentId`, then sets `activeAgentId` explicitly without executing secret operations (persistent default selector for future commands). |
| E2-26 | Agent registry reconcile | `icebox reconcile` scans `identities/` vs `config.json` `agents`; reports orphaned directories, missing directories, DID mismatches. Interactive prompts to fix (or `--yes` to auto-confirm). Security-critical commands fail closed until reconciled |
| E2-12 | Enclave ACLs | Enclave wrapping key created with `SecAccessControl`: `kSecAttrAccessibleWhenUnlockedThisDeviceOnly` + `.privateKeyUsage`. No biometric gate in v1 and no biometric toggle in MVP. `.userPresence` deferred post-MVP. ACL scoped to Icebox's code-signing identity via `com.apple.keychain-access-groups`. |
| E2-18 | Agent name validation | Agent names validated: lowercase letters, numbers, hyphens, 3-32 characters |
| E2-19 | Runtime unwrap | At runtime, load `key.enc`, ask Secure Enclave to decrypt, receive Ed25519 key in `secrecy::Secret` buffer, wipe after use |
| E2-22 | Key rotation (post-MVP) | `icebox rotate-key claw` -- generates new Ed25519 keypair, new enclave wrapping key, re-seals all vault entries atomically, updates manifest/DID. Warns if seed-derived (seed linkage lost). Not in MVP. |
| E2-23 | Enclave unwrap cooldown | After a successful `SecKeyCreateDecryptedData` call, enforce a 200ms minimum interval before the next unwrap for the same agent. Failed attempts double the cooldown (capped at 30s). |
| E2-25 | Single-flight unwrap | Coalesce concurrent unwrap requests for the same agent within a single process via `std::sync::OnceLock` / single-flight pattern. Only one thread touches the enclave; others wait. |
| E2-27 | Enclave algorithm migration | If `manifest.json` `enclaveAlgorithm` mismatches runtime constant, fail hard and require explicit `icebox migrate-enclave <agent>` flow |
| E2-28 | Legacy `agentId` backfill | On startup/load, if legacy config/manifest records are missing `agentId`, auto-generate UUID/ULID once, persist to both files atomically, and migrate `activeAgent` -> `activeAgentId` when present |
| E2-29 | Algorithm negotiation states | Identity/crypto algorithm fields support explicit states: `preferred`, `allowed`, `deprecated`; deprecated remains readable through the compatibility window |
| E2-30 | Identity type enum contract | `manifest.json` `type` is enum-based: MVP supports `agent`; reserves `human`, `robot`, `service`, `algorithm`. Unknown type fails safely with unsupported-type error until implemented |
| E2-31 | Capability-first authorization | Operation eligibility is checked via explicit capability flags (`canHoldSecrets`, `canRunCommands`, `canSign`, `hasEnclaveBinding`) instead of branching on `type` |
| E2-32 | Internal identity naming | Keep CLI/user UX `agent` terms in MVP, but use neutral `identity` terminology in internal domain/service layers |
| E2-33 | Identity lane metadata | Persist explicit identity operation lane metadata (`local-enclave`, `paired-remote-signer`) with fail-safe behavior for unknown lanes |
| E2-34 | Device enrollment bindings | Track per-device backend bindings for an identity without changing identity primary keys (`agentId`) |
| E2-35 | Approval/session states | Broker/CLI contract returns deterministic approval states (`ok`, `pending_approval`, `denied`, `expired`) for protected operations |
| E2-36 | Passkey/WebAuthn approval (post-MVP) | Identity-level second factor: user authenticates with passkey before enclave unwrap; passkey signs assertion, Icebox verifies. Sealing key remains Ed25519. Reserved `approvalBackend` in manifest. |

## E3 -- Encrypted Vault

| ID | Use Case | Description |
|---|---|---|
| E3-21 | Identity/config refactor baseline | Refactor identity/config foundations before broader E3 delivery: clarify DID backend naming (non-enclave behavior labels), tighten config-to-runtime error mapping (invalid config gets dedicated code path), split `register-agent` into smaller units, and reduce duplicate canonical-name scans with reusable/cached canonical sets. |
| E3-01 | Vault creation | `~/.icebox/identities/<name>/vault.enc` is created on first use as a JSON file of sealed blobs |
| E3-02 | Sealed-box encryption | Each secret is individually sealed via `crypto_box_seal` (Ed25519→X25519 + XSalsa20-Poly1305); wire-format compatible with libsodium |
| E3-03 | Per-secret isolation | Each secret is an independent sealed blob with immutable `entryId`; no shared master key or KDF |
| E3-04 | Vault integrity | Tampered sealed blobs are detected and rejected (AEAD authentication) |
| E3-05 | Empty vault | New vault with no secrets returns a clean state, not an error |
| E3-06 | Unseal via enclave | Decryption requires the agent's Ed25519 private key, unwrapped from the Secure Enclave at moment of use |
| E3-07 | `secrecy` + `Zeroize` | All secret buffers wrapped in `secrecy::Secret` with `Zeroize` on drop; `libc::mlock` pins key buffers. No GC in Rust -- all memory deterministically freed. This is the planned hardening point for residual plaintext-memory windows left by earlier bootstrap/wrap steps. |
| E3-08 | `mlock` pinning | Secret buffers are `mlock`'d to prevent paging to swap/disk |
| E3-09 | No secure temp | No temp files are created during vault decryption |
| E3-10 | Vault version field | Every `vault.enc` includes `"version": 1` at top level for forward-compatible format upgrades |
| E3-11 | Atomic vault writes | Vault updates written to `vault.enc.tmp` then atomically renamed via `std::fs::rename`; prevents corruption on crash |
| E3-12 | File locking | Advisory `flock` on `vault.enc.lock` during read-modify-write cycles; prevents concurrent process corruption |
| E3-29 | Vault locking/error-path refactor cleanup | Refactor vault lock/error-path internals after E3 core execution: centralize lock lifecycle helper boundaries and normalize lock/open/unlock error mapping while preserving E3-10/11/12 behavior and test outcomes |
| E3-13 | Vault load validation | Implement the [Vault Load Validation Pipeline](../architecture/vault-and-integrity.md) — single canonical function, steps 1-5 at load time, 6-7 at unseal. Reject with `ICE-201` / `ICE-202` / `ICE-203` / `ICE-204` / `ICE-205` per spec (`ICE-203` reserved for rollback via `seq`; `ICE-205` for entry-structure/uniqueness validation failures). Enforce unique service names on load and write path |
| E3-14 | Filesystem check | On startup, `statfs` the `~/.icebox/` directory. If the filesystem type matches a known network/synced FS (NFS, SMB, CIFS, or known cloud-sync FUSE types), print a stderr warning. `--require-local-fs` hard-fails |
| E3-15 | HMAC key generation | At `register-agent` time, generate a random 256-bit HMAC key in `secrecy::Secret`, encrypt via Secure Enclave P-256 key (`SecKeyCreateEncryptedData`), store as `hmac.enc` in agent directory, wipe plaintext from memory. |
| E3-16 | Vault HMAC on write | On every vault write (`add`, `remove`): unwrap `hmac.enc` via enclave, compute `HMAC-SHA256(hmac_key, vault_body)` (body excludes the `hmac` field), store tag as hex string in `vault.enc` `hmac` field, wipe HMAC key. |
| E3-17 | Vault HMAC verification on load | On vault load (for security-critical operations -- `run`, `add`, `remove`): unwrap `hmac.enc`, recompute HMAC, constant-time compare against stored tag. Mismatch → `ICE-204`. Missing `hmac.enc` (pre-HMAC agent) → warn once, skip verification. |
| E3-18 | HMAC key recovery | Phase 1.5. `recover-agent` generates a new `hmac.enc` (new random HMAC key, encrypted by the new enclave key). First vault write after recovery establishes HMAC baseline. |
| E3-19 | Strict list integrity mode | `icebox list --strict` performs HMAC verification before output; default `list` remains non-enclave/non-strict |
| E3-20 | Schema migration contract | Implement explicit vault schema migrators (`from_version` -> `to_version`) with atomic writeback, unsupported-version fail (`ICE-202`), idempotency guarantees, and fixture-based migration tests |
| E3-22 | Vault envelope identity metadata (post-MVP) | Add optional self-description metadata fields in vault envelope (for example identity public-key reference) with explicit anti-drift rules and migration guards. Defer from MVP to keep envelope minimal. |
| E3-23 | Vault write provenance (post-MVP) | Add optional authenticated provenance layer for vault writes (for example signature/attestation metadata) when auditability is required. Keep `crypto_box_seal` anonymous-sender semantics in MVP. |
| E3-24 | Sealed-entry associated-data envelope (post-MVP) | Evaluate and add explicit associated-data contract for sealed entries when structured metadata binding is required; include migration and compatibility rules. |
| E3-25 | Vault backup/recovery UX guardrails (post-MVP) | Add explicit UX/docs/runtime warnings and guidance for backup and key-rotation consequences (vault unreadable without matching identity private key). |
| E3-26 | Cipher-suite re-evaluation (post-MVP) | Re-evaluate vault sealing cipher choices (including XChaCha20-Poly1305 paths) after MVP stability; keep current libsodium sealed-box compatibility until an explicit migration plan exists. |
| E3-27 | Vault error-code split + typed validation mapping (post-MVP) | Replace MVP broad `ICE-201` vault mapping with granular typed-path mapping at runtime boundary: `ICE-201` parse/corruption, `ICE-202` schema/version, `ICE-205` entry structure/uniqueness, planned `ICE-206` precondition/missing-dependency failures; include fixture-based tests for each code path. |
| E3-28 | Ed25519->X25519 interop contract verification (post-MVP) | Add explicit fixture/vector tests comparing runtime Ed25519->X25519 conversion assumptions (`to_scalar_bytes` + `to_montgomery`) against libsodium-compatible behavior to lock portability expectations across implementations. |

## E4 -- Secret Management

| ID | Use Case | Description |
|---|---|---|
| E4-01 | Add secret | As a user, I can run `icebox add openai sk-...` to store an encrypted secret |
| E4-02 | List secrets | As a user, I can run `icebox list` to see service names without values |
| E4-02a | Service inventory output | As a user, I can run `icebox list --services` to output only service names for credential-rotation checklists after device loss (no secret values) |
| E4-03 | Update secret | Adding a secret for an existing service overwrites the previous value. `created` timestamp is preserved from the original entry; `updated` is set to the current timestamp. Previous sealed blob is discarded (no history in v1). |
| E4-04 | Remove secret | As a user, I can run `icebox remove openai` to delete a stored secret |
| E4-05 | Secret from stdin | As a user, I can pipe a secret via stdin to avoid shell history exposure |
| E4-06 | Service name validation | Service names are validated (alphanumeric, hyphens, no whitespace) |
| E4-07 | `--agent` flag | All secret management commands accept `--agent <name>` to target a specific agent for that invocation (defaults to active, does not mutate `activeAgentId`) |
| E4-08 | Shell history warning | When a secret is passed as a CLI argument, warn: "Tip: pipe secrets via stdin to avoid shell history exposure" |
| E4-09 | Service copy helper | `icebox copy-services --from <agent-a> --to <agent-b>` copies services via decrypt→re-seal to support safe manual rotation/migration workflows |

## E5 -- Secure Run

| ID | Use Case | Description |
|---|---|---|
| E5-01 | Basic run | As a user, I can run `icebox run openai "curl ..."` and get the command output |
| E5-02 | Direct exec (no shell) | Command string parsed via shlex splitter, executed via `std::process::Command`; no shell invocation |
| E5-03 | Env var injection (default) | Secret injected as env var in subprocess (default); avoids `ps` visibility of secrets in args |
| E5-04 | Subprocess env sanitization | Subprocess gets allowlist-first env (`PATH`, `LANG`, `LC_*`, `TZ`, injected secret); strips `HOME`, `USER`, `LOGNAME`, `PWD`; sets `TMPDIR` to per-run ephemeral dir (see E5-12, E5-13, E5-14). Broader env inheritance is explicit opt-in |
| E5-04a | Placeholder injection (opt-in, unsafe) | **Post-MVP (Phase 1.5+).** `--unsafe-substitute` flag enables `{{SECRET}}` substitution in args. Prints stderr warning about `ps -eww` visibility. Non-interactive mode requires additional explicit acknowledgement flag |
| E5-05 | Output passthrough | stdout and stderr from the subprocess are returned to the caller |
| E5-06 | Exit code forwarding | The subprocess exit code is forwarded to the caller |
| E5-07 | Missing secret error | Running with a service that has no stored secret returns a clear error |
| E5-08 | Memory wipe | Decrypted secrets are zeroed from `secrecy::Secret` buffers immediately after subprocess execution (via `Zeroize` on drop) |
| E5-09 | No logging | Secret values are never written to any log, stdout, or stderr by Icebox |
| E5-10 | `--agent` flag | `icebox run` accepts `--agent <name>` to use a specific agent's vault for that invocation (defaults to active, does not mutate `activeAgentId`) |
| E5-11 | `--dry-run` flag | `icebox run --dry-run` shows the command that would be executed (with `***` masking the secret) without actually running it |
| E5-12 | TMPDIR creation (`0700`) | Create per-run temp dir via `tempfile::tempdir()`; immediately chmod to `0o700`; set as `TMPDIR` in subprocess env |
| E5-13 | TMPDIR cleanup (RAII + signal) | `TempDir` implements `Drop` (automatic cleanup); install signal handler for `SIGINT`/`SIGTERM` to exit cleanly |
| E5-14 | Stale TMPDIR startup sweep | On every Icebox CLI invocation, scan system temp dir for `icebox-run-*` directories older than 1 hour and remove them |
| E5-15 | Command provenance warning (MVP) | `icebox run` warns when command source/provenance is unknown or untrusted (trust-boundary reminder). Warning-only in MVP; superseded by enforced broker policy controls in E8 |

## E6 -- Zero-Exposure Hardening (Cross-Cutting)

> E6 is not a standalone phase. Its items are distributed into E1, E2, E3, and E5 so that
> hardening is built into each component from day one, not bolted on after.

| Concern | Implemented In | Backlog Items |
|---|---|---|
| Enclave-wrapping of Ed25519 key | E2 (Agent Identity) | E2-02, E2-03, E2-04, E2-19 |
| Enclave ACLs (code-signed binary) | E2 (Agent Identity) | E2-12 |
| Enclave access rate-limiting | E2 (Agent Identity) | E2-23, E2-25 |
| `secrecy` + `Zeroize` + `mlock` | E3 (Vault) | E3-07, E3-08 |
| Memory wipe after use | E5 (Secure Run) | E5-08 |
| No logging of secrets | E5 (Secure Run) | E5-09 |
| No temp files | E3 (Vault) | E3-09 |
| Subprocess isolation + env sanitization | E5 (Secure Run) | E5-02, E5-04 |
| TMPDIR hardening + cleanup | E5 (Secure Run) | E5-12, E5-13, E5-14 |
| Disable core dumps | E1 (Bootstrap) | E1-07 |
| No network calls | E1 (Bootstrap) | E1-08 |
| No clipboard access | E1 (Bootstrap) | E1-09 |
| File permission baseline (`0700`/`0600`) | E1 (Bootstrap) | E1-20 |
| Code signing + notarization | E1 (Bootstrap) | E1-15, E1-17, E1-18 |
| Vault load validation (non-cooperative writer defense) | E3 (Vault) | E3-13 |
| Vault HMAC (cross-restart integrity) | E3 (Vault) | E3-15, E3-16, E3-17, E3-18 |
| Local filesystem requirement | E3 (Vault) | E3-14 |

## E7 -- DID Support (Phase 1.5)

| ID | Use Case | Description |
|---|---|---|
| E7-01 | `did:key` derivation | As a user, I can run `icebox did claw` to get the agent's `did:key` identifier (DID commands start in Phase 1.5) |
| E7-02 | DID document | Generate a DID document JSON from the agent's keypair |
| E7-03 | `did:web` publishing | Generate a `.well-known/did.json` for web-hosted identity verification |
| E7-04 | DID verification | Verify a `did:key` matches a registered agent's public key |

## E7.5 -- Seed Backup & Portability (Phase 1.5)

> Deferred from MVP. Ships alongside E7 (DID Support). MVP has enclave-only keys with no recovery path.

| ID | Use Case | Description |
|---|---|---|
| E7.5-01 | `--seed` flag | `icebox register-agent claw --seed` generates keypair + 24-word BIP39 mnemonic |
| E7.5-02 | Seed display | Seed is displayed as QR code (terminal) + plaintext, then wiped from memory immediately |
| E7.5-03 | Seed never stored | Icebox never persists the seed to disk, logs, or clipboard |
| E7.5-04 | Recover agent | `icebox recover-agent claw --seed "word1 word2 ..."` re-derives keypair, creates new enclave wrapping key, stores wrapped blob |
| E7.5-05 | Default no-seed (MVP) | MVP `register-agent` (no `--seed`) creates an enclave-wrapped Ed25519 key with no recovery path. Phase 1.5 adds `--seed` as opt-in. |
| E7.5-06 | Export agent | `icebox export claw` bundles `vault.enc` + `identity.pub` + `manifest.json` + `bundle.manifest.json` into a versioned deterministic `.icebox-agent` archive (excludes `key.enc` and `hmac.enc` -- device-bound). `bundle.manifest.json` contains SHA-256 checksums for each included file. |
| E7.5-07 | Import agent | `icebox import claw.icebox-agent --seed "..."` verifies `bundle.manifest.json` checksums before processing, then imports the archive, re-derives the keypair from seed, and creates new enclave wrapping key |
| E7.5-08 | Deterministic import validation | Import fails fast on checksum mismatch, unsupported required fields/algorithms, duplicate IDs (`agentId`, `entryId`), or conflicting agent records unless explicit recovery mode is selected |

**Prerequisite:** File SLIP-44 registration now so it is approved by Phase 1.5. See [Architecture Overview](../architecture/).

## E8 -- Socket Server

| ID | Use Case | Description |
|---|---|---|
| E8-01 | Brokered execution default | Introduce brokered execution path where untrusted clients request approved operations and do not receive long-lived plaintext credentials |
| E8-02 | Policy-gated operations | Enforce deny-by-default policy with explicit allowlists for operation type, target host/service, and agent capabilities |
| E8-03 | Runtime egress controls | Apply subprocess/network/filesystem egress controls for broker-managed operation execution; fail closed on policy violation |
| E8-04 | Short-lived delegated credentials | Exchange long-lived stored credentials for short-lived, scoped, audience-bound tokens where provider supports it; never persist delegated token beyond operation scope |
| E8-05 | Unsafe raw-secret path quarantine | Move raw secret injection path behind explicit `--unsafe-raw-secret` mode with strong warning, audit event, and disabled-by-default policy |
| E8-06 | Credential metadata projection | Expose agent-visible credential metadata (`type`, `provider`, `hint`, `capabilities`) without any plaintext `value` field |
| E8-07 | Broker request/response schema contract | Require operation requests to use `credentialRef`; guarantee responses never include plaintext secret material |
| E8-08 | Response and error redaction contract | Ensure success/error payloads omit secret bytes, raw auth headers, and equivalent sensitive material |
| E8-09 | Deterministic policy error mapping | Map broker policy-deny/authz failures to stable user-safe `ICE-3xx` codes for debugging/support without secret disclosure: `ICE-301` policy deny (generic), `ICE-302` missing capability, `ICE-303` destination/action not allowlisted, `ICE-304` identity/attestation failure, `ICE-305` unsafe mode disabled by policy |
| E8-10 | Security mode profiles and precedence | Support explicit security profiles: `yolo`, `balanced`, `strict` with `balanced` as default global policy profile; allow per-agent overrides with precedence `per-agent` > `global` |
| E8-11 | Action-based 2FA policy | Enforce per-action 2FA policy matrix by mode (`api-call`, `form-fill`, `view-credential`, `send-credential`) with fail-closed behavior |
| E8-12 | Tool schema reference enforcement | Require broker-facing tools to use `credentialRef` + `credentialPlacement`; reject raw secret value fields in protected flows |
| E8-13 | Vault backend abstraction (built-ins first) | Implement backend interface for system keychain + encrypted file fallback now; defer external plugin ecosystem and track separately in `D2` (`#17`) |
| E8-14 | Security mode migration contract | Migration behavior: existing installs default to `balanced`, no silent cutover to `strict`, explicit opt-down to `yolo`, and deterministic user-visible migration messaging |

## E9 -- OpenClaw Skill

| ID | Use Case | Description |
|---|---|---|
| E9-01 | Broker-first skill integration | OpenClaw skill uses brokered operation APIs by default and never requests/export long-lived plaintext credentials |
| E9-02 | Policy and attestation handshake | Skill startup requires policy compatibility check and client identity/attestation handshake before privileged operations |
| E9-03 | Capability-scoped session contracts | Skill session grants least-privilege capabilities (`read-only inventory`, `run approved operation`) with explicit expiry and revocation |

## E10 -- Browser Extension

> Backlog items TBD -- will be detailed when Phase 3 begins.


---

*Last updated: 2026-03-03*
