# Planning Status Model

This document defines how planning, execution, and closeout state are tracked in this repository.

## Source Of Truth

- Scope, requirements, and acceptance intent live in [BACKLOG.md](BACKLOG.md) and packet specs under `docs/plan/spec/`.
- Machine-readable packet lifecycle state lives in [task-status.json](task-status.json).
- Human-readable project snapshot lives in [CURRENT_STATE.md](CURRENT_STATE.md).
- `done` is owned by `skills/icebox-load/scripts/issue_packet.sh done`.

## Packet Lifecycle

Packet status values:

- `planned`
- `in_progress`
- `implemented`
- `done`

Meaning:

- `planned`: packet/spec exists, but closeout evidence is not complete.
- `in_progress`: packet is actively being executed.
- `implemented`: code/tests/docs evidence has been posted, but the packet is not yet fully closed.
- `done`: closeout evidence passed, packet is archived, and the status registry has been updated.

## Naming Conventions

Use these terms consistently:

- Epic:
  - Format: `E1`, `E2`, `E3`, `E7.5`
  - Purpose: roadmap/backlog grouping identifier
- Backlog item:
  - Format: `<epic>-<item>`, for example `E3-12`, `E5-01`, `E7.5-04`
  - Purpose: canonical requirement identifier in [BACKLOG.md](BACKLOG.md)
- Packet:
  - Format: `PKT-<backlog-id>`
  - File convention: `PKT-<backlog-id>-work-item.md`
  - Purpose: execution/closeout unit for one backlog contract
- Slice:
  - Format: descriptive label, not an ID namespace
  - Examples:
    - `Vault Core Slice`
    - `MVP Runnable Slice`
  - Purpose: cross-item progress view spanning multiple backlog items/packets

Rules:

- Do not invent separate numeric IDs for slices.
- Do not use packet IDs as backlog IDs.
- Do not use epic IDs alone when a backlog item exists and needs precise tracking.
- When naming an archived packet directory, use `pkt-<epic-lowercase>` (for example `pkt-e3`).

## Done Gate Responsibilities

The done flow is the only authority allowed to mark a packet `done`.

On successful closeout, it must:

1. Validate closeout evidence.
2. Explicitly account for architecture docs impact:
   - `docs/architecture/**` paths, or
   - `none (not impacted)`
3. Update [task-status.json](task-status.json).
4. Move the packet spec into `docs/plan/spec/archive/pkt-<epic>/`.
5. Regenerate [CURRENT_STATE.md](CURRENT_STATE.md).

## File Roles

Keep these boundaries strict:

| File | Role | Keep / Delete / Absorb |
|---|---|---|
| [BACKLOG.md](BACKLOG.md) | Canonical requirements and backlog scope | Keep |
| [CURRENT_STATE.md](CURRENT_STATE.md) | Human-readable current status snapshot | Keep |
| [task-status.json](task-status.json) | Canonical machine-readable packet lifecycle registry | Keep |
| [IMPLEMENTATION_BOOTSTRAP.md](IMPLEMENTATION_BOOTSTRAP.md) | Execution sequencing and hardening-order guidance only | Keep, narrowed |
| `START_CODING_CHECKLIST.md` | Duplicated execution gating/checklist material | Absorb into skills/status model and delete |
| [CI.md](CI.md) | CI policy and workflow intent only | Keep |

## Practical Rules

- Do not use [BACKLOG.md](BACKLOG.md) as a live status tracker.
- Do not use packet-template checkboxes as the primary completion source of truth.
- Do not infer `done` from code presence alone once the done gate exists; record it through the done flow.
- Use the root README for a short status snapshot only. Link to [CURRENT_STATE.md](CURRENT_STATE.md) for detail.

---
*Last updated: 2026-03-18*
