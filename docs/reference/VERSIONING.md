# Versioning Reference

This file is the versioning policy and release-planning template for Icebox.

## Standard

Icebox uses **Semantic Versioning (SemVer)** in `Cargo.toml`:

- `MAJOR.MINOR.PATCH`
- Example: `0.1.0`

## Rust Conventions

- `Cargo.toml` uses plain SemVer (no `v` prefix): `0.1.0`
- Git tags may use `v` prefix: `v0.1.0`
- Crate edition is `2024` with explicit minimum supported Rust version in `Cargo.toml`:
  - `rust-version = "1.85"`
  - Toolchains older than `1.85` are unsupported.

## Pre-1.0 Rules (`0.x.y`)

- Use `PATCH` (`0.1.1`) for fixes/hardening with minimal user-facing behavior change.
- Use `MINOR` (`0.2.0`) for meaningful user-facing behavior changes (new commands, flags, workflows).

## Pre-Release Tags

When you need unstable milestones before a final cut, use SemVer pre-release tags:

- `0.2.0-alpha.1`
- `0.2.0-beta.1`
- `0.2.0-rc.1`

## Planning-Doc Labeling

Avoid locking a numeric next version too early in planning docs.

Use labels first:

- `MVP Core`
- `Post-MVP Hardening`

Assign the concrete SemVer number at release-cut time, based on final scope.

## Current Decision

Planning docs should use:

- `MVP Core`
- `Post-MVP Hardening`

At release-cut time:

- choose `0.1.1` if the hardening drop is mostly fixes/security/quality,
- choose `0.2.0` if it includes meaningful user-visible capabilities.

## Release Decision Template

Use this template when finalizing a release number:

```md
Release Candidate: <name/date>

Scope Summary:
- <change 1>
- <change 2>

User-Facing Changes:
- [ ] New command(s)
- [ ] New flag(s)
- [ ] Behavior change requiring adaptation

Risk Profile:
- [ ] Mostly fixes/hardening
- [ ] Contains meaningful UX/API changes

SemVer Decision:
- Selected version: <x.y.z>
- Why this version:
  - <reason 1>
  - <reason 2>
```

## Related Docs

- `docs/plan/ROADMAP.md`
- `docs/plan/BACKLOG.md`
- `docs/plan/IMPLEMENTATION_BOOTSTRAP.md`


---

*Last updated: 2026-03-03*
