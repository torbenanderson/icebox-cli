---
name: icebox-commit-merge-hygiene
description: Standardize commit and merge message quality. Use when drafting commit messages, planning commit splits, writing PR titles, or structuring merge commit messages.
---

# Icebox Commit And Merge Hygiene

Use this skill when preparing commits or merges in this repository.

## Trigger Routing

Route by intent using trigger words:

1. `commit` flow (default):
   - trigger words: `commit`, `commit message`, `message for commit`, `write commit`
2. `pr` flow:
   - trigger words: `pr`, `pull request`, `pr prompt`, `pr body`, `pr title`, `pr quality report`, `coverage report`
3. `merge` flow:
   - trigger words: `merge`, `merge message`, `merge commit`, `squash message`

If intent is ambiguous, default to `commit` flow and produce one best template without options.

## Scope

1. Commit message structure and quality.
2. PR as prompt (modernized OpenClaw-style review flow).
3. Merge/PR title and merge-commit message structure.
4. Commit splitting guidance for reviewability.

## Alignment Sources

Follow these repository rules when generating commit/PR outputs:

1. `.gitmessage` for commit structure.
2. `CONTRIBUTING.md` PR guidelines (what/why/affected docs/tests).
3. `.github/OPENCLAW_COMMUNITY_ONRAMP.md` PR expectations (backlog IDs, concrete validation).
4. `docs/reference/DOCS_GOVERNANCE.md` PR hygiene for docs changes.
5. `skills/icebox-commit-merge-hygiene/references/merge-message-template.md` merge/squash title + body structure.

## Commit Message Standard

Base format uses `.gitmessage`:

- Subject: `<type>(<scope>): <imperative summary>`
- Body sections:
  - `Why`
  - `What`
  - `Scope`
  - `Validation`

Apply these rules:

1. Keep subject imperative and specific.
2. Keep scope concrete (module/path/area).
3. Document validation run and anything not run with reason.
4. Avoid vague subjects like "updates" or "fix stuff".

## Merge/PR Message Standard

Use this structure for PR titles and merge commit bodies:

1. Title:
   - `<type>(<scope>): <outcome>`
2. Body:
   - `Summary`: user-visible and technical outcome
   - `Changes`: grouped by concern (infra/contracts/process/code)
   - `Validation`: checks run
   - `Follow-ups`: deferred work

If multiple concerns exist, prefer split commits before merge.

## PR As Prompt Standard

Treat the PR body as an executable prompt for reviewers/agents.

Required PR prompt sections:

1. `Packet Metadata`
   - backlog IDs (epic + packets), issue references, spec paths, test IDs, docs touched
2. `Objective`
   - one outcome sentence
3. `Context`
   - why now, backlog IDs, trust/security notes when relevant
4. `Change Set`
   - concrete files/modules changed
5. `Constraints`
   - what must not change
6. `Validation`
   - checks run and expected outcomes
7. `Review Focus`
   - where reviewers should look first
8. `Done When`
   - explicit acceptance bullets
9. `Closeout Evidence`
   - PR link, tests run, docs updated, changed files, ADR link or `n/a`
10. `Merge Message`
   - one merge-ready summary for squash/merge commit reuse

Optional for epic/milestone PRs:

1. `Community Communication`
   - release/discussion/README announcement plan
2. `Release Note Snippet`
   - short copy-ready note with "what this is / what this isn't / how to try / what's next"

Use imperative, testable statements. Avoid narrative prose.

## PR Quality Report Standard

Use this when the user asks for a PR quality/coverage status update or report comment.

Required sections (in order):

1. `Definitions`
   - Distinguish:
     - `Test coverage`: required tests/scenarios executed and pass status.
     - `Code coverage`: measured executed code percentages.
2. `RAG legend`
   - `GREEN`: meets expected target for PR scope.
   - `AMBER`: acceptable with known gaps.
   - `RED`: below threshold or missing evidence.
3. `Status summary`
   - `Test coverage status: <RAG>`
   - `Code coverage status: <RAG>`
4. `Test Coverage (execution)`
   - workflow name, pass/fail, run URL.
5. `Requirement/Scenario Coverage (scope)`
   - packet/scope statement, key happy/failure path confirmation.
6. `Code Coverage (measurement)`
   - workflow name, pass/fail, run URL, artifact URL.
   - include totals: regions/functions/lines.
7. `Touched-module snapshot`
   - include key modules touched in PR with concise metrics.
8. `Follow-up focus`
   - concrete next improvements (1-3 bullets).

Formatting rules:

1. Keep report concise and scannable.
2. Avoid mixing test execution and coverage metrics in one bullet.
3. Prefer links to workflow runs/artifacts as evidence.
4. If a metric is unavailable, state `missing` and set RAG to `RED` for that section.

## Commit Splitting Rules

Split by concern when practical:

1. `infra`: CI/workflow/tooling
2. `contracts`: schemas, architecture contracts
3. `process/docs`: planning docs, governance, guidance
4. `code`: runtime behavior changes

Keep each commit independently understandable and testable.

Expected behavior:

1. Use multiple commits when changes span distinct topics.
2. Group each commit around one coherent concern/outcome.
3. Avoid mixing unrelated concerns in one commit unless explicitly requested.
4. If splitting is appropriate, generate a commit prompt and commit message for each planned commit.
5. Default split for test work:
   - `test`: runnable test artifacts under `src/**` or `tests/**`
   - `docs`: test-plan/spec mapping updates under `docs/plan/**`
   - combine only when the user explicitly asks for a single commit.

## Output Requirements

When this skill is used, provide:

1. One best output for the detected flow (no option sets unless explicitly requested).
2. For `commit` flow:
   - one commit message using `.gitmessage` sections
   - one commit prompt using the commit prompt format
3. For `pr` flow:
   - one PR title
   - one PR prompt body using the PR prompt format
4. For `merge` flow:
   - one merge title/message
   - one PR/merge prompt body if context is available

## Post-Commit Remote Push Policy

Default behavior after commit creation:

1. Always push commits to remote for the current branch.
2. If multiple commits are created, push after the commit set is complete.
3. Only skip push when explicitly instructed.
4. If push fails (no remote/auth/rejected), report the failure clearly and include the exact push command to retry.

## Commit Prompt Format

Generate one prompt per commit using this exact shape:

```text
Commit Prompt: <short label>
Objective:
- <single-sentence outcome>

Scope:
- <files/modules in this commit only>

Constraints:
- <non-goals / boundaries>

Implementation Tasks:
1. <task>
2. <task>

Validation:
- <commands/checks>
- <expected result>

Commit Message:
- Subject: <type>(<scope>): <imperative summary>
- Why:
  - <bullet>
- What:
  - <bullet>
- Scope:
  - <bullet>
- Validation:
  - <bullet>
```

## PR Prompt Format

When producing a PR body, use this exact shape:

```text
PR Prompt
Packet Metadata:
- Backlog ID(s): <epic + packet IDs, or N/A>
- Issue reference(s): <#id list or N/A>
- Spec path(s): <path list or N/A>
- Test ID(s): <T-* list or N/A>
- Docs touched: <paths or none>

Objective:
- <outcome>

Context:
- Backlog: <IDs or N/A>
- Why now: <reason>
- Security/trust impact: <none or explicit note>

Change Set:
- <grouped by concern: infra/contracts/process/code>

Constraints:
- <must-not-change items>

Validation:
- <checks run>
- <key results>

Review Focus:
1. <highest-risk file/path>
2. <next file/path>

Done When:
- <acceptance bullet>
- <acceptance bullet>

Closeout Evidence:
- PR link: <url or pending>
- Tests run (commands + result): <summary>
- Docs updated (paths): <paths or none>
- Files added/changed (paths): <paths>
- ADR link (or n/a): <value>

Merge Message:
- <title + concise merge body>

Community Communication (optional for epic/milestone PRs):
- GitHub Release (pre-release): <note or n/a>
- README current-state update: <note or n/a>
- Discussion/announcement: <note or n/a>

Release Note Snippet (optional):
- What this is: <text>
- What this isn't: <text>
- How to try it: <commands>
- Next: <follow-up epic flow>
```
