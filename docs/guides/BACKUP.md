# Icebox Backup & Recovery

> How to protect your agent keys and recover from key loss.

---

> **Phase 1.5:** Seed-based backup (`--seed` flag, BIP39 mnemonic, SLIP-0010 derivation, `recover-agent`, `export`/`import`) is **not available in the MVP**. It ships in Phase 1.5 alongside DID support. The sections below describe the full guide for when the feature ships. **MVP users:** see [MVP Reality](#mvp-reality-default-mode-only) for what applies today.

## MVP Reality First (Important)

Icebox stores rotatable credentials (API keys/tokens), not irreplaceable crypto assets.

If you lose your Mac in MVP:

1. Register a new agent.
2. Regenerate API keys/tokens from each provider dashboard.
3. Re-add secrets.

This is primarily a convenience loss, not permanent asset loss. For most setups this is minutes of work.

## Overview

Icebox stores each agent's Ed25519 private key **encrypted by a hardware-bound P-256 key inside the Secure Enclave**. The encrypted blob lives at `~/.icebox/identities/<name>/key.enc`, and secrets are sealed in an isolated vault at `~/.icebox/identities/<name>/vault.enc`.

In MVP, if the device's Secure Enclave is lost (machine failure, migration, hardware replacement), that agent's vault is not decryptable on new hardware. The operational recovery is to rotate/regenerate provider credentials and re-add them to a new agent. Phase 1.5 adds optional seed-backed key recovery.

This guide covers both modes and how to recover.

## MVP Reality (Default Mode Only)

**In the MVP, Icebox ships with enclave-only keys.** There is no `--seed` flag, no recovery phrase, and no `recover-agent` or `export`/`import` commands.

| What happens | MVP |
|---|---|
| **Register agent** | `icebox register-agent claw` creates an enclave-wrapped Ed25519 key. No mnemonic, no recovery path. |
| **Lose your device** | That agent's vault is **permanently locked**. The key was wrapped by a Secure Enclave key that only exists on that device's hardware. You must re-register the agent and re-add all secrets. |
| **Migrate to a new Mac** | Same as above -- no recovery. Copy `vault.enc` if you want to preserve the sealed blobs, but without the original device's enclave you cannot decrypt them. Re-register and re-add secrets. |
| **Backup strategy** | Time Machine (or similar) backs up `key.enc` and `hmac.enc`, but those blobs are **device-bound** -- they can only be unwrapped by the same Mac's Secure Enclave. Restoring to a different machine does not help. |

**Phase 1.5** adds optional seed backup flow for portability/cross-device restore. Until then, treat agent keys as ephemeral per-device and keep a service inventory (`icebox list --services`) so provider key rotation is straightforward after device loss.

---

## Key Modes

### Default (Maximum Security)

```bash
icebox register-agent claw
```

- Ed25519 private key is generated in software, then immediately encrypted by a **Secure Enclave P-256 wrapping key** and stored as `key.enc`
- The plaintext key never touches disk -- it only ever exists in protected memory
- **No recovery path** -- if the device's Secure Enclave is lost (new machine, hardware failure), the vault is permanently locked
- Best for: ephemeral agents, test environments, or users with existing machine-level backup strategies (Time Machine covers the encrypted blob but requires the same device's enclave)

### Seed Backup (Phase 1.5 — Recommended for Production)

```bash
icebox register-agent claw --seed
```

- Icebox generates a 24-word BIP39 mnemonic and derives the Ed25519 keypair via **SLIP-0010** (hardened path `m/7737'/0'/0'` -- Icebox-specific purpose root, isolated from other coin-type namespaces)
- The mnemonic is displayed **once** -- as a QR code in the terminal and as plain text
- The mnemonic is **never stored** by Icebox (wiped from memory immediately after display)
- The same mnemonic will always reproduce the same keypair (deterministic derivation)

## Backing Up Your Seed (Phase 1.5)

When you see the seed phrase (Phase 1.5), you have one chance to save it. Recommended approaches:

### Option 1: Scan the QR Code (Recommended)

1. Open your iPhone camera (or any QR scanner)
2. Scan the QR code displayed in the terminal
3. Save it in a secure location (Apple Notes with locked note, or a password manager)

### Option 2: Write It Down

1. Write the 24 words on paper, in order
2. Store in a physically secure location (safe, safety deposit box)
3. Do **not** photograph it or store it in an unencrypted file

### Option 3: Password Manager

1. Copy the 24 words into your password manager
2. Do **not** paste it anywhere else afterward
3. Clear your clipboard immediately

## What NOT to Do

- Do **not** store the seed in a plaintext file on your machine
- Do **not** email or message the seed to yourself
- Do **not** commit the seed to any git repository
- Do **not** paste it into any AI agent, chat, or web form
- Do **not** screenshot the QR code and leave it in your photo library

## Recovery (Phase 1.5)

If you lose access to your device's Secure Enclave (new machine, hardware failure, migration):

```bash
icebox recover-agent claw --seed "word1 word2 word3 ... word24"
```

This will:
1. Re-derive the exact same Ed25519 keypair from the seed via SLIP-0010
2. Create a new P-256 wrapping key in the Secure Enclave on the current device
3. Encrypt the Ed25519 private key with the new enclave key and store as `key.enc`
4. Your existing `vault.enc` will work immediately -- all sealed secrets are decryptable again

### Recovery Checklist

- [ ] Copy `~/.icebox/identities/<name>/` directory to the new machine (at minimum: `vault.enc`, `identity.pub`, `manifest.json`). Note: `key.enc` and `hmac.enc` from the old machine are **not needed** -- a new enclave wrapping key and HMAC key will be created
- [ ] Ensure `~/.icebox/config.json` exists (create if needed; `recover-agent` will update it)
- [ ] Run `icebox recover-agent <name> --seed "word1 word2 ... word24"` (creates new enclave key + wrapped blob)
- [ ] Verify with `icebox list --agent <name>` that your secrets are visible
- [ ] Test with `icebox run <service> "echo ok" --agent <name>` to confirm decryption works
- [ ] If this is your only agent, verify it's set as active in `config.json`

**What `recover-agent` updates in `manifest.json`:**

The `manifest.json` copied from the old machine contains stale device-specific fields. `recover-agent` **automatically** overwrites these -- you do not need to edit `manifest.json` by hand:

| Field | Old Machine Value | After `recover-agent` |
|---|---|---|
| `enclaveKeyRef` | Reference to the old device's Secure Enclave P-256 key (stale -- that key only exists on the old hardware) | **Replaced** with the new device's enclave key reference |
| `enclaveAlgorithm` | Algorithm constant (should be unchanged) | **Preserved** (verified to match the current runtime constant; mismatch → error) |
| (file: `hmac.enc`) | HMAC key encrypted by the old enclave key (stale -- cannot be unwrapped on new hardware) | **Regenerated** -- a new 256-bit HMAC key is generated, encrypted by the new enclave key, and stored as `hmac.enc`. The first vault write after recovery establishes a new HMAC baseline over the existing vault contents. |
| `did` | `did:key:z6Mk...` | **Preserved** (same Ed25519 keypair re-derived from seed → same DID) |
| `pubkeyFingerprint` | SHA-256 of multicodec-prefixed pubkey | **Verified** (must match the re-derived keypair; mismatch → seed is wrong → abort) |
| `derivationVersion` | SLIP-0010 version (e.g., `1`) | **Preserved** (same derivation path used for recovery) |
| `name`, `type`, `parent`, `created` | Original values | **Preserved** (identity metadata is portable) |

> **Important:** If you edit `manifest.json` manually before running `recover-agent`, do not change `derivationVersion` or `pubkeyFingerprint` -- these are used to verify that the seed produces the correct keypair. Tampering with them will cause recovery to fail with a clear error.

## Multiple Machines (Phase 1.5)

The seed can be used to set up the same agent on multiple machines:

1. Run `icebox recover-agent claw --seed "..."` on each machine (each creates its own enclave wrapping key)
2. Copy the agent's `vault.enc` to `~/.icebox/identities/claw/vault.enc` on each machine (or maintain separate vaults per machine)
3. All machines will have the same Ed25519 keypair (derived from seed) and can decrypt the same sealed secrets
4. Each machine's `key.enc` will be different (different enclave wrapping key) but the underlying Ed25519 key is identical

> **Note:** Icebox v1 does not sync vaults between machines. Each agent's vault is local to that machine. Multi-machine vault sync is planned for a future phase.

## FAQ

**Q: What if I didn't use `--seed` and lost my device?**
A: That agent's vault is permanently locked on new hardware. The practical recovery flow for MVP credentials is to re-register the agent, regenerate provider API keys/tokens, and re-add them. Other agents on the same machine are equally affected (each has its own enclave wrapping key). **MVP:** There is no `--seed` option yet; all MVP agents use this model.

**Q: Can I add a seed to an existing agent that was created without one?**
A: No. The seed is derived at keypair generation time. To get a seed-backed agent, register a new agent with `--seed`, then re-add your secrets to the new agent.

**Q: Is the seed the same as the private key?**
A: No. The seed is a BIP39 mnemonic that deterministically derives the private key via SLIP-0010. The private key itself is always stored encrypted by the Secure Enclave -- it only exists in plaintext momentarily in protected memory during use.

**Q: Can someone with my seed access my secrets?**
A: Yes. The seed can reproduce your private key, which can decrypt your vault. **Treat the seed with the same care as a master password.**


---

*Last updated: 2026-02-16*
