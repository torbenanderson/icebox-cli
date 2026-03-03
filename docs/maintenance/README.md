# Maintenance

Recurring operational work: dependency updates, security audits, releases, and tooling hygiene.

## Scope

Maintenance covers:

- **Dependencies** — Dependabot PRs, merge order, policy. See [DEPENDENCIES.md](DEPENDENCIES.md).
- **Audits** — `cargo audit`, mutation testing, security reviews.
- **Releases** — Pre-release checklist, version bumps, changelog.
- **Docs & tooling** — Schema sync, mdBook, CI workflow updates.

## How to Use

| File | Purpose |
|------|---------|
| [CALENDAR.md](CALENDAR.md) | Recurring tasks and cadence. Add new tasks here as they are identified. |
| [LOG.md](LOG.md) | Lightweight log of completed work and decisions. Append when work is done. |
| [DEPENDENCIES.md](DEPENDENCIES.md) | Dependabot PR priority, merge order, and policy. |

**Add a task:** Edit `CALENDAR.md` — define cadence and owner if known.

**Record completion:** Append to `LOG.md` — date, task, and any notes for next time.

## Related

- [CI Process](../plan/CI.md) — Workflow triggers, schedules, blocking policy.
- [CONTRIBUTING.md](../../CONTRIBUTING.md) — Contribution workflow and PR expectations.
- [Versioning Policy](../reference/VERSIONING.md) — Release semantics.

---

*Last updated: 2026-03-03*
