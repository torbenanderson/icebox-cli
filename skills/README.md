# Skills Index

This directory contains repository-local skills used by Codex.

## Skill List

1. `icebox-docs-standards`
   - Path: `skills/icebox-docs-standards/SKILL.md`
   - Purpose: global documentation standards (mdBook/rustdoc surfaces, docs nav/index hygiene, footer policy, publish-ready docs).
   - Trigger: every task in this repository.
2. `icebox-first-deliverable-alignment`
   - Path: `skills/icebox-first-deliverable-alignment/SKILL.md`
   - Purpose: kickoff alignment gates before coding (`roadmap -> backlog -> spec -> tests -> ADR -> docs`).
   - Trigger: first slice/new command/kickoff alignment requests.
3. `icebox-ai-harness`
   - Path: `skills/icebox-ai-harness/SKILL.md`
   - Purpose: schema contract propagation, CI workflow guardrails, and commit-splitting by concern.
   - Trigger: schema/examples updates, architecture contracts for persisted artifacts, `.github/workflows/*` edits, or explicit commit-splitting requests.
4. `icebox-commit-merge-hygiene`
   - Path: `skills/icebox-commit-merge-hygiene/SKILL.md`
   - Purpose: consistent commit message format, merge/PR message structure, concern-based commit split planning, and default push-to-remote after commits.
   - Trigger: commit/PR/merge message requests with intent routing by trigger words; defaults to commit flow.

## Trigger Words

Use these phrases as quick routing hints.

1. `icebox-docs-standards`
   - "update docs"
   - "docs nav"
   - "SUMMARY.md"
   - "docs footer"
   - "publish docs"
2. `icebox-first-deliverable-alignment`
   - "start building"
   - "first deliverable"
   - "new command"
   - "vertical slice"
   - "roadmap backlog spec tests ADR"
3. `icebox-ai-harness`
   - "schema change"
   - "update JSON schema"
   - "workflow change"
   - "GitHub Actions"
   - "split commits by concern"
4. `icebox-commit-merge-hygiene`
   - commit flow: "commit", "commit message", "write commit", "message for commit"
   - PR flow: "pr", "pull request", "pr prompt", "pr body", "pr title"
   - merge flow: "merge", "merge message", "merge commit", "squash message"
   - default: if unclear, run commit flow

## Recommended Order

If multiple skills apply, use:

1. `icebox-docs-standards`
2. `icebox-first-deliverable-alignment`
3. `icebox-ai-harness`
4. `icebox-commit-merge-hygiene`

## Boundary Notes

- Do not duplicate docs standards checks in other skills; defer to `icebox-docs-standards`.
- Do not duplicate kickoff gating logic in `icebox-ai-harness`; defer to `icebox-first-deliverable-alignment`.
- Do not duplicate commit/merge message policy in other skills; defer to `icebox-commit-merge-hygiene`.
