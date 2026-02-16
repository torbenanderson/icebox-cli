# Secret Management & Run

This page defines the MVP command behavior contract for secret lifecycle operations
and `icebox run`.

## Secret Commands

- `add`, `list`, `remove` operate on the resolved target agent vault.
- `list --services` is an explicit service-inventory view (service names only, no values).
- Target resolution order:
  1. explicit `--agent <name>` (one-shot only),
  2. active agent from config (`activeAgentId`),
  3. fail with guidance when no active/selected agent exists.
- Name-based CLI flows resolve to internal `agentId` before vault operations.
- `--agent` must not mutate persistent default selection.

## Secret Lifecycle (MVP)

1. Resolve target identity and capabilities.
2. Load and validate vault artifact.
3. For write operations (`add`/`remove`):
   - acquire lock,
   - update entry set,
   - update integrity metadata,
   - write atomically.
4. For read operations:
   - return service metadata only (`list`, `list --services`) unless operation requires unseal (`run`).

## `run` Execution Model

### Core Contract

- Direct exec via `std::process::Command` only (no shell invocation).
- Command arguments are parsed/executed as process argv, not shell-expanded expressions.
- Secret injection is env-based in MVP default mode.
- Exit code is forwarded to caller.
- Subprocess stdout/stderr are passed through to caller.

### Environment Sanitization

- Allowlist-first env model for subprocess execution.
- Baseline preserved variables: `PATH`, `LANG`, `LC_*`, `TZ`.
- Explicitly stripped ambient variables include: `HOME`, `USER`, `LOGNAME`, `PWD`.
- Per-run ephemeral `TMPDIR` is created with owner-only permissions and injected.
- Secret value is injected only for subprocess scope, then dropped/zeroized in Icebox.

### Secret Env Var Naming

- MVP requires deterministic env var naming for `run`; this must be frozen in CLI UX contract before implementation completion.
- Until code freeze, treat env var naming as a compatibility-sensitive interface and document any final name/override behavior alongside CLI docs and tests.

## Trust Boundary

- Icebox protects secret lifecycle in its own process.
- Executed commands are still trusted code and may exfiltrate secrets.
- `run` does not sandbox subprocess network/filesystem behavior.

## Failure/Diagnostics Expectations

- Missing secret for service returns clear non-panicking error.
- Validation/integrity issues map to `ICE-201/202/203/204/205` as applicable.
- Default output avoids sensitive internals; debug mode may include additional context.

## Related Docs

- `security-model.md`
- `vault-and-integrity.md`
- `identity-and-enclave.md`
- `errors-and-diagnostics.md`
- `data-models-and-layout.md`
- `mvp-decision-lock.md`


---

*Last updated: 2026-02-16*
