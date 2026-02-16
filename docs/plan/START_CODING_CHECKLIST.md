# Start Coding Checklist

Use this checklist before writing feature code.

## 1. MVP Core Acceptance Criteria (Freeze)

- [ ] `register-agent` works end-to-end with minimal local artifacts
- [ ] `add` stores a secret
- [ ] `run` executes with direct exec (no shell) and injected secret
- [ ] `remove` deletes the secret
- [ ] One integration test proves: `register-agent -> add -> run -> remove`

## 2. Decision Recording Process

- [ ] Keep locked architecture decisions current in `docs/architecture/mvp-decision-lock.md`
- [ ] Reference affected backlog items and tests in decision updates
- [ ] Use detailed commit bodies to capture rationale when no doc update is required

## 3. CLI UX Contract (Freeze Early)

- [ ] Define stdout/stderr conventions per command
- [ ] Define exit code behavior for success, input errors, runtime errors
- [ ] Define `--debug` output boundaries and safety rules

## 4. Test Fixture Strategy

- [ ] Add shared test helper for temporary Icebox home directory
- [ ] Avoid writing to real `~/.icebox` in tests
- [ ] Add deterministic helpers for timestamps/paths where feasible

## 5. Security Invariants (Must-Hold List)

- [ ] No plaintext secrets persisted to disk
- [ ] `run` path is no-shell direct exec
- [ ] Icebox output never prints secret values
- [ ] Vault rollback maps to `ICE-203`; integrity mismatch maps to `ICE-204`
- [ ] Runtime paths enforce owner-only modes (`0700` dirs, `0600` sensitive files)

## 6. CI Baseline

- [ ] Run `cargo fmt --check`
- [ ] Run `cargo clippy -- -D warnings`
- [ ] Run `cargo test`
- [ ] Enable Linux + macOS in CI early

## 7. Dependency Policy

- [ ] Pin security-sensitive crates tightly
- [ ] Document update cadence and review requirements
- [ ] Require changelog review for crypto/security dependency changes

## 8. Initial PR Sequence

1. `scaffold`: crate layout, CLI shell, error skeleton
2. `config-agent`: config + register/use/list/remove-agent basics
3. `vault-core`: add/list/remove with safe file writes
4. `run-core`: run command path + env injection + exit propagation
5. `error-surface`: complete ICE mapping and user-safe messages

## References

- `docs/plan/IMPLEMENTATION_BOOTSTRAP.md`
- `docs/plan/BACKLOG.md`
- `docs/plan/TESTING.md`
- `docs/architecture/README.md`


---

*Last updated: 2026-02-16*
