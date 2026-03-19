# Merge Message Template

Use this guide for merge commits/squash messages so merge history stays auditable.

## Title Format

`<type>(<scope>): <outcome>`

Examples:

- `feat(error): land E1-13 structured CLI error mapping`
- `docs(plan): sync E1-13 packet metadata and testing links`

## Body Template

```text
Summary:
- <user-visible and technical outcome>

Changes:
- code: <runtime behavior changes>
- test: <runnable tests added/updated>
- docs: <planning/spec/reference updates>
- infra/contracts/process: <workflow/schema/process updates, if any>

Validation:
- <command and pass/fail result>
- <command and pass/fail result>

Follow-ups:
- <deferred work item or n/a>
```

## Epic/Milestone Variant

Use this variant when merging an epic-level PR that spans multiple packet issues.

```text
<Epic ID> - <Epic name> (<slice/phase label>)

<1-2 sentence executive summary focused on user value and delivery stage.>

Scope:
- <concern group 1>
- <concern group 2>
- <concern group 3>

Goal:
- <why this merge matters before next epics>

Refs: <backlog IDs and/or issue references>
```

## Community Communication Add-On (Optional)

For milestone/epic merges, include a short copy-ready announcement block:

```text
What this is:
- <foundation delivered now>

What this isn't:
- <not yet implemented capabilities>

How to try:
- <build/test/run commands>

Next:
- <upcoming epic sequence>
```

## Required Quality Bar

- Keep the title imperative and specific.
- Group changes by concern (code/test/docs/infra/contracts/process).
- Include concrete validation commands actually run.
- Call out deferred work explicitly; do not hide it in narrative text.
- When merge scope changes packet lifecycle/process behavior, reference:
  - `docs/plan/STATUS_MODEL.md`
  - `docs/plan/CURRENT_STATE.md`
  - `docs/plan/task-status.json`
