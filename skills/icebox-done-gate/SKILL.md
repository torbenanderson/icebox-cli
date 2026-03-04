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
2. Ensure commit hygiene before closeout evidence:
   - Split commits by concern when practical (do not mix unrelated changes in one commit).
   - Preferred split for packet closeout:
     - runtime/code/refactor behavior
     - docs/spec/reference alignment
   - Each commit message must include clear `Why`, `What`, `Scope`, and `Validation`.
   - Run tests/validation before each commit (or clearly state why a check was not run).
3. Auto-build closeout evidence and post it to the issue:
   - `skills/icebox-load/scripts/issue_packet.sh closeout --issue <id>`
   - PR behavior:
     - if branch is not `main`/`master`, push branch and reuse existing PR for branch or auto-create an epic-level draft PR.
     - if branch is `main`/`master`, auto-create/switch to epic branch (`pkt/<epic-id>`, for example `pkt/e1` for backlog `E1-01`) before push/PR.
   - Runs default validation commands:
     - `cargo check`
     - `cargo fmt --check`
     - `cargo clippy -- -D warnings`
     - `cargo test`
   - Captures:
     - `PR link:` (auto-created/reused PR URL)
     - `Tests run (commands + result):`
     - `Docs updated (paths):`
     - `Files added/changed (paths):`
     - `ADR link:` (default `n/a`, must be real if ADR required is yes)
   - Packet-scope override:
     - `--file-path <path>` (repeatable)
     - `--doc-path <path>` (repeatable)
   - On `pkt/*` branches, packet-scope override is required to avoid branch-wide evidence bleed.
4. Run docs closeout checklist (required before done transition):
   - Internal docs updated when impacted:
     - `docs/plan/**` (backlog/spec/testing/plan index)
     - `docs/architecture/**` (when architecture/contract/security behavior changes)
   - External/public docs updated when impacted:
     - `README.md`
     - `docs/README.md`
     - user-facing docs pages under `docs/`
   - mdBook surface sync checks:
     - `docs/SUMMARY.md` reflects added/removed/moved pages
     - `docs/book.toml` remains aligned with docs structure and build metadata
   - Evidence comment must include:
     - `Internal docs updated:` paths or `none (not impacted)`
     - `External docs updated:` paths or `none (not impacted)`
5. Validate docs build/readiness when docs changed:
   - `mdbook build docs`
   - `cargo doc --workspace --all-features --no-deps`
6. Run hard validation:
   - `skills/icebox-load/scripts/issue_packet.sh validate-closeout --issue <id>`
7. If validation passes, transition:
   - `skills/icebox-load/scripts/issue_packet.sh transition --issue <id> --to done`
8. Preferred one-shot command:
   - `skills/icebox-load/scripts/issue_packet.sh done --issue <id> [--file-path <path>]... [--doc-path <path>]...`
9. If validation fails, return a short blocker list and do not transition.

## Hard Gate

Never transition to `done` when closeout evidence is incomplete.
Never transition to `done` when internal/external docs impact is not explicitly accounted for.
Never transition to `done` if docs changed and mdBook/rustdoc validation was not run.

## Output Requirements

Return:

1. Closeout status: `passed` or `blocked`
2. Any missing evidence fields (if blocked)
3. Docs checklist status:
   - internal docs: `updated` or `none (not impacted)`
   - external docs: `updated` or `none (not impacted)`
   - mdBook sync: `pass`/`fail`/`n/a`
4. Transition line on success:
   - `Transition: in-progress -> done`
