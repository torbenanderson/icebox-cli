## Packet Metadata

- Backlog ID(s): `E1 (epic), E1-13` (or `E?-??`)
- Issue reference(s): `#<id>, #<id>` (epic + packet refs allowed)
- Spec path(s): `docs/plan/spec/PKT-...-work-item.md` (comma-separated if multiple)
- Test ID(s): `T-...` (comma-separated)
- Docs touched: `<paths>` or `none`

## Objective

- <single outcome sentence>

## Context

- Why now: <reason>
- Security/trust impact: <none or explicit note>

## Change Set

- <group changes by concern: code/tests/docs/contracts/infra>

## Constraints

- <what must not change>

## Validation

- [ ] `cargo fmt --check`
- [ ] `cargo clippy -- -D warnings`
- [ ] `cargo test`
- [ ] Docs/schema checks (if applicable)

Commands + key results:

```text
<paste command summary with pass/fail>
```

## Review Focus

1. `<highest-risk file/path>`
2. `<next file/path>`

## Done When

- [ ] <acceptance bullet>
- [ ] <acceptance bullet>

## Closeout Evidence

- PR link:
- Tests run (commands + result):
- Docs updated (paths):
- Files added/changed (paths):
- ADR link (or `n/a`):

## Merge Message

Title:
- `<type>(<scope>): <outcome>`

Body:
- Summary:
- Changes:
- Validation:
- Follow-ups:

## Community Communication (optional for epic/milestone PRs)

- GitHub Release/pre-release note:
- README current-state update:
- Discussion/announcement:

## Release Note Snippet (optional)

- What this is:
- What this isn't:
- How to try it:
- Next:

## Checklists

- [ ] Docs only
- [ ] Code change
- [ ] Tests added/updated
- [ ] Security-sensitive behavior changed
- [ ] No plaintext secrets added to logs/output/files
- [ ] Fail-closed behavior preserved where expected
- [ ] `run` trust boundary unchanged or explicitly documented
- [ ] File mode / integrity implications reviewed
- [ ] Canonical docs updated for behavior changes (or `n/a`)
- [ ] Planning/testing docs updated when mappings changed (or `n/a`)
- [ ] Schema/fixtures updated for persisted artifact changes (or `n/a`)

## Linked Issues

Refs #<id>
