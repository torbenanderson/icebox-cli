# Discussion Proposals

Use D-prefixed proposal issues to track ideas that originate in discussions or external channels.

## Purpose

1. Keep externally proposed ideas visible in the same workflow as backlog work.
2. Preserve source context (discussion link + proposer) for later review.
3. Enable clean closure when ideas are not adopted.

## Title And Identity

1. Use title format: `[D#] Proposal: <topic> from discussion #<id>`.
2. `D#` is maintained in issue titles for proposal tracking.
3. GitHub issue number (`#<n>`) remains the canonical issue identifier.

## Required Fields

Use issue form: `.github/ISSUE_TEMPLATE/discussion_proposal.yml`

Required content:

1. Discussion link
2. Summary
3. Status
4. Backlog mapping (`E*-*` or `n/a`)
5. Contributor attribution (`Proposed By` and profile link when available)

Default labels:

1. `discussion-proposal`
2. `proposal`

## Status Lifecycle

Use forward progress through:

1. `logged`
2. `under review`
3. `added to backlog` (include backlog ID mapping such as `E2-XX`)
4. `closed without adoption`

## Mapping To Delivery Work

When adopted:

1. Add backlog mapping in the proposal issue.
2. Link the proposal issue from backlog/spec/packet work as needed.
3. Add milestone/PR/commit links back to the proposal issue for traceability.
4. Include source discussion link and contributor attribution in ADR/decision docs when that proposal drives architecture or policy changes.

## Aging And Closure

If a proposal remains inactive for an extended period, close it with `closed without adoption` and retain links/context for future reopening.

---

*Last updated: 2026-02-20*
