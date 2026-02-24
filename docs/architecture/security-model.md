# Security Model

This page is the security entry point for Icebox MVP. It summarizes attacker assumptions,
assets, boundaries, and required controls, and links to deeper subsystem specs.

## Security Goals (MVP)

- Keep plaintext secrets and private keys off disk.
- Minimize plaintext lifetime in process memory.
- Detect vault tampering/corruption before security-critical operations.
- Avoid accidental command execution expansion (`run` must not invoke a shell).
- Reduce ambient process exposure (env sanitization, no outbound network from `icebox`).

## Assets

- Agent private key material (wrapped as `key.enc`).
- Vault secret material (`vault.enc` sealed entries).
- Vault integrity state (`hmac.enc`, `seq`, integrity metadata).
- Agent identity metadata (`manifest.json`, `identity.pub`, `config.json`).
- Device-binding metadata and backend references used for local protection.

## Trust Boundaries

- Boundary A: Icebox process.
  - Icebox enforces secret lifecycle guarantees only inside its own process.
- Boundary B: Spawned subprocess (`icebox run` target).
  - Subprocess is trusted-by-user code and may exfiltrate injected secrets.
- Boundary C: Local filesystem.
  - Security assumptions require local, owner-only storage semantics.

## Threat Model

### In Scope

- Offline disk attacker reading/modifying Icebox files.
- Local process attacker attempting env/process observation.
- Non-cooperative external writer racing or tampering with vault files.
- User/operator mistakes (unsafe permissions, wrong agent selection, untrusted `run` command).

### Out Of Scope (MVP)

- Full host compromise (kernel/root attacker can generally defeat process-level controls).
- Preventing exfiltration by trusted subprocesses.
- Remote attack surface in `icebox` itself (no outbound network contract in MVP).
- Hardware or platform trust breaks (for example compromised Secure Enclave/OS trust roots).

## Defenses

### Identity/Key Protection

- Identity branch (`K_identity`) is portable by contract; device branch (`K_device`) is local by contract.
- Local lane (MVP): Ed25519 private key is enclave-wrapped and stored only as `key.enc`.
- Runtime unwrap occurs only when needed for operation execution.
- Private key is never intentionally persisted in plaintext form.
- Paired/remote-signer lane (post-MVP): private identity key operations are delegated; caller receives operation results only.

### Vault Confidentiality + Integrity

- Per-entry sealed blobs in `vault.enc`.
- Canonical load validation pipeline with `ICE-201/202/203/204/205` mapping.
- Vault integrity key material is generated at registration and stored as enclave-wrapped `hmac.enc`.
- Security-critical vault operations require successful integrity verification using `hmac.enc`.
- Integrity checks and rollback signals are enforced on security-critical paths.
- Atomic write + lock discipline for read-modify-write vault updates.

### Execution Hardening

- `run` executes via direct `std::process::Command` (no shell).
- Allowlist-first subprocess environment model.
- Command output/exit propagation with secret handling bounded to Icebox process.

### Host/Runtime Hardening

- Core dumps disabled (`RLIMIT_CORE=0`).
- No outbound network from `icebox` process in MVP.
- Owner-only path/file mode requirements (`0700` dirs, `0600` sensitive files).

## Residual Risk

- Trusted subprocesses can exfiltrate injected secrets via network/files/stdout/stderr.
- Compromised signed binaries remain a hard threat class.
- Root-level local attackers can bypass many user-space protections.
- Coordinated rollback/downgrade defenses must be implemented correctly to avoid false assurance.
- For local unwrap flows, transient plaintext-in-memory windows cannot be reduced to zero in a normal user-space CLI model.

## Control Mapping (Where Spec Lives)

- Identity + enclave model: `identity-and-enclave.md`
- Vault validation + integrity semantics: `vault-and-integrity.md`
- Runtime command path + trust boundary: `secret-management-and-run.md`
- Data/file mode requirements: `data-models-and-layout.md`
- Error code surface: `errors-and-diagnostics.md`
- Implementation-level crate/flow details: `rust-implementation.md`

## Related Docs

- `overview.md`
- `secret-management-and-run.md`
- `vault-and-integrity.md`
- `identity-and-enclave.md`
- `mvp-decision-lock.md`


---

*Last updated: 2026-02-24*
