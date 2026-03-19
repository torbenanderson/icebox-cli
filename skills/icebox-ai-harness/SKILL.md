---
name: icebox-ai-harness
description: Enforce Icebox schema-contract propagation, CI workflow guardrails, and commit splitting by concern. Use when modifying JSON schemas/examples, architecture contracts tied to persisted artifacts, or `.github/workflows/*`.
---

# Icebox AI Harness

Use this skill for contract and automation integrity. Do not duplicate generic docs standards or kickoff alignment gates; those are handled by `icebox-docs-standards` and `icebox-execute`.

1. Contract propagation for persisted artifact changes:
   - Update canonical architecture contract docs under `docs/architecture/` first.
   - Update target-state contract schemas under `docs/architecture/contracts/*` and `docs/architecture/contracts/examples/*`.
   - Update current runtime machine-readable artifacts under `docs/reference/` when implementation-facing registries change.
   - Propagate to planning docs that track delivery/test impact: `docs/plan/BACKLOG.md`, `docs/plan/TESTING.md`, and `docs/plan/IMPLEMENTATION_BOOTSTRAP.md`.
2. CI workflow guardrails:
   - When changing `.github/workflows/*`, preserve build/test/docs coverage expectations.
   - Ensure docs pipeline compatibility remains intact with `book.toml` and docs structure.
3. Governance update hook:
   - Update `docs/reference/DOCS_GOVERNANCE.md` when process ownership/rules materially change.
4. Commit splitting by concern (non-interactive):
   - infra: `.github/workflows/*`
   - contracts: `docs/architecture/*`, `docs/architecture/contracts/*`, `docs/architecture/contracts/examples/*`
   - process: `docs/plan/*`, governance updates
