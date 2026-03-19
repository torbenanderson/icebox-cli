# Icebox Implementation Bootstrap

This document now defines sequencing and execution guardrails only.

Live completion state no longer belongs here. Use:

- [CURRENT_STATE.md](CURRENT_STATE.md) for the human-readable status snapshot
- [task-status.json](task-status.json) for machine-readable packet lifecycle state
- [STATUS_MODEL.md](STATUS_MODEL.md) for file-role and closeout rules

## Scope

- Phase 1 MVP remains split into:
  - MVP Core (`v0.1.0` internal validation slice)
  - MVP Hardening Pack (`v0.1.1` public-release gate)
- The canonical early path is still:
  - `register-agent -> add -> run -> remove`
- No seed/recovery/export/import commands in MVP
- No DID CLI commands in MVP

## Execution Guardrails

- Maintain an always-runnable slice: `register-agent -> add -> run -> remove`
- Do not block the runnable slice on HMAC, rollback protection, or deeper hardening layers
- Keep packet scope narrow; one packet should map to one backlog contract
- Keep packet status in the registry, not in this document

## Sequencing Model

1. Backlog contract is defined in [BACKLOG.md](BACKLOG.md).
2. Packet spec is created or refreshed under `docs/plan/spec/`.
3. Issue is loaded and moved through execution states.
4. Code, tests, docs, and architecture impact are delivered.
5. The done gate validates evidence, updates the status registry, archives the packet, and regenerates current-state docs.

## Runnable Slice Gate

This is the sequencing gate before deeper hardening work:

- `register-agent <name>` creates minimum usable local identity/config artifacts
- `add <service> <secret>` persists a secret in vault storage
- `run <service> <command>` resolves secret and executes with direct exec (no shell)
- `remove <service>` deletes persisted secret cleanly
- One integration test covers the full flow end-to-end

Current truth for whether this gate passes belongs in [CURRENT_STATE.md](CURRENT_STATE.md), not here.

## Hardening Layer Order

After the runnable slice gate is passing, apply hardening in this order:

1. Structured error-code coverage at CLI boundary
2. Atomic writes + file locking for config/vault
3. Owner-only filesystem permission enforcement (`0700` dirs, `0600` sensitive files)
4. Secure Enclave wrap/unwrap path
5. Vault integrity verification (`hmac.enc`, HMAC checks, rollback detection)
6. Runtime execution hardening (allowlist env, tempdir controls, provenance warnings)

## Public Release Boundary

`v0.1.0` remains an internal validation slice only.

Public `v0.1.1` still requires the hardening minimums already tracked in:

- [BACKLOG.md](BACKLOG.md)
- [TESTING.md](TESTING.md)
- [CI.md](CI.md)
- architecture docs under `docs/architecture/`

## Deferred Scope

Still deferred from MVP:

- DID CLI commands
- Seed backup/recovery/export/import
- Portable `.icebox-agent` import/export command surface
- Biometric toggle/config
- `--unsafe-substitute`
- Phase 2 policy enforcement templates/allowlists

---

*Last updated: 2026-03-18*
