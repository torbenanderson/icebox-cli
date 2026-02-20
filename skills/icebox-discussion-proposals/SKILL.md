---
name: icebox-discussion-proposals
description: Track external ideas as D-prefixed proposal issues. Use when asked to add/log a discussion link into GitHub issue tracking.
---

# Icebox Discussion Proposals

Use this skill to capture external/community ideas as first-class, linkable proposal issues.

## Use When

Trigger on requests like:

- `add discussion <link>`
- `log this discussion idea`
- `track this discussion proposal`

## Goal

Create or update a D-prefixed issue that preserves source context and keeps proposal status visible until it is either mapped to backlog work or closed.

## Canonical Pattern

1. Title: `[D#] Proposal: <topic> from discussion #<id>`
2. Labels: `discussion-proposal`, `proposal`
3. Template: `.github/ISSUE_TEMPLATE/discussion_proposal.yml`

## Lifecycle

Use this status flow:

1. `logged`
2. `under review`
3. `added to backlog` (fill backlog mapping like `E2-XX`)
4. `closed without adoption`

## Automation

Use `skills/icebox-discussion-proposals/scripts/discussion_proposal.sh`:

1. Get next D identifier:
   - `skills/icebox-discussion-proposals/scripts/discussion_proposal.sh next-id`
2. Draft proposal title/body from a discussion URL:
   - `skills/icebox-discussion-proposals/scripts/discussion_proposal.sh draft --link <url>`
3. Create GitHub issue directly:
   - `skills/icebox-discussion-proposals/scripts/discussion_proposal.sh create --link <url>`
4. Optional flags:
   - `--d-id D7`
   - `--summary "<text>"`
   - `--proposed-by "@handle"`
   - `--backlog E2-XX`
   - `--status "under review"`
   - `--title "<custom title>"`

## Output Requirements

Return:

1. `D#` used
2. Created issue URL (or draft content if creation was not requested/possible)
3. Source discussion URL
4. Current status and backlog mapping value
