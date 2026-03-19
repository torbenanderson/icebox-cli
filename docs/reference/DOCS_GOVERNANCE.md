# Documentation Governance

This file defines documentation ownership and hygiene rules.

## Source Of Truth By Concern

- `docs/architecture/`:
  Technical architecture contracts and subsystem behavior.
- `docs/architecture/decisions/`:
  Forward-only ADR log for new architecture decisions.
- `docs/architecture/mvp-decision-lock.md`:
  Locked implementation decisions and rationale summaries.
- `docs/plan/`:
  Execution sequencing, milestones, backlog, packet lifecycle state, and test planning.
  - Scope/requirements: `docs/plan/BACKLOG.md`
  - Lifecycle/naming rules: `docs/plan/STATUS_MODEL.md`
  - Human status snapshot: `docs/plan/CURRENT_STATE.md`
  - Machine-readable status: `docs/plan/task-status.json`
- `docs/guides/`:
  User/operator guides.
- `docs/process/`:
  Process governance, lifecycle gates, and discussion-proposal workflow (D-issues).
- `docs/reference/`:
  Current machine-readable runtime artifacts and policy references.
- `docs/architecture/contracts/`:
  Planned and target-state machine-readable contracts for persisted artifacts.

## Size And Duplication Guardrails

- Prefer linking to canonical docs over repeating policy text.
- If a page exceeds ~300 lines, consider splitting by concern.
- If a rule appears in 3+ files, keep one canonical copy and backlink elsewhere.
- Keep "what", "how", and "why" separated:
  - what = architecture/plan contracts
  - how = implementation/bootstrap details
  - why = `docs/architecture/mvp-decision-lock.md` (primary), commit history (supporting trail)

## Footer Policy

- Every Markdown doc under `docs/` must end with:
  - `---`
  - `*Last updated: YYYY-MM-DD*` (ISO date)
- Update this date whenever the file content changes.

## PR Hygiene Rule

When a PR changes docs, include:

1. Canonical file changed.
2. Secondary docs linked (or explicitly noted as not needed).
3. Any moved/renamed path references updated.
4. Required PR sections: `Packet Metadata`, `Objective`, `Validation`, `Done When`, `Closeout Evidence`.
5. Packet metadata fields: backlog ID(s) (epic and/or packet), issue reference(s), spec path(s), test ID(s), docs touched.
6. Closeout evidence fields: PR link, tests run, docs updated, architecture docs updated, files changed, ADR link (`n/a` allowed).
7. If planning/process rules change, update:
   - `docs/plan/STATUS_MODEL.md`
   - `docs/reference/DOCS_GOVERNANCE.md`
   - any affected workflow/skill templates

## Docs Build Contract

- mdBook configuration lives at repository root in `book.toml`.
- The correct local build command is:
  - `mdbook build`
- Do not refer to `docs/book.toml` or `mdbook build docs` unless the repository layout actually changes.

## Review Checklist

- [ ] Links resolve after file moves.
- [ ] Decision-impacting changes update `docs/architecture/mvp-decision-lock.md` when needed.
- [ ] Decision-impacting changes add/update ADRs under `docs/architecture/decisions/` when applicable.
- [ ] Persisted-artifact contract design changes update `docs/architecture/contracts/` + fixtures.
- [ ] Current runtime machine-readable artifacts under `docs/reference/` match implementation.
- [ ] Planning/process changes keep `docs/plan/STATUS_MODEL.md`, `docs/plan/CURRENT_STATE.md`, and `docs/plan/task-status.json` aligned.
- [ ] mdBook references use `book.toml` at repo root and `mdbook build`.
- [ ] No duplicate policy blocks copied across multiple docs without need.


---

*Last updated: 2026-03-18*
