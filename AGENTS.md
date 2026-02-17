## Skills
A skill is a set of local instructions stored in a `SKILL.md` file.

### Available skills
- icebox-docs-standards: Enforce mdBook + rustdoc documentation standards, docs structure hygiene, and publish-ready Markdown practices. (file: `skills/icebox-docs-standards/SKILL.md`)
- icebox-ai-harness: Enforce schema-contract propagation, CI workflow guardrails, and commit-splitting by concern for infra/contracts/process changes. (file: `skills/icebox-ai-harness/SKILL.md`)
- icebox-first-deliverable-alignment: Align roadmap, backlog, spec, tests, and ADR expectations before starting a first deliverable or new CLI command. (file: `skills/icebox-first-deliverable-alignment/SKILL.md`)
- icebox-commit-merge-hygiene: Standardize commit and merge message quality using repository commit template conventions and concern-based commit splits. (file: `skills/icebox-commit-merge-hygiene/SKILL.md`)

## How to use skills
- Discovery: Use the list above as the source of truth for skills available in this repository.
- Trigger rules: Always load and apply `icebox-docs-standards` for every task in this repository. Load `icebox-first-deliverable-alignment` when the user asks to start building a first slice, kick off a new command/feature, or requests cross-artifact alignment checks (roadmap/backlog/spec/tests/ADR). Load `icebox-ai-harness` if the user names it (with `$icebox-ai-harness` or plain text), or if the task modifies schema contracts (`docs/reference/schemas/*` or examples), architecture contracts tied to persisted artifacts, workflow files under `.github/workflows/`, or requires commit splitting by concern. Load `icebox-commit-merge-hygiene` when the user asks for commit/PR/merge messaging; map intent by trigger words (`commit*` -> commit flow, `pr*` -> PR flow, `merge*`/`squash*` -> merge flow), and default to commit flow when ambiguous.
- Skill order: If all apply, use `icebox-docs-standards` -> `icebox-first-deliverable-alignment` -> `icebox-ai-harness` -> `icebox-commit-merge-hygiene`. Otherwise, preserve that relative order for whichever subset applies.
- Missing/blocked: If the skill path cannot be read, state that briefly and continue with best-effort fallback.
- Progressive disclosure:
  1) Open required skill files in this order when applicable:
     - `skills/icebox-docs-standards/SKILL.md`
     - `skills/icebox-first-deliverable-alignment/SKILL.md`
     - `skills/icebox-ai-harness/SKILL.md`
     - `skills/icebox-commit-merge-hygiene/SKILL.md`
  2) Read only what is needed to execute the task.
  3) If additional references are later added, load only the specific files required.
