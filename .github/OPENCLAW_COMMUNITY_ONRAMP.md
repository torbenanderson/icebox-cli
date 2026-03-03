# OpenClaw Community On-Ramp

This page is the fastest path for new contributors to start useful work. For full workflow, PR guidelines, and security expectations, see [CONTRIBUTING.md](../CONTRIBUTING.md).

## Start Here

1. Read `README.md` (trust boundary + MVP scope).
2. Read [CONTRIBUTING.md](../CONTRIBUTING.md) and [SECURITY.md](../SECURITY.md).
3. Pick one issue from the queue below.

## Working Rules (MVP)

- Keep changes small and scoped to one backlog item.
- Prefer docs/tests/contracts first when implementation is not ready.
- For details on PR workflow, security-sensitive changes, and vulnerability reporting, see [CONTRIBUTING.md](../CONTRIBUTING.md).

## First 5 Community Issues

### 1) `docs: freeze run env var contract`

- Backlog mapping: `E5-03`, `E5-04`
- Goal:
  - Define the exact secret env var name for `icebox run`.
  - Define override behavior (if any) and collision behavior.
  - Update architecture + testing docs for deterministic behavior.
- Files likely touched:
  - `docs/architecture/secret-management-and-run.md`
  - `docs/plan/BACKLOG.md`
  - `docs/plan/TESTING.md`
- Done when:
  - Contract is explicit and testable.
  - At least one test row references final behavior.

### 2) `docs/tests: durable rollback detection acceptance criteria`

- Backlog mapping: `E3-13`, `E3-16`, `E3-17`
- Goal:
  - Specify persisted monotonic integrity anchor acceptance criteria for cross-restart rollback detection.
  - Ensure architecture/planning/testing docs align on this expectation.
- Files likely touched:
  - `docs/architecture/vault-and-integrity.md`
  - `docs/architecture/security-model.md`
  - `docs/plan/IMPLEMENTATION_BOOTSTRAP.md`
  - `docs/plan/TESTING.md`
- Done when:
  - Cross-restart rollback requirement is unambiguous.
  - Blocker tests clearly reflect the requirement.

### 3) `docs: Linux backend evaluation rubric`

- Backlog mapping: Phase 2 planning note
- Goal:
  - Add a strict comparison rubric for Linux backend candidates:
    - TPM
    - OS keyring
    - software fallback
    - YubiKey/PIV/PKCS#11 path
  - Include security, UX, CI viability, and operational complexity criteria.
- Files likely touched:
  - `docs/architecture/platform-and-distribution.md`
  - `docs/plan/ROADMAP.md`
- Done when:
  - Maintainers can evaluate candidates consistently without ad hoc debates.

### 4) `reference: expand error code registry guidance`

- Backlog mapping: `E1-19`, `E3-13`
- Goal:
  - Add short contributor guidance for when to create new `ICE` codes vs reuse existing.
  - Add naming/category conventions for new vault/runtime codes.
- Files likely touched:
  - `docs/reference/error-codes.json`
  - `docs/architecture/errors-and-diagnostics.md`
  - `docs/reference/DOCS_GOVERNANCE.md`
- Done when:
  - Future contributors can extend codes without overloading semantics.

### 5) `planning: newcomer-friendly issue labels and triage`

- Backlog mapping: process support
- Goal:
  - Define label conventions (`good first issue`, `docs`, `security`, `needs-design`, `blocked`).
  - Add triage checklist for maintainers.
- Files likely touched:
  - `.github/ISSUE_TEMPLATE/config.yml`
  - `CONTRIBUTING.md`
- Done when:
  - New contributors can self-select work with minimal maintainer back-and-forth.

## Suggested Labels

- `good first issue`
- `docs`
- `security`
- `planning`
- `help wanted`

For PR expectations (backlog links, file references, concrete descriptions), see [CONTRIBUTING.md](../CONTRIBUTING.md#pr-guidelines).
