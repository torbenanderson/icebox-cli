# Refactor Backlog

Optional source-code reductions and quality improvements. Consider during maintenance passes or when touching adjacent modules.

## src/ Line Reduction Opportunities

Target: ~60–70 lines (~4–5% of current ~1,570). Trade-offs: added dependency vs boilerplate, clarity vs brevity.

| Change | Est. savings | Notes |
|--------|--------------|-------|
| **thiserror for error types** | ~40–50 lines | Replace manual `Display` + `Error` impls in agent, config, enclave, hardening. Adds `thiserror` dep (lightweight, widely used). |
| **Enclave force-failure consolidation** | ~8 lines | `maybe_force_failure()` and `maybe_force_wrap_failure()` are nearly identical; extract `env_force_failure(var: &str)`. |
| **RegistrationCleanup::cleanup_on_error** | ~5 lines | Loop over `(path, created)` pairs instead of three separate calls. Loses per-file error messages unless extended. |
| **cleanup_file_if_created** | ~3 lines | Tighter `or_else` / match instead of explicit match arms. |
| **Config save chain** | ~3 lines | Extract small helper for write/flush/rename to reduce repetition. |

## E3 Maintenance Queue

Refactors intentionally deferred from execution packets to keep scope tight.

| ID | Refactor | Notes |
|---|---|---|
| E3-30 | Vault module split maintenance | Split `src/vault.rs` into focused modules (`mod.rs`, `crypto.rs`, `io.rs`, `paths.rs`) to isolate concerns (types/errors/crypto/locking/I/O) without changing runtime behavior. |
| E3-31 | Error-helper + conversion consolidation maintenance | Consolidate repeated helper/error-mapping patterns (for example `io_err`/`serialize_err` boilerplate and config->register conversion ownership) while preserving existing runtime error codes/messages. |

### Not recommended

- **Stripping doc comments** — Saves lines but hurts clarity and rustdoc.
- **Moving unit tests out of src/** — Shifts LOC to `tests/`, not net reduction.
- **error.rs format helpers** — Already compact; further reduction reduces readability.

## When to Tackle

- During a dependency update pass (e.g. add `thiserror` with other deps).
- When modifying error-handling paths (apply thiserror incrementally).
- Low-priority; defer if no other reason to touch the module.

---

*Last updated: 2026-03-04*
