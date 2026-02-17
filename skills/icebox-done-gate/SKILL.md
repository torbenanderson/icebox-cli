---
name: icebox-done-gate
description: Close out execution packets with hard evidence checks (PR/tests/docs/files/ADR) before transitioning issues to done.
---

# Icebox Done Gate

Use this skill to close an in-progress execution packet and move it to `done`.

## Goal

Apply a strict closeout gate so completed work has auditable evidence and clean handoff.

## Use When

Use on requests like:

- "done"
- "closeout"
- "mark done"
- "ship this"
- "finish packet"

## Inputs

- Canonical reference: `#<issue-number>`
- Optional summary of final changes

## Workflow

1. Confirm issue is in `in-progress`.
2. Verify closeout evidence fields are present and non-empty:
   - `PR link:`
   - `Tests run (commands + result):`
   - `Docs updated (paths):`
   - `Files added/changed (paths):`
   - `ADR link:` (required when ADR required is yes)
3. Run hard validation:
   - `skills/icebox-load/scripts/issue_packet.sh validate-closeout --issue <id>`
4. If validation passes, transition:
   - `skills/icebox-load/scripts/issue_packet.sh transition --issue <id> --to done`
5. If validation fails, return a short blocker list and do not transition.

## Hard Gate

Never transition to `done` when closeout evidence is incomplete.

## Output Requirements

Return:

1. Closeout status: `passed` or `blocked`
2. Any missing evidence fields (if blocked)
3. Transition line on success:
   - `Transition: in-progress -> done`
