# Maintenance Calendar

Recurring tasks and cadence. Add entries as work is identified; update cadence when process changes.

## Weekly

| Task | When | Notes |
|------|------|-------|
| Security audit | Saturday 01:00 UTC | `cargo audit` via `security-audit.yml`. Triage advisories; patch or backlog. |
| Mutation testing | Saturday 01:00 UTC | `cargo mutants` via `mutation-testing.yml` (Linux). Review survivors; improve tests. |

## Per-PR

| Task | When | Notes |
|------|------|-------|
| Dependency bumps | When merging Dependabot PRs | Follow [DEPENDENCIES.md](DEPENDENCIES.md) merge order. Low-risk first. |

## Pre-Release

| Task | When | Notes |
|------|------|-------|
| _TBD_ | Before tagging `v*` | Changelog, version bump, release checklist. To be defined at first release. |

## Monthly / Quarterly

| Task | When | Notes |
|------|------|-------|
| _TBD_ | As identified | Larger maintenance tasks (docs sync, tooling upgrades, deprecation cycles). |

---

*Last updated: 2026-03-03*
