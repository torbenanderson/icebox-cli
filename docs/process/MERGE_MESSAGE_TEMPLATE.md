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

## Required Quality Bar

- Keep the title imperative and specific.
- Group changes by concern (code/test/docs/infra/contracts/process).
- Include concrete validation commands actually run.
- Call out deferred work explicitly; do not hide it in narrative text.

---

*Last updated: 2026-02-19*
