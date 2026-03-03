# Dependency Maintenance

Policy for Dependabot PRs: merge order, risk assessment, and priority.

## Merge Order

Merge in this order to minimize risk and conflicts:

| Priority | PR type | Rationale |
|----------|---------|-----------|
| 1 | **Rust crates** (cargo minor/patch) | Runtime deps; patch bumps are low risk. Merge first. |
| 2 | **actions/checkout** | Used in every workflow. Merge early to avoid branch drift. |
| 3 | **actions/setup-python** | Isolated to `docs-schemas.yml`; low blast radius. |
| 4 | **actions/github-script** | Minor version; used in ci-enhancements, pr-body-guard. |
| 5 | **actions/upload-artifact** | Multi-major jumps (e.g. 4→7). Node 24, ESM. Merge last; verify Runner compatibility. |

## Impact Summary

| Dependency | Impact | Risk |
|------------|--------|------|
| Rust (clap, rustix, etc.) | High — affects shipped binary | Low — patch bumps |
| actions/checkout | High — all workflows | Low |
| actions/setup-python | Low — docs-schemas only | Low |
| actions/github-script | Low | Low |
| actions/upload-artifact | Medium — artifact upload for coverage/release | Medium — Node 24, Runner ≥ 2.327.1 |

## Policy

- Prefer merging low-risk PRs first.
- For `actions/upload-artifact` and other multi-major jumps: confirm Node/Runner compatibility before merge.
- Rebase Dependabot PRs if stale; avoid modifying them manually where possible.

## Related

- [CI Process](../plan/CI.md) — Workflow configuration.
- `.github/dependabot.yml` — Dependabot configuration.

---

*Last updated: 2026-03-03*
