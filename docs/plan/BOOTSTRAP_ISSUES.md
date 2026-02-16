# Bootstrap Issue Drafts

Copy these into GitHub issues for MVP Core implementation.

## 1. Scaffold CLI Crate

**Title:** `scaffold: initialize Rust crate and module layout`

**Goal:** Create a compilable crate with thin `main.rs` and library-first module layout.

**Includes:**
- `Cargo.toml` baseline
- `src/main.rs`, `src/lib.rs`
- module files for `cli`, `config`, `agent`, `vault`, `runner`, `error`

**Done when:**
- `cargo check` passes
- `icebox --help` works

## 2. CLI Contract Surface

**Title:** `cli: add MVP core commands and global flags`

**Goal:** Implement command/flag parsing only (no deep logic).

**Includes:**
- global flags: `--debug`, `--agent`
- commands: `register-agent`, `use-agent`, `list-agents`, `add`, `list`, `remove`, `run`, `remove-agent`

**Done when:**
- all commands parse
- invalid arguments produce `ICE-701`-mapped errors

## 3. Config + Agent Basics

**Title:** `config: implement active agent registry and local config storage`

**Goal:** Add minimal config read/write and active-agent selection behavior.

**Includes:**
- create/read/write `config.json`
- `register-agent` and `use-agent` minimum behavior
- `--agent` remains one-shot and does not mutate config

**Done when:**
- `register-agent` creates config + agent entry
- `use-agent` updates `activeAgentId`
- tests cover one-shot `--agent` semantics

## 4. Vault Core Persistence

**Title:** `vault: implement add/list/remove with atomic writes`

**Goal:** Build non-crypto core persistence first for MVP runnable slice.

**Includes:**
- vault file create/load/save
- `add`, `list`, `remove`
- atomic write path + lock file baseline

**Done when:**
- `add/list/remove` pass integration test
- corruption from partial write is prevented by temp+rename strategy

## 5. Run Core Path

**Title:** `run: direct exec with env injection and exit propagation`

**Goal:** Implement runnable core command execution path.

**Includes:**
- resolve service secret
- execute without shell
- inject secret via env var
- forward stdout/stderr + exit code

**Done when:**
- integration test passes: `register-agent -> add -> run -> remove`
- shell metacharacters are treated literally in command args

## 6. Error Surface + Registry Validation

**Title:** `errors: implement ICE code mapping and registry consistency checks`

**Goal:** Ensure stable user-safe error contract.

**Includes:**
- map core failures to ICE codes
- safe default messages, debug detail in debug mode
- test/validation against `docs/reference/error-codes.json`

**Done when:**
- code-to-message mapping is deterministic
- tests verify `ICE-203` vs `ICE-204` semantics

## 7. CI Baseline

**Title:** `ci: add fmt/clippy/test gates on Linux and macOS`

**Goal:** Start enforcement early while scope is still small.

**Includes:**
- GitHub Actions workflow
- `fmt`, `clippy`, `test`
- matrix for Linux + macOS

**Done when:**
- PRs require passing checks before merge

## 8. Post-MVP Hardening Tracking

**Title:** `planning: track Post-MVP hardening pack items`

**Goal:** Keep deferred hardening visible after MVP Core internal validation (`v0.1.0`) completes.

**Includes:**
- track: `E1-12`, `E2-23`, `E2-25`, `E2-26`, `E3-14`, `E3-19`, `E4-09`, `E5-04a`, `E5-14`, `E5-15`
- connect each item to tests and release milestone

**Done when:**
- all deferred items exist as linked issues with owners/milestones


---

*Last updated: 2026-02-16*
