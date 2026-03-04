# Security Policy

## Supported Scope

Icebox is currently alpha and docs-first. Security posture is evolving during MVP implementation.

Primary security-sensitive scope includes:

- key generation/wrapping/unwrapping flows,
- vault encryption/integrity/rollback protections,
- `icebox run` secret injection and environment sanitization,
- file permissions and local-filesystem safety checks.

For architecture details, trust boundaries, and diagrams, see [Security Model](docs/architecture/security-model.md).

## Reporting a Vulnerability

Please report vulnerabilities privately.

Preferred channel:

1. Use GitHub Security Advisories for this repository ("Report a vulnerability" / private advisory).

If private advisory tooling is unavailable, contact project maintainers privately and include:

- affected version/commit,
- impact summary,
- reproduction steps or proof of concept,
- suggested mitigation (if known).

Do not disclose exploit details in public issues before coordinated disclosure.

## Disclosure Process

Project intent:

1. Acknowledge receipt within 5 business days.
2. Triage severity and impact.
3. Prepare a fix and tests.
4. Coordinate disclosure timing with reporter.
5. Publish advisory and remediation notes once patch is available.

Timing can vary for complex issues, but reporters will receive status updates.

## Safe Harbor

Good-faith security research is welcome. Please avoid:

- privacy violations,
- destructive testing on third-party systems,
- social engineering, spam, or denial-of-service.

Testing should be limited to systems and data you own or are authorized to test.
