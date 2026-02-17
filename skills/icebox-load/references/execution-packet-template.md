# Execution Packet Template

## Title

- `<backlog-id>: <short outcome>`

## References

- Issue ID (canonical): `#<issue-number>`
- Packet ID (pre-issue fallback): `PKT-<backlog-id>-<slug>`
- Spec path:

## State

- Current state: `draft|ready-for-review|ready-to-execute|in-progress|done`

## Objective

- Single-sentence outcome:

## Backlog Mapping

- Backlog item(s):
- Roadmap/phase link:

## Scope

- In scope:
- Out of scope:

## Acceptance Criteria

- AC1:
- AC2:
- AC3:

## Test Plan

- Test IDs in `docs/plan/TESTING.md`:
- Happy path:
- Failure path:

## ADR Triage

- ADR required? (`yes`/`no`)
- Rationale:
- ADR path (if yes): `docs/architecture/decisions/<adr-file>.md`

## Docs Impact

- [ ] `docs/README.md`
- [ ] `docs/SUMMARY.md`
- [ ] Command docs
- [ ] Architecture docs
- [ ] Reference docs/schemas

## Definition Of Loaded (Required For `ready-to-execute`)

- [ ] Backlog item is current and unambiguous
- [ ] Backlog mapped
- [ ] Spec linked
- [ ] Tests mapped
- [ ] ADR triaged
- [ ] Docs impact listed

## Execution Plan (Required Before Coding)

- Commit split plan:
- Planned validation commands:
- Risk notes:

## Closeout Evidence (Required For `done`)

- PR link:
- Tests run (commands + result):
- Docs updated (paths):
- ADR link (if required):

## Labels / State

- `draft`
- `ready-for-review`
- `ready-to-execute` (only after Definition Of Loaded is complete)
- `in-progress` (set at execution start)
- `done` (only after Closeout Evidence is complete)
