# Planning Docs

This folder contains execution planning and delivery sequencing documents.

## Contents

- [ROADMAP.md](ROADMAP.md)
- [BACKLOG.md](BACKLOG.md)
- [IMPLEMENTATION_BOOTSTRAP.md](IMPLEMENTATION_BOOTSTRAP.md)
- [START_CODING_CHECKLIST.md](START_CODING_CHECKLIST.md)
- [BOOTSTRAP_ISSUES.md](BOOTSTRAP_ISSUES.md)
- [TESTING.md](TESTING.md)
- [CI.md](CI.md)

## Notes

- `TESTING.md` defines the intended test strategy and expected structure under `tests/`.
- E2 identity artifact contract notes:
  - `PKT-E2-03` storage contract for `enclave.keyref` / `identity.pub` / `key.enc`: [spec/PKT-E2-03-work-item.md](spec/PKT-E2-03-work-item.md)
  - E2-04 no-plaintext-on-disk hardening contract: [spec/PKT-E2-04-work-item.md](spec/PKT-E2-04-work-item.md)
- E2 fake-enclave test-harness behavior (`ICEBOX_TEST_FAKE_ENCLAVE`) is documented as non-production in [TESTING.md](TESTING.md).
- The `tests/` folder and tests may not exist yet during planning; this is expected until implementation starts.
- Release-slicing/deferred-item policy is canonical in `../architecture/mvp-decision-lock.md`.
- Docs hygiene/source-of-truth rules are in `../reference/DOCS_GOVERNANCE.md`.


---

*Last updated: 2026-03-03*
