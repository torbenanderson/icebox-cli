---
name: icebox-first-deliverable-alignment
description: Align roadmap, backlog, spec, tests, and ADR expectations before coding a first deliverable or new CLI command. Use for kickoff requests like "start building", "new command", or "first vertical slice".
---

# Icebox First Deliverable Alignment

Use this skill before writing implementation code for a new CLI command or first vertical slice.

## Goal

Create a small, explicit delivery contract across planning artifacts so implementation can start without churn.

## Inputs

- Proposed deliverable name (for example: `icebox init`)
- Thin-slice outcome statement
- Any known constraints (platform, security, UX, timeline)

## Alignment Workflow (Run In Order)

1. Confirm scope sentence
   - One sentence in "verb + artifact + user value" form.
   - Example: "Add `icebox init` to create `.icebox/config.json` for first-run setup."
2. Roadmap check (`docs/plan/ROADMAP.md`)
   - Ensure the deliverable maps to a roadmap milestone/epic.
   - If missing, add the smallest milestone entry with outcome only.
3. Backlog check (`docs/plan/BACKLOG.md`)
   - Add or update one story with:
   - user story statement
   - acceptance criteria (observable behavior)
   - identifiers linking to roadmap item
4. Spec check (short spec)
   - Create/update a minimal feature spec using `references/mini-spec-template.md`.
   - Keep it to one page and behavior-focused (no deep design prose).
5. Test-plan check (`docs/plan/TESTING.md`)
   - Add acceptance tests mapped to the backlog item.
   - Define at least one happy-path and one failure-path test.
6. ADR triage (`docs/architecture/decisions/`)
   - Use ADR only when the decision is long-lived and cross-feature.
   - If needed, add an ADR stub from `docs/architecture/decisions/ADR-TEMPLATE.md`.
7. Docs sync check (`docs/README.md`, `docs/SUMMARY.md`, command docs)
   - For any doc creation/update, load and apply `skills/icebox-docs-standards/SKILL.md`.
   - Ensure docs navigation/index entries are updated when pages are added/removed.
   - Ensure touched Markdown follows the repo footer policy from docs standards.

## Gate Policy

Hard gates before coding:

- Backlog story exists with acceptance criteria.
- Test cases for those criteria exist in `docs/plan/TESTING.md`.
- Docs sync completed for the deliverable, applying `skills/icebox-docs-standards/SKILL.md` (including navigation and footer policy where applicable).

Soft warnings (do not block first implementation pass):

- Roadmap entry missing.
- Short spec missing.
- ADR not written for a decision that may be long-lived.

## Required Output Format

After running this skill, report:

1. Alignment status table with `present`, `missing`, or `out-of-sync` for:
   - roadmap
   - backlog
   - spec
   - tests
   - ADR
   - docs
2. Decision log bullets:
   - decisions made
   - deferred decisions
   - ADR-needed decisions
3. Next action:
   - either "ready to implement"
   - or a short closure list of missing hard gates

## ADR Trigger Rules

Write/update an ADR when any of these are true:

- Introduces or changes a persisted file format/schema contract.
- Sets command behavior that future commands must follow (global UX/exit-code contract).
- Changes security or trust-boundary behavior.
- Commits to compatibility/versioning policy.

If none are true, keep decisions in the short spec.
