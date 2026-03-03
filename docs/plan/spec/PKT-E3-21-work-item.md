# E3-21 Execution Spec

## Objective

- Deliver E3-21 (Identity/config refactor baseline).
- Backlog contract: Refactor identity/config foundations before broader E3 delivery: clarify DID backend naming (non-enclave behavior labels), tighten config-to-runtime error mapping (invalid config gets dedicated code path), split `register-agent` into smaller units, and reduce duplicate canonical-name scans with reusable/cached canonical sets.

## Problem

- Current naming and error-mapping boundaries increase maintenance risk before E3 vault work expands surface area.
- `register-agent` currently carries multiple responsibilities in one flow, increasing regression risk and cleanup complexity.
- Canonical-name scans are repeated in loops and may drift across call sites over time.

## Scope

- In scope:
  - DID module naming clarity:
    - Rename `did/enclave_darwin.rs` and `did/enclave_stub.rs` to backend-oriented names, or equivalent structure, to reflect they are backend identifiers only.
  - Config/runtime error mapping:
    - Stop collapsing config failures into generic I/O where error category is known.
    - Route invalid config paths to a dedicated runtime code path instead of generic identity-setup mapping.
  - `register-agent` refactor:
    - Split monolithic flow into smaller helpers or equivalent guard-based structure while preserving existing behavior and cleanup guarantees.
  - Canonical name lookup efficiency:
    - Add reusable canonical-name set/utility in config layer to avoid repeated per-entry canonicalization loops.
  - Utility placement review:
    - Normalize placement for generic helpers vs DID-specific helpers to reduce module ambiguity.
- Out of scope:
  - New user-facing commands or behavior outside existing E2/E3 contracts
  - Cryptographic algorithm changes
  - Cross-epic architecture changes unrelated to identity/config refactor baseline

## Acceptance Criteria

- AC1: DID backend identifier modules use names that match actual behavior (backend labeling, not enclave implementation); runtime outputs remain unchanged.
- AC2: Invalid/corrupt config mapping reaches the dedicated runtime invalid-config code path (`ICE-309`, not fallback generic identity setup), with deterministic user-safe messaging.
- AC3: Config error categories remain distinguishable through runtime mapping (parse/validation/serialize where applicable), without fragile string matching.
- AC4: `register-agent` logic is decomposed into smaller units (or equivalent guard pattern) while preserving existing cleanup/no-plaintext invariants.
- AC5: Canonical-name duplicate checks use a single reusable canonical utility/set to keep behavior aligned with E2-18; consistency and single-source validation are required, while additional caching/optimization is optional if behavior remains deterministic.
- AC6: Mapped tests validate unchanged external behavior and failure-path correctness.

## Rust Implementation Plan

- Crate/module touch points:
  - `src/did/*` (backend naming/layout)
  - `src/agent.rs` (register flow decomposition)
  - `src/config.rs` (canonical-name utility and config validation/error boundaries)
  - `src/error.rs` and `src/lib.rs` (runtime mapping for config-invalid paths)
- Error handling:
  - Keep typed error variants end-to-end where practical.
  - Avoid message-text matching for semantic classification.
  - No `unwrap()`/`expect()` in runtime code paths.
- I/O and side-effect boundaries:
  - Preserve atomic/checked writes and cleanup behavior already established in E2.
  - Preserve fail-closed behavior for invalid config state.

## Security/Runtime Notes

- This packet is a structural refactor baseline; it must not weaken no-plaintext-on-disk guarantees.
- Preserve existing default-mode safe messaging and debug-mode detail boundaries.
- Preserve deterministic behavior for duplicate guard and canonical-name parsing alignment.

## Test Mapping

- Linked tests from `docs/plan/TESTING.md`:
- T-E3-21a
- T-E3-21b
- T-E3-21c
- Add at least:
  - one happy-path regression test
  - one failure-path test per error-surface change

## ADR Triage

- ADR required? (no)
- Rationale: refactor stays inside existing architecture boundaries and does not introduce a new long-lived architecture decision.

## Docs Impact

- [x] docs/plan/BACKLOG.md
- [x] docs/plan/backlog.md
- [x] docs/plan/spec/PKT-E3-21-work-item.md
- [x] docs/plan/TESTING.md
- [ ] docs/architecture/decisions/ADR-*.md (if scope changes during execution)
- [ ] docs/README.md (if user-facing behavior changes)

## Validation Commands

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`

## Execution Notes

- Commit split plan will be finalized in the issue `Execution Plan` comment during `execute`.

---
*Last updated: 2026-03-03*
