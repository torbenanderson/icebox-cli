# Technology Choices

## Language & Runtime

- Rust, single-binary CLI, library-first internals.
- Rationale:
  - strong memory safety baseline,
  - deterministic resource management (important for secret lifetimes),
  - good platform interop for macOS Security.framework FFI.

## Platform Strategy

- Primary target: macOS with Secure Enclave integration.
- Non-macOS builds compile with enclave stubs for non-enclave paths and test coverage.
- Platform-specific code is isolated under `#[cfg]` boundaries.

## Cryptography Choices (MVP)

- Identity keys:
  - Ed25519 for identity/signing compatibility.
- Sealed secret payloads:
  - libsodium-compatible sealed-box model (`crypto_box_seal` wire compatibility target).
- Integrity:
  - HMAC-SHA256 for vault tamper detection metadata.
- Wrapping at rest:
  - Secure Enclave P-256 key wraps sensitive local key material.

## Crate Selection (Planned MVP Set)

- `ed25519-dalek`, `x25519-dalek`:
  - mature Rust implementations for key operations and conversions required by sealed-box flow.
- `xsalsa20poly1305` / sealed-box-compatible primitives:
  - align with existing sealed-box format expectations.
- `secrecy`, `zeroize`:
  - reduce accidental secret exposure in debug/logging and enforce buffer wipe on drop.
- `security-framework-sys` (or equivalent FFI surface):
  - direct access to Secure Enclave and Keychain APIs.
- `fs2` or `fs4`:
  - advisory file locking for vault write coordination.
- `nix`:
  - OS interfaces such as `RLIMIT_CORE` and filesystem checks.

## CLI/Architecture Libraries

- `clap` for CLI command/flag parsing.
- `serde` and `serde_json` for schema-bound persisted artifacts.
- `thiserror` (library) + `anyhow` (application boundary) for error layering.
- `tracing`/`tracing-subscriber` for controlled diagnostic mode.

## Security Crates

- Security-sensitive dependencies are pinned tightly in MVP:
  - crypto primitives,
  - secret-handling crates,
  - enclave FFI crates.
- Non-security infrastructure crates may remain version-flexible.

## Why These Choices (Tradeoff Summary)

- Prefer explicit, auditable primitives over highly abstract crypto wrappers.
- Keep local-first CLI architecture simple before introducing daemon/network complexity.
- Accept macOS-first constraints in exchange for stronger hardware-backed key wrapping.
- Keep domain logic testable independently from platform I/O and enclave FFI.

## Architectural Direction

- Keep platform FFI isolated.
- Keep domain logic testable and independent from I/O.
- Keep CLI thin; place behavior in library modules for easier testing and review.

## Non-Goals (MVP)

- No async runtime or long-lived daemon in Phase 1.
- No cross-platform secure-hardware parity in MVP.
- No seed/recovery/import/export command implementation in Phase 1.

## Related Docs

- `rust-implementation.md`
- `platform-and-distribution.md`
- `security-model.md`
- `README.md`


---

*Last updated: 2026-02-16*
