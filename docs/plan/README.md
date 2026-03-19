# Planning Docs

This folder contains execution planning and delivery sequencing documents.

## Contents

- [CURRENT_STATE.md](CURRENT_STATE.md)
- [STATUS_MODEL.md](STATUS_MODEL.md)
- [task-status.json](task-status.json)
- [ROADMAP.md](ROADMAP.md)
- [BACKLOG.md](BACKLOG.md)
- [IMPLEMENTATION_BOOTSTRAP.md](IMPLEMENTATION_BOOTSTRAP.md)
- [BOOTSTRAP_ISSUES.md](BOOTSTRAP_ISSUES.md)
- [TESTING.md](TESTING.md)
- [CI.md](CI.md)

## Notes

- `CURRENT_STATE.md` is the human-readable status snapshot.
- `task-status.json` is the machine-readable packet lifecycle registry.
- `STATUS_MODEL.md` defines which planning files are authoritative for status vs scope vs sequencing.
- `TESTING.md` defines the intended test strategy and expected structure under `tests/`.
- E2 identity artifact contract notes:
  - `PKT-E2-03` storage contract for `enclave.keyref` / `identity.pub` / `key.enc`: [spec/archive/pkt-e2/PKT-E2-03-work-item.md](spec/archive/pkt-e2/PKT-E2-03-work-item.md)
  - E2-04 no-plaintext-on-disk hardening contract: [spec/archive/pkt-e2/PKT-E2-04-work-item.md](spec/archive/pkt-e2/PKT-E2-04-work-item.md)
- E2 fake-enclave test-harness behavior (`ICEBOX_TEST_FAKE_ENCLAVE`) is documented as non-production in [TESTING.md](TESTING.md).
- Release-slicing/deferred-item policy is canonical in `../architecture/mvp-decision-lock.md`.
- Docs hygiene/source-of-truth rules are in `../reference/DOCS_GOVERNANCE.md`.


---

*Last updated: 2026-03-18*
