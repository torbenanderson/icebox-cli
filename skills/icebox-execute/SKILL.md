---
name: icebox-execute
description: Execute implementation with alignment gates across roadmap, backlog, spec, tests, ADR, and docs before coding. Use for kickoff requests like "execute", "start building", "build", or "execute backlog".
---

# Icebox Execute

Use this skill before writing implementation code for a new command, feature, or scoped backlog slice.

## Goal

Create a small, explicit delivery contract across planning artifacts so implementation can start without churn.

## Inputs

- Canonical reference: `#<issue-number>` (preferred)
- Fallback reference before issue exists: `PKT-<backlog-id>-<slug>`
- Proposed deliverable name (for example: `icebox init`)
- Thin-slice outcome statement
- Any known constraints (platform, security, UX, timeline)

## Reference Rule

Execute against a single referenced packet:

1. Preferred: `execute #<issue-number>`
2. Fallback: `execute PKT-...` only when issue is not created yet
3. If both are provided, issue ID is source of truth
4. If no reference is provided, request one before starting implementation

## Issue State Machine

Execution must follow:

1. `draft`
2. `ready-for-review`
3. `ready-to-execute`
4. `in-progress`
5. `done`

Use forward-only transitions. `execute` begins at `ready-to-execute` and moves the item to `in-progress`.

## Execute Refusal Rule

`execute #<id>` must refuse to run unless both are true:

1. Issue has `ready-to-execute` label.
2. Required checklist boxes are complete:
   - backlog mapped
   - spec linked
   - tests mapped
   - ADR triaged
   - docs impact listed

Automation check command:

- `skills/icebox-load/scripts/issue_packet.sh validate-execute --issue <id>`

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
8. Execution plan comment (before coding)
   - Add one issue comment titled `Execution Plan`.
   - Include commit split plan (required).
9. Closeout evidence check (before done)
   - Require evidence links/notes for:
     - PR link
     - tests run
     - docs updated
     - ADR link (if required)
   - Validate with:
     - `skills/icebox-load/scripts/issue_packet.sh validate-closeout --issue <id>`
10. State transitions
   - Start execution:
     - `skills/icebox-load/scripts/issue_packet.sh transition --issue <id> --to in-progress`
   - Complete execution:
     - `skills/icebox-load/scripts/issue_packet.sh transition --issue <id> --to done`

## Gate Policy

Hard gates before coding:

- Backlog story exists with acceptance criteria.
- Test cases for those criteria exist in `docs/plan/TESTING.md`.
- Docs sync completed for the deliverable, applying `skills/icebox-docs-standards/SKILL.md` (including navigation and footer policy where applicable).
- Referenced issue/packet is marked ready for execution (for GitHub issues: `ready-to-execute`).
- Required checklist boxes are complete.
- `Execution Plan` comment exists with commit split plan.

Soft warnings (do not block first implementation pass):

- Roadmap entry missing.
- Short spec missing.
- ADR not written for a decision that may be long-lived.

## Required Output Format

After running this skill, report:

1. Alignment status table with `present`, `missing`, or `out-of-sync` for:
   - reference
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
4. State transition line:
   - `Transition: ready-to-execute -> in-progress` (on start)
   - `Transition: in-progress -> done` only after closeout evidence is complete

## ADR Trigger Rules

Write/update an ADR when any of these are true:

- Introduces or changes a persisted file format/schema contract.
- Sets command behavior that future commands must follow (global UX/exit-code contract).
- Changes security or trust-boundary behavior.
- Commits to compatibility/versioning policy.

If none are true, keep decisions in the short spec.
