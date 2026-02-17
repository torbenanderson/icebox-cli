## Skills
A skill is a set of local instructions stored in a `SKILL.md` file.

### Available skills
- icebox-docs-standards: Enforce mdBook + rustdoc documentation standards, docs structure hygiene, and publish-ready Markdown practices. (file: `skills/icebox-docs-standards/SKILL.md`)
- icebox-load: Prepare backlog items for implementation by loading them into reviewable issue/spec/test packets before coding. (file: `skills/icebox-load/SKILL.md`)
- icebox-ai-harness: Enforce schema-contract propagation, CI workflow guardrails, and commit-splitting by concern for infra/contracts/process changes. (file: `skills/icebox-ai-harness/SKILL.md`)
- icebox-execute: Execute implementation with alignment gates across roadmap, backlog, spec, tests, ADR, and docs before coding. (file: `skills/icebox-execute/SKILL.md`)
- icebox-commit-merge-hygiene: Standardize commit and merge message quality using repository commit template conventions and concern-based commit splits. (file: `skills/icebox-commit-merge-hygiene/SKILL.md`)

## How to use skills
- Discovery: Use the list above as the source of truth for skills available in this repository.
- Trigger rules: Always load and apply `icebox-docs-standards` for every task in this repository. Load `icebox-load` when the user asks to prepare/load backlog items into issues/specs before implementation, review an execution packet, or stage work for approval. Load `icebox-execute` when the user says "execute" or asks to start building, build a component, execute backlog work, kick off a new command/feature, or requests cross-artifact alignment checks (roadmap/backlog/spec/tests/ADR). Load `icebox-ai-harness` if the user names it (with `$icebox-ai-harness` or plain text), or if the task modifies schema contracts (`docs/reference/schemas/*` or examples), architecture contracts tied to persisted artifacts, workflow files under `.github/workflows/`, or requires commit splitting by concern. Load `icebox-commit-merge-hygiene` when the user asks for commit/PR/merge messaging; map intent by trigger words (`commit*` -> commit flow, `pr*` -> PR flow, `merge*`/`squash*` -> merge flow), and default to commit flow when ambiguous.
- Skill order: If all apply, use `icebox-docs-standards` -> `icebox-load` -> `icebox-execute` -> `icebox-ai-harness` -> `icebox-commit-merge-hygiene`. Otherwise, preserve that relative order for whichever subset applies.
- Missing/blocked: If the skill path cannot be read, state that briefly and continue with best-effort fallback.
- Progressive disclosure:
  1) Open required skill files in this order when applicable:
     - `skills/icebox-docs-standards/SKILL.md`
     - `skills/icebox-load/SKILL.md`
     - `skills/icebox-execute/SKILL.md`
     - `skills/icebox-ai-harness/SKILL.md`
     - `skills/icebox-commit-merge-hygiene/SKILL.md`
  2) Read only what is needed to execute the task.
  3) If additional references are later added, load only the specific files required.
