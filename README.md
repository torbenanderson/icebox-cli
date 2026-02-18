# Icebox CLI

> Icebox gives AI agents powerful tools without storing secrets in agent state.

Secure credential broker for AI agents. Never expose API keys or passwords to [OpenClaw](https://openclaw.ai) (or any agent) -- local Mac CLI using Secure Enclave + encrypted vault.

> **WARNING -- ALPHA SOFTWARE**
> Icebox is in early alpha. While it is designed to be a security product, it has **not yet been independently audited**. There may be bugs, incomplete implementations, or undiscovered vulnerabilities. **Do not rely on Icebox as your sole security layer for production secrets.** Use with caution, review the code, and report any issues you find.

---

## Vision

Icebox is a **credential broker** for AI agents with strict secret-handling boundaries.

It is designed so secrets are encrypted at rest, unwrapped only at execution time, and never persisted in agent state.

## Why

AI agents need API keys to do useful work -- but most approaches leak those keys into agent memory, logs, or files. Icebox is the middleman that injects secrets into trusted command execution with explicit isolation controls.

*"Give your agent superpowers. Keep keys out of agent memory and logs."*

## Install

> **Coming soon.** Icebox is not yet released. Once E1 (bootstrap) is complete:

```bash
# Option 1: Cargo install (dev only — unsigned binary, limited enclave access)
cargo install icebox-cli

# Option 2: Download signed binary (macOS) — recommended for production
# curl -sSL https://icebox.my/install.sh | sh
```

Requires **macOS** (Apple Silicon or Intel T2) for full security flow in MVP. `~/.icebox/` must be on a **local filesystem** -- not iCloud Drive, Dropbox, NFS, or any synced/network drive (see [Architecture](docs/architecture/README.md) and [Vault & Integrity](docs/architecture/vault-and-integrity.md)).

## Key Features (MVP)

| Feature | Description | Status |
|---|---|---|
| **Agent Identity** | `icebox register-agent claw` -- creates Ed25519 keypair (Secure Enclave-wrapped) + isolated vault per agent | Planned |
| **Recovery Model (MVP)** | If a device is lost, regenerate provider API keys/tokens and re-add them to a new agent. Seed-based recovery is deferred. | MVP |
| **Seed Backup (Optional)** | `icebox register-agent claw --seed` -- 24-word recovery phrase for portability/cross-device recovery ([guide](docs/guides/BACKUP.md)) | Phase 1.5 |
| **Secure Vault** | Per-agent encrypted vault (`~/.icebox/identities/<name>/vault.enc`) using `crypto_box_seal` (libsodium-compatible) | Planned |
| **Add Secret** | `icebox add openai sk-...` -- encrypted to the agent's public key | Planned |
| **List Secrets** | `icebox list` -- shows service names (no values) | Planned |
| **Service Inventory** | `icebox list --services` outputs service names only (no secret values) for regeneration checklists | Planned |
| **Remove Secret** | `icebox remove openai` -- deletes a stored secret | Planned |
| **Run Secure Command** | `icebox run openai "curl ..."` -- decrypts, injects, runs, returns result | Planned |
| **Multi-Agent** | `--agent <name>` one-shot targeting on all commands + `use-agent <name>` to change persistent default; isolated vaults per agent | Planned |
| **DID Support** | `did:key` identity commands and `did:web` publishing | Phase 1.5 |
| **Secret-Handling Boundary** | Icebox keeps secrets out of long-lived agent state and injects them only at execution time into trusted subprocesses; those subprocesses can still exfiltrate via stdout/stderr/files/network | Planned |

## Current Status (February 15, 2026)

- This repository is currently **design and planning docs only**.
- The Rust implementation is not yet present in `src/`.
- Use these docs as the implementation contract for the bootstrap phase.

## Non-Functional Requirements

- **Platform:** macOS only (Apple Silicon + Intel T2; Secure Enclave hardware key wrapping)
- **Language:** Rust (single binary, no runtime, memory-safe by default)
- **License:** MIT (open source from day 1)
- **Security:** No logs, no clipboard, no outbound network from the `icebox` process in v1
- **Performance:** < 50ms overhead per credential use
- **Install:** `cargo install` (dev) or signed binary (production)

## Linux Status

- **Today (MVP):** Full-flow operation is macOS-only due to Secure Enclave dependency.
- **What works on Linux now:** contributors can build/test non-enclave paths (CLI/config/vault/crypto/schemas).
- **Linux full-flow plan:** post-`v0.1.1` discovery track (earliest Phase 2 planning), no committed GA date yet.
- **Candidate Linux key backends:** TPM-backed wrapping, OS keyring-backed wrapping, software-only fallback mode for CI/dev (lower security guarantees), and external hardware token paths (for example YubiKey via PIV/PKCS#11), all subject to evaluation.

See `docs/architecture/platform-and-distribution.md` for the canonical platform strategy.

## Roadmap

| Phase | Focus |
|---|---|
| **Phase 1 (MVP)** | CLI core -- agent identity, encrypted vault, secure run |
| **Phase 1.5** | DID support (`did:key` + `did:web`) + seed backup (`--seed`, recovery, export/import) |
| **Phase 2** | Unix socket server + OpenClaw skill integration |
| **Phase 3** | Browser extension (token-based login) |

## Documentation

- Docs index: [docs/README.md](docs/README.md)
- Architecture: [docs/architecture/README.md](docs/architecture/README.md)
- Planning: [docs/plan/README.md](docs/plan/README.md)
- Reference: [docs/reference/VERSIONING.md](docs/reference/VERSIONING.md), [docs/reference/error-codes.json](docs/reference/error-codes.json)
- Guides: [docs/guides/BACKUP.md](docs/guides/BACKUP.md)
- Contributing: [CONTRIBUTING.md](CONTRIBUTING.md)
- Security policy: [SECURITY.md](SECURITY.md)
- Code of conduct: [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)
- OpenClaw community on-ramp: [.github/OPENCLAW_COMMUNITY_ONRAMP.md](.github/OPENCLAW_COMMUNITY_ONRAMP.md)

### Local Docs Commands

```bash
# mdBook (guides/architecture/planning docs from docs/)
mdbook build
mdbook serve --open

# Rust API docs (from source comments /// and //!)
cargo doc --workspace --all-features --no-deps
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps

# Optional: run rustdoc examples as doctests
cargo test --doc
```

Rustdoc output path:

- `target/doc/icebox_cli/index.html`

### Local CI Commands

```bash
# Rust checks (matches .github/workflows/ci.yml)
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features

# Docs checks (matches .github/workflows/docs-site.yml)
mdbook build
cargo doc --workspace --all-features --no-deps
```

## Start Coding

- Planning index: [docs/plan/README.md](docs/plan/README.md)
- Pre-coding checklist: [docs/plan/START_CODING_CHECKLIST.md](docs/plan/START_CODING_CHECKLIST.md)
- Issue drafts for MVP Core: [docs/plan/BOOTSTRAP_ISSUES.md](docs/plan/BOOTSTRAP_ISSUES.md)


## First Run

The first time you run Icebox, you'll be prompted to name your agent:

```
Welcome to Icebox -- secure credential broker for AI agents.

No agents registered yet.

Enter a name for your first agent (e.g., claw, dev, my-openclaw):
> claw
Creating agent 'claw'...
```

Each agent gets its own identity and vault. Use `--agent <name>` to target a specific command invocation, or `icebox use-agent <name>` to change the persistent default.

## Backup & Recovery

Icebox stores rotatable credentials (API keys/tokens), not irreplaceable assets. In MVP, if a device is lost, the practical recovery flow is to register a new agent, regenerate keys from providers (OpenAI, GitHub, Stripe, etc.), and re-add them.

**Phase 1.5** adds optional seed-based recovery (`--seed`) for portability/cross-device workflows. MVP prioritizes hardening the core security model first. See the **[Backup & Recovery Guide](docs/guides/BACKUP.md)** for full details.

## Debugging

Icebox shows minimal, non-technical errors by default. For troubleshooting, add `--debug`:

```bash
icebox run openai "curl ..." --debug
```

**Do not use `--debug` in production or when sharing output** -- it includes internal paths and crypto details.

## Trust Boundary

`icebox run` should execute trusted commands only. Icebox controls secret handling in its own process and avoids persisting secrets in agent state, but the executed subprocess still receives the injected credential and can exfiltrate it via stdout/stderr, files, or network.

## License

[MIT](LICENSE)

---

**Version:** 0.1 (MVP) | [icebox.my](https://www.icebox.my) | **Date:** February 2026
