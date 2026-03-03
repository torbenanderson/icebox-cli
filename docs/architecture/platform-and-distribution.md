# Platform & Distribution

## Platform Constraints

- macOS is the primary target for Secure Enclave integration.
- Non-macOS builds use enclave stubs for non-enclave test paths.
- Logical model is platform-stable, backend scheme is platform-specific.

## Lane Model (Cross-Platform)

- `local-enclave` lane:
  - local backend executes wrap/unwrap/sign operations.
  - MVP implementation target starts on macOS.
- `paired-remote-signer` lane (post-MVP):
  - paired device/service performs key operations and returns results.
  - desktop/CLI path does not receive raw long-lived private key bytes in this mode.

## Linux Strategy (Planning Status)

- Linux is explicitly in scope for post-MVP exploration, but not committed for Phase 1/1.5.
- Earliest planning window: Phase 2 discovery after public `v0.1.1` hardening gates are stable.
- No GA date is committed yet for Linux full-flow support.

### Candidate Linux Key-Protection Tracks

1. TPM-backed wrapping path (TPM 2.0 + local sealed key workflow).
2. OS keyring path (for example Secret Service/libsecret-backed envelope key storage).
3. Software-only fallback (passphrase-protected local key material) for CI/dev workflows with reduced security guarantees.
4. External hardware token path (for example YubiKey via PIV/PKCS#11), subject to UX/security evaluation.
5. Mobile-paired signing/unwrap proxy path for approval-first workflows.

### Acceptance Bar For Linux Full-Flow Support

- Equivalent fail-closed behavior on key unwrap and vault integrity checks.
- Deterministic file-permission and local-filesystem safety checks.
- CI coverage for end-to-end `register-agent -> add -> run -> remove` on Linux runners.
- Clear security-mode labeling so users can distinguish hardware-backed vs software-backed protection.

## Contributor Expectations (Current)

- Linux contributors can implement/test CLI, config, vault, crypto, schemas, and non-enclave security checks today.
- Secure Enclave-specific behavior remains macOS-only until a Linux key-protection backend is selected and merged.
- Documentation and issue templates should continue to mark this as a known limitation to reduce repeated "when Linux?" churn.

## Signing & Entitlements

- Production distribution requires signed/notarized binaries.
- Entitlements and Keychain/ACL behavior are part of the security boundary.

## Release Channels

- Direct signed binary first.
- Homebrew packaging after first stable release.

## Related Docs

- `mvp-decision-lock.md`


---

*Last updated: 2026-02-24*
