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
  Execution sequencing, milestones, backlog, and test planning.
- `docs/guides/`:
  User/operator guides.
- `docs/reference/`:
  Machine-readable artifacts and policy references.
- `docs/reference/schemas/`:
  JSON Schema contracts and fixtures for persisted artifacts.

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
5. Packet metadata fields: backlog ID, issue #, spec path, test IDs, docs touched.
6. Closeout evidence fields: PR link, tests run, docs updated, files changed, ADR link (`n/a` allowed).

## Review Checklist

- [ ] Links resolve after file moves.
- [ ] Decision-impacting changes update `docs/architecture/mvp-decision-lock.md` when needed.
- [ ] Decision-impacting changes add/update ADRs under `docs/architecture/decisions/` when applicable.
- [ ] Persisted-artifact changes update `docs/reference/schemas/` + fixtures.
- [ ] No duplicate policy blocks copied across multiple docs without need.


---

*Last updated: 2026-02-19*
