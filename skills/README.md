# Skills Index

This directory contains repository-local skills used by Codex.

## Skill List

1. `icebox-docs-standards`
   - Path: `skills/icebox-docs-standards/SKILL.md`
   - Purpose: global documentation standards (mdBook/rustdoc surfaces, docs nav/index hygiene, footer policy, publish-ready docs).
   - Trigger: every task in this repository.
2. `icebox-load`
   - Path: `skills/icebox-load/SKILL.md`
   - Purpose: load backlog work into reviewable execution packets (issue + spec + tests + ADR triage + docs impact) before coding, using a strict issue state machine.
   - Trigger: load/prep/review-packet requests before implementation; use GitHub issue `#<id>` as canonical reference.
   - Automation: `skills/icebox-load/scripts/issue_packet.sh` + `.github/ISSUE_TEMPLATE/execution_packet.yml`
   - Can auto-create packet issue from backlog ID (`create --backlog <id>`).
3. `icebox-execute`
   - Path: `skills/icebox-execute/SKILL.md`
   - Purpose: kickoff alignment gates before coding (`roadmap -> backlog -> spec -> tests -> ADR -> docs`) with execute refusal until `ready-to-execute` + required checklist completion.
   - Trigger: "execute", build-component requests, execute-backlog requests, and kickoff alignment requests; prefer `execute #<id>`.
4. `icebox-ai-harness`
   - Path: `skills/icebox-ai-harness/SKILL.md`
   - Purpose: schema contract propagation, CI workflow guardrails, and commit-splitting by concern.
   - Trigger: schema/examples updates, architecture contracts for persisted artifacts, `.github/workflows/*` edits, or explicit commit-splitting requests.
5. `icebox-commit-merge-hygiene`
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
2. `icebox-load`
   - "load backlog item"
   - "prepare issue"
   - "preflight this feature"
   - "stage for review"
   - "ready this for execute"
3. `icebox-execute`
   - "execute"
   - "start building"
   - "build component"
   - "execute backlog"
   - "new command"
   - "roadmap backlog spec tests ADR"
4. `icebox-ai-harness`
   - "schema change"
   - "update JSON schema"
   - "workflow change"
   - "GitHub Actions"
   - "split commits by concern"
5. `icebox-commit-merge-hygiene`
   - commit flow: "commit", "commit message", "write commit", "message for commit"
   - PR flow: "pr", "pull request", "pr prompt", "pr body", "pr title"
   - merge flow: "merge", "merge message", "merge commit", "squash message"
   - default: if unclear, run commit flow

## Recommended Order

If multiple skills apply, use:

1. `icebox-docs-standards`
2. `icebox-load`
3. `icebox-execute`
4. `icebox-ai-harness`
5. `icebox-commit-merge-hygiene`

## Boundary Notes

- Do not duplicate docs standards checks in other skills; defer to `icebox-docs-standards`.
- Do not duplicate load/preflight packet generation in other skills; defer to `icebox-load`.
- Do not duplicate kickoff gating logic in `icebox-ai-harness`; defer to `icebox-execute`.
- Do not duplicate commit/merge message policy in other skills; defer to `icebox-commit-merge-hygiene`.

## Quick Flow

Use this minimal workflow:

1. Load from backlog ID (auto-creates packet issue if needed):
   - `load E1`
   - this also auto-builds load artifacts on the issue (backlog/spec/tests/ADR/docs impact)
   - and auto-adds epic/project context + matching milestone when available
2. Load from existing issue if already created:
   - `load #<issue-id>`
3. Fix packet until review-ready:
   - `fix load #<issue-id>`
4. Mark ready and execute:
   - `execute #<issue-id>`
5. Commit and push implementation:
   - `commit`
6. Close out packet:
   - add PR/tests/docs/ADR evidence in issue
   - transition to `done`

Backlog ID standard:

- Use `E*` IDs everywhere, defaulting to epic IDs (examples: `E1`, `E4`, `E7.5`).
- Optional extended forms are allowed when needed (example: `E1-02`).

State path:

`draft -> ready-for-review -> ready-to-execute -> in-progress -> done`

## Geek Note

If your team likes command-style kickoff words, define your own "engage command" aliases (yes, very Star Trek). Just map them to `icebox-execute` and keep one canonical skill as the source of truth.
