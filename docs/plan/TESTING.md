# Icebox Test Plan

> Test strategy and test backlog mapped to [BACKLOG.md](BACKLOG.md) user stories.
>
> **Implementation note:** This document references the planned `tests/` directory and files (for example `tests/integration.rs` and `tests/security.rs`). During planning, those files may not exist yet.

---

## Strategy

### Approach

- **Unit tests** live in `#[cfg(test)] mod tests { ... }` blocks alongside source in each module
- **Integration tests** live in the `tests/` directory for cross-module flows
- **CLI/E2E tests** run as user-facing binary flows under `tests/`
- **System/security tests** verify hardening guarantees and run under `tests/`
- **Enclave tests** are gated by `#[cfg(target_os = "macos")]` and require real Secure Enclave hardware (macOS only)
- Tests run on every push via CI (E1-04)

### Test Architecture (Rust-Conventional)

| Level | Purpose | Typical Scope | Default Location |
|---|---|---|---|
| **Unit** | Validate pure logic and small APIs | function/module behavior | Inline in `src/**` with `#[cfg(test)]` |
| **Integration** | Validate crate boundaries and module interaction | public API and domain flows | `tests/integration_*.rs` |
| **CLI/E2E** | Validate user-visible CLI behavior | args, exit codes, stdout/stderr, error UX | `tests/e2e_*.rs` |
| **System/Security** | Validate runtime hardening and OS contracts | permissions, env, platform gates, enclave boundaries | `tests/system_*.rs`, `tests/security_*.rs` |

### Test File Layout Conventions

- Keep unit tests inline in source modules.
- Keep integration/E2E/system tests under `tests/` using clear prefixes:
  - `integration_*.rs`
  - `e2e_*.rs`
  - `system_*.rs`
  - `security_*.rs`
- If the suite grows, use grouped top-level entry files plus shared modules:
  - entry files remain top-level under `tests/` (Cargo test discovery)
  - shared helpers can live in subfolders (for example `tests/common/`, `tests/fixtures/`)
- Avoid deeply nested standalone test files without top-level entry points; Cargo runs top-level `tests/*.rs` crates directly.

### Platform Constraints

| CI Runner | OS | What Runs | What's Skipped |
|---|---|---|---|
| macOS (Apple Silicon) | macOS | **All tests** including enclave integration, entitlement checks, Keychain ACL tests | Nothing |
| Linux (Ubuntu) | Linux | Vault crypto, sealed-box interop, DID derivation, CLI parsing, config, multicodec encoding | Enclave tests (`#[cfg(target_os = "macos")]`), TMPDIR `statfs` tests, Security.framework tests |

Tests in `tests/` that require the enclave use `#[cfg(target_os = "macos")]` and are automatically excluded on Linux. The stub (`backend_stub.rs`, `#[cfg(not(target_os = "macos"))]`) compiles on Linux and returns descriptive errors, allowing non-enclave code paths to be tested.

For local development on macOS, `cargo test` runs everything. On Linux, the same command compiles the stub and runs all non-enclave tests.

#### Runner Scope Boundaries (Strict Contract)

| Runner | Must include | Must skip |
|--------|--------------|-----------|
| **macOS** | All tests: unit, integration, E2E, system, security, enclave. `backend_darwin.rs` compiled and exercised. Entitlement/Keychain/Security.framework tests. | Nothing. |
| **Linux** | Unit, integration, E2E, system, security tests that do not touch enclave. Vault crypto, sealed-box, DID derivation, CLI, config. `backend_stub.rs` compiled. | Any code or test behind `#[cfg(target_os = "macos")]`. Enclave integration. TMPDIR `statfs` tests. Security.framework / Keychain tests. |

**Source contract:** Enclave code lives in `backend_darwin.rs` with `#[cfg(target_os = "macos")]`; non-macOS builds use `backend_stub.rs` with `#[cfg(not(target_os = "macos"))]`. CI workflows must not attempt to run macOS-only tests on Linux.

### Test Levels

| Level | Scope | Location | When to run |
|---|---|---|---|
| **Unit** | Single function / module | `src/<module>/mod.rs` inline `#[cfg(test)]` | Every push, every PR |
| **Integration** | Multi-module flows (register -> add -> run) | `tests/integration_*.rs` | Every push, every PR |
| **CLI/E2E** | End-user command behavior and exit semantics | `tests/e2e_*.rs` | Every push, every PR |
| **System/Security** | Hardening verification (memory, env, no-log, OS contracts) | `tests/system_*.rs`, `tests/security_*.rs` | Every push + nightly |

### Conventions

- Parameterized tests via macros or `rstest` where appropriate
- Rust stdlib `#[test]` + `assert!` macros for assertions (keeps the binary auditable)
- Temporary `~/.icebox/` test directories and enclave test keys per test (cleaned up after via `TempDir` RAII)
- No real secrets in test fixtures -- use generated test keypairs and dummy values
- Security tests should verify *absence* of leaks, not just *presence* of features
- Cryptographic interop tests (did:key vectors + sealed-box libsodium round-trips) are required merge gates in CI

### Usage

Common commands for local development and CI parity:

| Command | Purpose |
|---------|---------|
| `cargo test` | Run all tests (unit, integration, E2E, system, security). On macOS includes enclave tests; on Linux runs non-enclave only. |
| `cargo test -q` | Same as above, compact output (one character per test). |
| `cargo test --lib` | Run only unit tests in `src/`. |
| `cargo test --test <name>` | Run a single test crate (e.g. `integration`, `e2e`, `security`, `system`). |
| `cargo fmt --check` | Verify code is formatted (CI gate). |
| `cargo clippy -- -D warnings` | Lint with clippy, fail on warnings (CI gate). |
| `cargo check` | Type-check without full build (CI gate). |
| `cargo audit` | Check dependencies against RustSec advisory database. Install: `cargo install cargo-audit --locked`. |
| `cargo llvm-cov --summary-only` | Code coverage summary (which lines ran during tests). Install: `cargo install cargo-llvm-cov --locked`. |
| `cargo llvm-cov --html` | Generate HTML coverage report in `target/llvm-cov/html/index.html`. |
| `cargo mutants` | Mutation testing: mutate code and verify tests catch changes. Install: `cargo install cargo-mutants --locked`. On macOS, `backend_stub` is not compiled; run on Linux (or in CI) to cover stub mutations. |

### Phase 1 Vertical Slice Gate

Before full Phase 1 hardening is complete, maintain an always-green thin end-to-end gate:

- `register-agent` succeeds and creates identity artifacts
- `add` stores a secret in `vault.enc`
- `run` injects secret and executes command without shell
- `remove` deletes the secret cleanly

This gate is sequencing-only and does not replace the full backlog test matrix.

### MVP Core vs Post-MVP Hardening Test Gating

For release slicing:

- Treat tests mapped to deferred backlog items as **Post-MVP Hardening** gates (not MVP Core blockers).
- `v0.1.0` is internal validation only; first public GA (`v0.1.1`) requires MVP hardening minimums.
- Canonical implementation sequencing: [Implementation Bootstrap](IMPLEMENTATION_BOOTSTRAP.md).

### Public `v0.1.1` Blocker Matrix (P0)

These tests are public-release blockers and must pass on macOS CI before shipping `v0.1.1`:

- P0-1 No integrity downgrade path: `T-E3-17c`, `T-SEC-17`, `T-SEC-18`
- P0-2 Durable rollback protection across restart: `T-E3-13e`, `T-SEC-17`, `T-SEC-19`
- P0-3 Local filesystem default fail-closed: `T-E3-14`, `T-E3-14b`
- P0-4 Runtime hardening baseline: `T-E1-07`, `T-E1-08`, `T-E1-20`, `T-E5-04`, `T-E5-04b`, `T-SEC-08`
- P0-5 Signed/notarized + entitlement boundary: `T-E1-17`, `T-E1-18`, `T-SEC-15`, `T-SEC-16`

---

## Test Backlog

> Each test maps to a backlog item. Format: `T-<backlog-id>` = test for that user story.

### E1 -- Project Bootstrap

| Test ID | Backlog | Test Description |
|---|---|---|
| T-E1-01 | E1-01 | Integration tests verify Cargo scaffold happy path (`Cargo.toml` + `src/main.rs` exist, package name is `icebox-cli`) and failure path (`cargo metadata` fails for missing manifest path) |
| T-E1-02 | E1-02 | CLI scaffolding is wired with `clap`; E2E tests verify happy path (`--help` exits 0 with usage text) and failure path (unknown flag exits 2 with argument error) |
| T-E1-03 | E1-03 | Project structure modules exist and compile (`agent`, `config`, `vault`, `runner`, `did`) including platform-gated enclave split (`backend_darwin.rs` on macOS and `backend_stub.rs` elsewhere) |
| T-E1-04 | E1-04 | CI workflows validate push/PR gates on macOS and Linux; happy path: merge-blocking jobs pass (`check`, `fmt`, `clippy -D warnings`, `test`) and enhancement jobs/reporting run as configured, failure path: any merge-blocking check marks workflow red and blocks merge until fixed |
| T-E1-06 | E1-06 | `icebox --version` outputs version string, commit hash, and build date |
| T-E1-07 | E1-07 | Runtime hardening sets `RLIMIT_CORE=0` at startup (happy path) and returns a deterministic error when setting the limit fails (failure path) |
| T-E1-10 | E1-10 | Default error messages contain no internal paths, key material, or crypto details |
| T-E1-11 | E1-11 | `--debug` flag outputs detailed internal messages including paths and error codes |
| T-E1-12 | E1-12 | `--quiet` flag suppresses all non-essential output |
| T-E1-13 | E1-13 | Errors include structured codes (`ICE-XXX`) in both default and debug modes |
| T-E1-08 | E1-08 | No outbound network connections from the `icebox` process during any operation (verify via sandbox or socket monitoring) |
| T-E1-09 | E1-09 | No clipboard interaction during any operation (pasteboard unchanged before/after) |
| T-E1-17 | E1-17 | Release binary contains embedded entitlements: `com.apple.security.smartcard`, `com.apple.keychain-access-groups`, hardened runtime enabled. Verified via `codesign -d --entitlements -` |
| T-E1-18 | E1-18 | Release binary passes `spctl --assess --type execute` (Gatekeeper) and `stapler validate` (notarization staple) |
| T-E1-19 | E1-19 | `docs/reference/error-codes.json` exists, is valid JSON, and includes all documented `ICE-XXX` codes without duplicates |
| T-E1-20 | E1-20 | Runtime paths are owner-only: `~/.icebox/` and agent dirs are `0700`; sensitive files are `0600`; unsafe modes fail closed on security-critical commands |
| T-E1-21 | E1-21 | Persisted artifacts include correct `format` + `schemaVersion`; missing or mismatched markers are rejected |
| T-E1-22 | E1-22 | Canonical serialization checks pass (UTC RFC3339 timestamps, lowercase hex, fixed base64 variant, deterministic export ordering) |
| T-E1-23 | E1-23 | Unknown `x_icebox_*`/`x_vendor_*` extension fields round-trip unchanged |
| T-E1-24 | E1-24 | Compatibility policy enforced: `N/N-1` read-write, `N-2` read-only warning path, older versions fail with `ICE-202` |
| T-E1-25 | E1-25 | `createdByVersion` and `lastMigratedByVersion` are set/updated correctly across create and migrate flows |
| T-E1-26 | E1-26 | Deprecated fields/algorithms follow staged lifecycle behavior and warnings before removal |
| T-E1-27 | E1-27 | Golden fixture suite for manifest/vault/bundle passes byte-for-byte conformance checks |
| T-E1-28 | E1-28 | CI validates JSON Schemas against metaschema and validates fixtures against manifest/config/vault/bundle-manifest schemas |
| T-E1-29 | E1-29 | Decision-impacting PRs include ADR entries using `docs/architecture/decisions/ADR-TEMPLATE.md` and link affected docs/backlog/tests |
| T-E1-30 | E1-30 | Release candidate gate verifies signed binary has hardened runtime and required entitlements, then runs real `register-agent` Secure Enclave key creation on supported macOS hardware; public release is blocked on failure |
| T-E1-31a | E1-31 | Default behavior on unsupported/non-enclave backend remains fail-closed; registration does not silently fall back to insecure software backend |
| T-E1-31b | E1-31 | Explicit developer/insecure backend mode requires clear opt-in flag, emits prominent security warning, and marks runtime/backend metadata as non-enclave security level |
| T-E1-31c | E1-31 | In developer/insecure backend mode, CLI/docs surfaces never claim enclave-grade guarantees and support diagnostics distinguish this mode from secure local-enclave lane |

### E2 -- Agent Identity

| Test ID | Backlog | Test Description |
|---|---|---|
| T-E2-01 | E2-01 | `register-agent` creates `~/.icebox/identities/<name>/` and writes `identity.pub` (happy path); when identity setup fails, command exits non-zero with structured runtime error code |
| T-E2-02 | E2-02 | Happy path: `register-agent` creates a per-agent P-256 Secure Enclave wrapping key in `local-enclave` lane and records stable key-reference metadata; verify via Security.framework query. Failure path: forced key creation/access failure returns deterministic structured runtime error and non-zero exit. Non-exportability check: private key bytes are not returned via runtime/public API and are not written to disk. |
| T-E2-03 | E2-03 | Happy path: Ed25519 private key is wrapped with the E2-02 device-branch key and persisted as non-empty `key.enc` blob parseable by expected unwrap format; manifest linkage (`enclaveKeyRef`) remains coherent. Failure path: wrapping error returns deterministic structured runtime error and non-zero exit with no plaintext key spill. |
| T-E2-04 | E2-04 | Happy path: registration/wrap flow writes no plaintext Ed25519 private-key material to disk; only wrapped `key.enc` exists for identity private-key persistence in `local-enclave` lane. Failure path: any unsafe persistence attempt fails closed with deterministic structured runtime error; scan checks confirm no plaintext key bytes in agent artifacts. |
| T-E2-05a | E2-05 | `identity.pub` is exactly 34 bytes: first two bytes are `0xed 0x01` (Ed25519 multicodec varint), remaining 32 bytes are a valid Ed25519 public key |
| T-E2-05b | E2-05 | `manifest.json` `did` field equals `did:key:z` + base58btc encoding of the 34-byte `identity.pub` contents. Verified against W3C did:key test vectors |
| T-E2-05c | E2-05 | `manifest.json` `pubkeyFingerprint` equals lowercase hex SHA-256 of the 34-byte `identity.pub` contents |
| T-E2-05d | E2-05 | Cross-library round-trip: Icebox-generated `did:key` string is parseable by a reference JavaScript did:key resolver; extracted public key matches `identity.pub` bytes |
| T-E2-05e | E2-05 | Legacy compatibility: 32-byte `identity.pub` (raw Ed25519) is accepted in read path during migration window and does not panic/fail unexpectedly |
| T-E2-05f | E2-05 | Migration behavior: legacy 32-byte `identity.pub` upgrades to 34-byte multicodec format (`0xed01` prefix) deterministically; re-running migration is idempotent |
| T-E2-06 | E2-06 | `manifest.json` contains required v1 fields including immutable `agentId` plus reserved nullable fields (`keyAlgorithm`, `curve`, `didMethod`, `derivationScheme`, `coinType`, `network`, `keyPurposes`); unknown fields survive read/write round-trip |
| T-E2-07a | E2-07 | `icebox list-agents` reads from `config.json` `agents` array and returns all registered entries with `agentId`, name, and DID |
| T-E2-07b | E2-07 | If an agent directory exists in `identities/` but has no entry in `agents`, `list-agents` shows it as "unregistered" with a warning |
| T-E2-07c | E2-07 | If an entry exists in `agents` but its directory is missing, `list-agents` shows it as "missing" with a warning |
| T-E2-08a | E2-08 | Removing an agent deletes the directory, removes the enclave wrapping key, and removes the entry from `config.json` `agents` array |
| T-E2-08b | E2-08 | Removing the active agent clears `activeAgentId` in `config.json` |
| T-E2-09 | E2-09 | Registering a duplicate agent name (already in `agents` array) returns an error (not a silent overwrite) |
| T-E2-10 | E2-10 | First-run with empty `agents` array (or missing `config.json`) triggers an interactive prompt; validates name input |
| T-E2-10b | E2-10 | First-run in non-interactive mode fails with guidance to run `register-agent` (no prompt attempted) |
| T-E2-11a | E2-11 | After `register-agent`, `config.json` `activeAgentId` is set and `agents` array contains the new agent's `agentId`, name, and DID |
| T-E2-11b | E2-11 | Using `--agent <name>` does **not** mutate `activeAgentId`; persistent default remains unchanged |
| T-E2-13 | E2-13 | `icebox use-agent <name>` resolves name -> `agentId` and updates `activeAgentId` in `config.json` without performing vault operations |
| T-E2-26a | E2-26 | Create an orphaned directory (no registry entry); `icebox reconcile` detects and offers to add it |
| T-E2-26b | E2-26 | Add a registry entry with no directory; `icebox reconcile` detects and offers to remove it |
| T-E2-26c | E2-26 | Set a registry DID that doesn't match `manifest.json`; `icebox reconcile` detects and offers to update |
| T-E2-26d | E2-26 | `icebox reconcile --yes` auto-confirms all fixes without prompting |
| T-E2-12a | E2-12 | Enclave wrapping key is created with `kSecAttrAccessibleWhenUnlockedThisDeviceOnly` + `.privateKeyUsage`; verify via `SecItemCopyMatching` attributes |
| T-E2-12b | E2-12 | Enclave key is scoped to Icebox's Keychain access group; a different signed binary cannot access the key (returns `errSecItemNotFound` or `errSecAuthFailed`) |
| T-E2-12c | E2-12 | Unsigned binary attempting `SecKeyCreateDecryptedData` on Icebox's enclave key fails with an entitlement or auth error |
| T-E2-18 | E2-18 | Invalid agent names (too short, uppercase, special chars) are rejected with a clear error |
| T-E2-19 | E2-19 | Runtime unwrap: load `key.enc` -> enclave decrypt -> Ed25519 in secrecy Secret -> unseal works -> buffer dropped (zeroize) |
| T-E2-23 | E2-23 | Rapid successive enclave unwrap calls (< 200ms apart) are rejected after the first succeeds; verify cooldown enforced |
| T-E2-25 | E2-25 | Concurrent threads requesting unwrap for the same agent result in only one actual enclave call; all threads receive the same result |
| T-E2-27 | E2-27 | `enclaveAlgorithm` mismatch in `manifest.json` fails hard and requires explicit `migrate-enclave` flow |
| T-E2-28a | E2-28 | Legacy config/manifest with missing `agentId` is auto-migrated once; generated UUID/ULID is persisted to both files atomically |
| T-E2-28b | E2-28 | Legacy `activeAgent` name is migrated to `activeAgentId`; resulting active record matches previous name selection |
| T-E2-29 | E2-29 | Algorithm metadata enforces `preferred`/`allowed`/`deprecated` states; deprecated values remain readable through compatibility window |
| T-E2-30a | E2-30 | `manifest.json` with `type: \"agent\"` validates in MVP; reserved types are parseable but marked unsupported for runtime operations |
| T-E2-30b | E2-30 | Unknown `type` fails safely with deterministic unsupported-type error (no fallback to `agent`) |
| T-E2-31 | E2-31 | Authorization checks use capability flags; disabling `canRunCommands` blocks `run` even when `type` is `agent` |
| T-E2-32 | E2-32 | Internal identity resolution/services remain type-neutral while CLI continues to accept `agent`-named commands |
| T-E2-33 | E2-33 | Manifest/config lane metadata supports `local-enclave` and `paired-remote-signer`; unknown lane fails with deterministic unsupported-lane error |
| T-E2-34 | E2-34 | Device enrollment bindings preserve stable `agentId` identity while adding/removing per-device backend references |
| T-E2-35 | E2-35 | Protected operation contract returns deterministic state: `ok`, `pending_approval`, `denied`, or `expired` |

#### E2 Test-Harness Artifact Notes (Non-Production)

- `ICEBOX_TEST_FAKE_ENCLAVE=1` is a test harness mode only.
- In fake-enclave mode:
  - `enclave.keyref` is still a label string, but no real hardware key is created.
  - `key.enc` currently uses an internal test encoding: `fake-enclave-wrap-v1:` prefix plus byte-wise XOR payload.
- This fake encoding is not a production format guarantee and must not be used as a compatibility contract for real local-enclave artifacts.

### E3 -- Encrypted Vault

| Test ID | Backlog | Test Description |
|---|---|---|
| T-E3-21a | E3-21 | DID backend naming refactor keeps runtime behavior unchanged: backend resolution still returns deterministic identifiers and all existing E2/E3 tests continue to pass without command/output regressions |
| T-E3-21b | E3-21 | Invalid/corrupt `config.json` paths map to dedicated runtime error code (not generic identity setup); duplicate/validation/parse failures remain distinguishable and deterministic |
| T-E3-21c | E3-21 | `register-agent` refactor preserves cleanup invariants: failure in any artifact step leaves no partial unsafe state and does not write plaintext private key material |
| T-E3-01 | E3-01 | First `add` creates `vault.enc` as valid JSON containing format/version/encrypted entry; failure paths include missing `identity.pub` and missing active agent (`ICE-201` in MVP mapping) |
| T-E3-02 | E3-02 | Seal/unseal round-trip: encrypt a secret, decrypt it, verify plaintext matches; include interop check that Ed25519→X25519 conversion used by runtime remains libsodium-compatible; tampered on-disk sealed blob fails decryption |
| T-E3-03 | E3-03 | Two secrets sealed independently; decrypting one does not require or affect the other; each entry has unique immutable `entryId` |
| T-E3-04 | E3-04 | Tampered vault blob (flipped bit) is detected and rejected with an AEAD error |
| T-E3-05 | E3-05 | Empty vault returns clean state (empty list, no error) |
| T-E3-06 | E3-06 | Decryption fails gracefully when the wrong agent's key is used |
| T-E3-07 | E3-07 | Secret buffers are wrapped in `secrecy::Secret` with `Zeroize`; verify zeroization on drop (test helper) |
| T-E3-08 | E3-08 | Secret buffers are `mlock`'d (verify via `libc::mlock` on key buffers) |
| T-E3-09 | E3-09 | No temp files exist in `$TMPDIR` or agent directory during or after vault operations |
| T-E3-10 | E3-10 | `vault.enc` contains `"version": 1` field; parser rejects vault with missing/unknown version |
| T-E3-11 | E3-11 | Vault write creates `vault.enc.tmp` first, then renames atomically; interrupted write doesn't corrupt vault |
| T-E3-12 | E3-12 | Concurrent Icebox processes acquire flock and do not corrupt vault during simultaneous writes |
| T-E3-13a | E3-13 | Manually corrupt `vault.enc` JSON (truncate, add garbage); verify vault load returns `ICE-201` (parse failure) |
| T-E3-13b | E3-13 | Remove required top-level fields (`version`, `seq`, `entries`) from `vault.enc`; verify `ICE-202` error |
| T-E3-13c | E3-13 | Replace a vault entry's `sealedBlob` with a different base64 blob; verify AEAD rejection on unseal |
| T-E3-13d | E3-13 | Swap two entries' blobs between service keys; verify service name binding rejects on unseal |
| T-E3-13f | E3-13 | Create duplicate/invalid entry structures in `entries` (for example duplicate service or malformed entry object); verify `ICE-205` |
| T-E3-13e | E3-13 | Decrement `seq` in `vault.enc` while Icebox has a cached value; verify rollback rejection with `ICE-203` |
| T-E3-14 | E3-14 | Mock `statfs` returning an NFS filesystem type for `~/.icebox/`; verify security-critical command hard-fails by default |
| T-E3-14b | E3-14 | With explicit dev/test local-fs override enabled, verify warning-only behavior is clearly indicated |
| T-E3-15 | E3-15 | After `register-agent`, `hmac.enc` exists in agent directory and is a non-empty enclave-encrypted blob |
| T-E3-16a | E3-16 | After `icebox add`, `vault.enc` contains an `hmac` field (64-char hex string); verify HMAC is not all-zeros |
| T-E3-16b | E3-16 | After two consecutive `add` calls, the `hmac` field changes (different vault contents → different HMAC) |
| T-E3-17a | E3-17 | Replace `vault.enc` with an older copy (different `hmac` field); vault load returns `ICE-204` |
| T-E3-17b | E3-17 | Replace `vault.enc` contents but preserve the `hmac` field from the new file (content mismatch); vault load returns `ICE-204` |
| T-E3-17c | E3-17 | Remove `hmac.enc` from an MVP-created agent directory; vault load fails closed (no warning-and-proceed downgrade path) |
| T-E3-17d | E3-17 | `icebox list` does **not** trigger an enclave unwrap for HMAC verification (no biometric/enclave call on read-only listing) |
| T-E3-19 | E3-19 | `icebox list --strict` triggers HMAC verification and fails on integrity mismatch |
| T-E3-18 | E3-18 | Phase 1.5. After `recover-agent`, `hmac.enc` exists with a new enclave-encrypted HMAC key; first vault write produces a valid HMAC tag that passes verification on the next load |
| T-E3-20a | E3-20 | Fixture migration: load historical `vault.enc` schema fixture (`vN`) and migrate to current version (`vN+1`) without data loss |
| T-E3-20b | E3-20 | Re-running migration on already migrated vault is idempotent (no content change except expected metadata) |
| T-E3-20c | E3-20 | Unsupported vault version fails with `ICE-202` and does not mutate on-disk file |

### E4 -- Secret Management

| Test ID | Backlog | Test Description |
|---|---|---|
| T-E4-01 | E4-01 | `icebox add openai sk-test` stores a sealed blob; `icebox list` shows "openai" |
| T-E4-02 | E4-02 | `icebox list` shows service names only; no secret values in output |
| T-E4-02a | E4-02a | `icebox list --services` outputs only service names (one per line or deterministic stable format), with no secret values |
| T-E4-03a | E4-03 | Adding a secret for an existing service overwrites the sealed blob; verify the new value after unseal |
| T-E4-03b | E4-03 | After overwrite, `created` timestamp is preserved from the original entry; `updated` is set to the current timestamp (not `null`) |
| T-E4-03c | E4-03 | On first `add` (no existing entry), `created` is set to now and `updated` is `null` |
| T-E4-04 | E4-04 | `icebox remove openai` deletes the secret; `icebox list` no longer shows it |
| T-E4-05 | E4-05 | Piping a secret via stdin works: `echo "sk-test" \| icebox add openai` |
| T-E4-06 | E4-06 | Invalid service names (spaces, special chars) are rejected with a clear error |
| T-E4-07 | E4-07 | `--agent` flag targets the correct agent's vault (add to agent A, not visible in agent B) |
| T-E4-08 | E4-08 | Passing a secret as CLI arg triggers a shell history warning on stderr |
| T-E4-09 | E4-09 | `copy-services --from A --to B` re-seals all services to agent B and preserves per-service metadata invariants |

### E5 -- Secure Run

| Test ID | Backlog | Test Description |
|---|---|---|
| T-E5-01 | E5-01 | `icebox run openai "echo test"` returns "test" on stdout |
| T-E5-02 | E5-02 | Command is executed via `std::process::Command` without shell; shell metacharacters are not interpreted |
| T-E5-03 | E5-03 | Secret is available as env var in subprocess; not visible via `ps` (i.e., not in process args) |
| T-E5-04 | E5-04 | Subprocess inherits `PATH`, `LANG`, `TZ` but not `HOME`, `USER`, `LOGNAME`, `PWD` |
| T-E5-04a | E5-04a | Phase 1.5+. `--unsafe-substitute` flag enables `{{SECRET}}` substitution in command args; verify stderr warning is printed |
| T-E5-04b | E5-04 | Env is allowlist-first by default; non-allowlisted parent env vars are not inherited unless explicit override flag is used |
| T-E5-04c | E5-04a | Phase 1.5+. Non-interactive `--unsafe-substitute` without explicit acknowledgement flag fails |
| T-E5-05 | E5-05 | Subprocess stderr is captured and returned alongside stdout |
| T-E5-06 | E5-06 | Non-zero subprocess exit code is forwarded to the caller |
| T-E5-07 | E5-07 | Running with a missing service name returns a clear error (not a crash) |
| T-E5-08 | E5-08 | After `run`, secret buffer is dropped (verify `Zeroize` on drop) |
| T-E5-09 | E5-09 | Secret value does not appear in Icebox's own stdout, stderr, or any log output |
| T-E5-10 | E5-10 | `--agent` flag targets the correct agent's vault for decryption |
| T-E5-12 | E5-12 | Per-run TMPDIR is created with `0700` permissions; subprocess receives it as `TMPDIR` env var |
| T-E5-13 | E5-13 | After normal `icebox run` completion, the per-run TMPDIR directory no longer exists |
| T-E5-14 | E5-14 | Create a stale `icebox-run-*` directory >1 hour old; verify Icebox removes it on startup |
| T-E5-14b | E5-14 | Stale TMPDIR sweep runs on non-`run` commands too (every CLI invocation) |
| T-E5-15 | E5-15 | MVP path: `icebox run` emits trust-boundary warning when command provenance is unknown/untrusted (warning-only; no enforcement) |

### E7 -- DID Support (Phase 1.5)

| Test ID | Backlog | Test Description |
|---|---|---|
| T-E7-01 | E7-01 | `icebox did claw` outputs a valid `did:key:z...` string matching the format `did:key:z<base58btc(0xed01 \|\| pubkey)>`; matches `manifest.json` `did` field |
| T-E7-02 | E7-02 | Generated DID document is valid JSON and contains the correct public key |
| T-E7-03 | E7-03 | `did:web` output is a valid `.well-known/did.json` document |
| T-E7-04 | E7-04 | Verifying a `did:key` against the agent's stored pubkey returns true; wrong key returns false |

### E7.5 -- Seed Backup & Portability (Phase 1.5)

| Test ID | Backlog | Test Description |
|---|---|---|
| T-E7.5-01 | E7.5-01 | `--seed` generates a valid 24-word BIP39 mnemonic alongside the keypair |
| T-E7.5-02 | E7.5-02 | Seed is displayed as QR code + plaintext; seed is wiped from memory after display |
| T-E7.5-03 | E7.5-03 | Seed never appears on disk, in logs, or in clipboard after registration |
| T-E7.5-04 | E7.5-04 | `recover-agent` with a valid seed re-derives the same keypair, creates new enclave wrapper, passes seal/unseal round-trip |
| T-E7.5-05 | E7.5-05 | Default (no `--seed`) creates an enclave-wrapped key; no mnemonic is output |
| T-E7.5-06 | E7.5-06 | `icebox export claw` creates versioned deterministic `.icebox-agent` archive containing `vault.enc`, `identity.pub`, `manifest.json`, `bundle.manifest.json`; does NOT contain `key.enc`, `hmac.enc`, or plaintext keys |
| T-E7.5-07 | E7.5-07 | `icebox import` verifies `bundle.manifest.json` checksums before processing; with valid seed it restores agent from archive and secrets decrypt on new device |
| T-E7.5-08 | E7.5-08 | Import rejects checksum mismatch, unsupported required fields/algorithms, duplicate `agentId`/`entryId`, and conflicting records unless explicit recovery mode is set |

### E8 -- Socket Server (Phase 2)

| Test ID | Backlog | Test Description |
|---|---|---|
| T-E8-01 | E8-01 | Brokered execution performs approved remote/API operation without disclosing long-lived plaintext credential to client process |
| T-E8-02 | E8-02 | Deny-by-default policy rejects operations not explicitly allowlisted by action + destination + capability |
| T-E8-03 | E8-03 | Broker egress controls block non-approved network/filesystem targets and emit deterministic policy error |
| T-E8-04 | E8-04 | Delegated token flow issues short-lived scoped token, enforces TTL/audience/scope, and clears token material after operation |
| T-E8-05 | E8-05 | Raw secret injection requires explicit `--unsafe-raw-secret`; default mode rejects attempt and logs policy audit event |
| T-E8-06 | E8-06 | Credential metadata API returns `type/provider/hint/capabilities` and never returns plaintext `value` |
| T-E8-07 | E8-07 | Broker operation requests require `credentialRef`; broker responses never include plaintext secret material |
| T-E8-08 | E8-08 | Success and error payload redaction removes secret bytes and auth-header material from returned objects/messages |
| T-E8-09 | E8-09 | Policy-deny/authz failures map to stable `ICE-3xx` codes: generic deny=`ICE-301`, missing capability=`ICE-302`, destination/action deny=`ICE-303`, attestation/auth failure=`ICE-304`, unsafe mode blocked=`ICE-305`; mapping is deterministic across repeated runs |
| T-E8-10 | E8-10 | Security mode selection applies deterministic behavior with global default (`balanced`) and per-agent override precedence (`per-agent` wins over `global`) |
| T-E8-11 | E8-11 | Action-based 2FA matrix enforced by mode: `api-call`, `form-fill`, `view-credential`, `send-credential` produce expected auto/2FA/blocked outcomes |
| T-E8-12 | E8-12 | Tool payload containing raw credential value field is rejected; `credentialRef` + `credentialPlacement` payload is accepted |
| T-E8-13 | E8-13 | Built-in backend abstraction supports system keychain and encrypted-file fallback while external plugin backends remain disabled/deferred |
| T-E8-14 | E8-14 | Migration rules enforced: existing installs default to `balanced`, no silent `strict` cutover, explicit opt-down to `yolo`, and deterministic user-visible migration notice |

### E9 -- OpenClaw Skill (Phase 2)

| Test ID | Backlog | Test Description |
|---|---|---|
| T-E9-01 | E9-01 | OpenClaw skill uses broker API flow and cannot retrieve/export stored long-lived credential values |
| T-E9-02 | E9-02 | Skill invocation fails closed when policy compatibility/attestation handshake is missing or invalid |
| T-E9-03 | E9-03 | Session capability grant enforces least privilege; revoked or expired grants are rejected deterministically |

---

## Integration Tests

Cross-cutting flows that span multiple epics:

| Test ID | Flow | Description |
|---|---|---|
| T-INT-01 | Full lifecycle | Register agent -> add secret -> list -> run -> remove secret -> remove agent |
| T-INT-02 | Multi-agent isolation | Register two agents; add same service to each with different secrets; verify isolation |
| T-INT-03 | Seed recovery round-trip (Phase 1.5) | Register with `--seed` -> add secrets -> delete `key.enc` + enclave key -> recover from seed -> verify secrets decrypt |
| T-INT-04 | Wrong agent decryption | Add secret to agent A; attempt `run` with agent B; verify failure with clear error |
| T-INT-05 | First-run flow (interactive only) | Fresh `~/.icebox/` with TTY; run `icebox add` -> verify prompt triggers -> agent created -> secret stored |

### Vertical Slice Integration Gate (Early)

| Test ID | Flow | Description |
|---|---|---|
| T-INT-VS-01 | MVP thin slice | Non-interactive setup -> `register-agent` -> `add` -> `run` -> `remove`; verifies end-to-end behavior before full hardening matrix is complete |

## Security Tests

Dedicated tests that verify hardening guarantees:

| Test ID | Concern | Description |
|---|---|---|
| T-SEC-01 | No disk leaks | After a full lifecycle, no plaintext secrets exist anywhere in `~/.icebox/` or `$TMPDIR` |
| T-SEC-02 | Memory wipe | Verify `secrecy::Secret` / `Zeroizing` buffers are zeroed after use (verify `Zeroize` on drop) |
| T-SEC-03 | No env leaks | After `run`, the secret is not present in the parent process environment |
| T-SEC-04 | Error safety | Trigger every error path; verify no error message contains a plaintext secret or key |
| T-SEC-05 | Core dump disabled | Verify `RLIMIT_CORE == 0` in the running process |
| T-SEC-06 | Tamper detection | Modify vault bytes on disk; verify AEAD rejection |
| T-SEC-07 | Secret lifecycle | Verify all `secrecy::Secret` / `Zeroizing` instances are properly dropped; no buffer leaked after full lifecycle |
| T-SEC-08 | Subprocess env clean | After `run`, verify subprocess env contains only allowed vars (`PATH`, `LANG`, `TZ`, injected secret) |
| T-SEC-09 | No shell execution | Inject shell metacharacters (`; rm -rf /`, `$(whoami)`, backticks) in command; verify they are treated as literals |
| T-SEC-10 | TMPDIR permissions | After `icebox run`, verify the per-run temp dir (while it exists) has mode `0700` and is owned by current user |
| T-SEC-11 | TMPDIR cleanup on signal | Send `SIGTERM` to `icebox run` during subprocess execution; verify the per-run temp dir is cleaned up |
| T-SEC-12 | External vault overwrite | While Icebox holds `flock`, overwrite `vault.enc` from an external process (bypassing lock); verify Icebox detects the inconsistency on next load |
| T-SEC-13 | Non-cooperative writer defense | Write a syntactically valid but semantically wrong `vault.enc` (wrong schema, missing `seq`); verify Icebox rejects it cleanly without panicking |
| T-SEC-14 | Enclave abuse resistance | In a loop, call the enclave unwrap path as fast as possible; verify cooldown engages (calls within 200ms rejected), failure backoff doubles the interval, and single-flight coalescing prevents concurrent amplification |
| T-SEC-15 | Hardened runtime | Verify release binary has hardened runtime enabled (`codesign --display --verbose`; look for `runtime` flag). Verify DYLD environment variable injection is blocked. |
| T-SEC-16 | Enclave entitlement | Attempt enclave operations with a binary that lacks `com.apple.security.smartcard` entitlement; verify `errSecMissingEntitlement` (or equivalent) is returned |
| T-SEC-17 | Cross-restart vault rollback | Write a vault (state A), add a secret (state B), then replace `vault.enc` with state A (but leave `hmac.enc` untouched from state B). Restart Icebox; verify `ICE-204` on next security-critical vault load. |
| T-SEC-18 | HMAC key tamper | Corrupt `hmac.enc` (flip a byte); verify vault load fails with an enclave decryption error (not `ICE-204` -- the HMAC key itself can't be unwrapped). |
| T-SEC-19 | Coordinated vault + HMAC rollback | Roll back both `vault.enc` and `hmac.enc` to a consistent older state. Verify rollback is detected across process restart via persisted monotonic integrity anchor (not process-local cache only). |
| T-SEC-20 | Reconcile fail-closed | Introduce registry/filesystem drift, then run `add`/`run`/`remove`; verify commands fail until `icebox reconcile` resolves drift |
| T-SEC-21 | No plaintext export in broker mode | Attempt credential-read APIs from untrusted client path; verify broker never returns long-lived plaintext secret material |
| T-SEC-22 | Policy bypass resistance | Attempt non-allowlisted host/action combinations and command-shape tampering; verify deny with deterministic policy code |
| T-SEC-23 | Delegated token containment | Verify delegated tokens are short-lived, capability scoped, and rejected outside audience/TTL/policy constraints |
| T-SEC-24 | Unsafe mode governance | Verify `--unsafe-raw-secret` is disabled by default and requires explicit policy flag plus user-visible warning to execute |


---

*Last updated: 2026-03-03*
