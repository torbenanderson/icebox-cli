---
name: icebox-guardrails
description: Enforce Icebox docs/CI guardrails. Use when adding or modifying architecture/planning docs, JSON schema contracts, GitHub Actions workflows, or ADR-worthy decisions, and when preparing commits that should be split by concern (infra, contracts, process).
---

# Icebox Guardrails

1. Update canonical architecture contracts first under `docs/architecture/`.
2. Propagate contract changes to `docs/plan/BACKLOG.md`, `docs/plan/TESTING.md`, and `docs/plan/IMPLEMENTATION_BOOTSTRAP.md`.
3. Update `docs/reference/schemas/*` and `docs/reference/schemas/examples/*` for persisted artifact changes.
4. Update `docs/reference/DOCS_GOVERNANCE.md` when process or ownership rules change.
5. Add or update ADR entries under `docs/architecture/decisions/` for decision-impacting changes.
6. Keep Markdown footer policy consistent in touched docs (`---` + `*Last updated: YYYY-MM-DD*`).
7. Split commits by concern:
   - infra: `.github/workflows/*`
   - contracts: schemas + architecture contracts
   - process: backlog/testing/bootstrap/governance/ADR updates
