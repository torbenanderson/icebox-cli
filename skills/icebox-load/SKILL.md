---
name: icebox-load
description: Load backlog work into a review-ready execution packet (issue + mini spec + acceptance tests + ADR triage + docs impact) before coding begins.
---

# Icebox Load

Use this skill to prepare work for review before implementation.

## Goal

Turn a backlog item into a clear, reviewable packet so execution can happen with low churn.

## Use When

Use on requests like:

- "load this backlog item"
- "prepare issue/spec before coding"
- "stage for review"
- "ready this for execute"

## Inputs

- Backlog item ID(s) or feature name
- Proposed scope/outcome
- Known constraints (security/platform/timeline)

## Canonical Identifier

Use GitHub issue ID as the canonical execution reference.

1. Preferred ID: `#<issue-number>` (for example `#128`)
2. Optional local fallback before issue creation: `PKT-<backlog-id>-<slug>`
3. After issue exists, treat the packet ID as secondary metadata and the issue ID as primary.

## Issue State Machine

Use this strict lifecycle:

1. `draft`
2. `ready-for-review`
3. `ready-to-execute`
4. `in-progress`
5. `done`

Allowed transitions are forward-only in this order.

## Automation Commands

Use `skills/icebox-load/scripts/issue_packet.sh`:

1. Ensure required state labels exist:
   - `skills/icebox-load/scripts/issue_packet.sh ensure-labels`
2. Transition state safely:
   - `skills/icebox-load/scripts/issue_packet.sh transition --issue <id> --to ready-for-review`
3. Use issue template:
   - `.github/ISSUE_TEMPLATE/execution_packet.yml`

## Workflow

1. Identify canonical backlog source
   - Locate item in `docs/plan/BACKLOG.md`.
2. Build mini spec
   - Create/update spec using `references/execution-packet-template.md`.
3. Map acceptance tests
   - Add/verify linked tests in `docs/plan/TESTING.md`.
4. ADR triage
   - Mark whether ADR is required (`yes`/`no`) with rationale.
5. Docs impact map
   - List docs that must change (`docs/README.md`, `docs/SUMMARY.md`, command docs, architecture or reference docs as needed).
6. Issue packet output
   - Produce a ready-to-file issue body with:
     - objective
     - scope
     - acceptance criteria
     - test plan
     - ADR flag
     - docs checklist
     - out-of-scope list
7. Bind references
   - Ensure the issue body includes:
     - `Issue ID`: `#<n>` (once created)
     - `Packet ID`: `PKT-...` (optional but recommended)
     - spec path (if created)
     - backlog ID mapping
8. Set state
   - New/updated packet state must be `draft` or `ready-for-review`.
   - Promote to `ready-to-execute` only after review approval and checklist completion.

## Labels And Handoff

Default labels:

- `draft`
- `ready-for-review`
- `ready-to-execute`
- `in-progress`
- `done`

This skill stops at preparation and does not start implementation.

## Output Requirements

Return:

1. Execution packet status:
   - backlog: `ready|needs-update|missing`
   - spec: `ready|needs-update|missing`
   - tests: `ready|needs-update|missing`
   - ADR triage: `ready|needs-update|missing`
   - docs impact: `ready|needs-update|missing`
2. Draft issue body (copy-ready)
3. Canonical reference line:
   - `Reference: #<issue-number>` (preferred)
   - `Fallback: PKT-...` (only before issue exists)
4. A clear handoff line:
   - `ready for review`
   - or missing items to close before review
5. Definition-of-loaded checklist status:
   - backlog mapped
   - spec linked
   - tests mapped
   - ADR triaged
   - docs impact listed
