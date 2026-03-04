use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use crypto_box::SecretKey as X25519SecretKey;
use ed25519_dalek::SigningKey;
use serde_json::Value;
use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const FAKE_PREFIX: &[u8] = b"fake-enclave-wrap-v1:";

fn temp_path(prefix: &str) -> std::path::PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{}-{nanos}", std::process::id()))
}

fn decode_fake_wrapped_private_key(key_enc: &[u8]) -> [u8; 32] {
    let encoded = key_enc
        .strip_prefix(FAKE_PREFIX)
        .expect("fake wrap prefix should be present");
    assert_eq!(
        encoded.len(),
        32,
        "wrapped fake key payload should be 32 bytes"
    );
    let mut out = [0u8; 32];
    for (index, byte) in encoded.iter().enumerate() {
        out[index] = byte ^ 0xAA;
    }
    out
}

fn decrypt_sealed_blob_with_fake_identity(
    key_enc_path: &std::path::Path,
    sealed_blob_b64: &str,
) -> Result<Vec<u8>, crypto_box::aead::Error> {
    let key_enc = fs::read(key_enc_path).expect("key.enc should be readable");
    let ed25519_private = decode_fake_wrapped_private_key(&key_enc);
    let signing_key = SigningKey::from_bytes(&ed25519_private);
    let x25519_secret = X25519SecretKey::from(signing_key.to_scalar_bytes());

    let sealed = BASE64_STANDARD
        .decode(sealed_blob_b64)
        .expect("sealed blob should be valid base64");
    x25519_secret.unseal(&sealed)
}

#[test]
fn e3_02_add_seals_secret_with_crypto_box_compatible_blob() {
    let icebox_home = temp_path("icebox-e3-02");

    let register = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("register-agent")
        .arg("claw")
        .env("ICEBOX_HOME", &icebox_home)
        .env("ICEBOX_TEST_FAKE_ENCLAVE", "1")
        .output()
        .expect("failed to run register-agent");
    assert!(
        register.status.success(),
        "register-agent should succeed: {}",
        String::from_utf8_lossy(&register.stderr)
    );

    let plaintext = "sk-live-abc123";
    let add = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("add")
        .arg("openai")
        .arg(plaintext)
        .env("ICEBOX_HOME", &icebox_home)
        .output()
        .expect("failed to run add");
    assert!(
        add.status.success(),
        "add should succeed: {}",
        String::from_utf8_lossy(&add.stderr)
    );

    let vault_path = icebox_home
        .join("identities")
        .join("claw")
        .join("vault.enc");
    let vault_bytes = fs::read(&vault_path).expect("vault.enc should be readable");
    let vault: Value =
        serde_json::from_slice(&vault_bytes).expect("vault.enc should be valid json");
    let sealed_blob = vault["entries"][0]["sealedBlob"]
        .as_str()
        .expect("sealedBlob should be a string");
    let decrypted = decrypt_sealed_blob_with_fake_identity(
        &icebox_home.join("identities").join("claw").join("key.enc"),
        sealed_blob,
    )
    .expect("sealed blob should decrypt with matching identity key");
    assert_eq!(decrypted, plaintext.as_bytes());

    fs::remove_dir_all(&icebox_home).expect("temp ICEBOX_HOME cleanup should succeed");
}

#[test]
fn e3_02_tampered_blob_fails_decryption() {
    let icebox_home = temp_path("icebox-e3-02-tamper");

    let register = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("register-agent")
        .arg("claw")
        .env("ICEBOX_HOME", &icebox_home)
        .env("ICEBOX_TEST_FAKE_ENCLAVE", "1")
        .output()
        .expect("failed to run register-agent");
    assert!(
        register.status.success(),
        "register-agent should succeed: {}",
        String::from_utf8_lossy(&register.stderr)
    );

    let add = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("add")
        .arg("openai")
        .arg("sk-live-abc123")
        .env("ICEBOX_HOME", &icebox_home)
        .output()
        .expect("failed to run add");
    assert!(
        add.status.success(),
        "add should succeed: {}",
        String::from_utf8_lossy(&add.stderr)
    );

    let vault_path = icebox_home
        .join("identities")
        .join("claw")
        .join("vault.enc");
    let vault_bytes = fs::read(&vault_path).expect("vault.enc should be readable");
    let mut vault: Value =
        serde_json::from_slice(&vault_bytes).expect("vault.enc should be valid json");
    let sealed_blob = vault["entries"][0]["sealedBlob"]
        .as_str()
        .expect("sealedBlob should be a string");
    let mut sealed = BASE64_STANDARD
        .decode(sealed_blob)
        .expect("sealed blob should be valid base64");
    let index = sealed.len() - 1;
    sealed[index] ^= 0x01;
    vault["entries"][0]["sealedBlob"] = Value::String(BASE64_STANDARD.encode(sealed));
    let tampered_bytes =
        serde_json::to_vec_pretty(&vault).expect("tampered vault should serialize");
    fs::write(&vault_path, tampered_bytes).expect("tampered vault should be written to disk");

    let reloaded_bytes = fs::read(&vault_path).expect("tampered vault should be readable");
    let reloaded_vault: Value =
        serde_json::from_slice(&reloaded_bytes).expect("tampered vault should parse");

    let key_enc = icebox_home.join("identities").join("claw").join("key.enc");
    let tampered = reloaded_vault["entries"][0]["sealedBlob"]
        .as_str()
        .expect("tampered sealed blob should be a string");

    let result = decrypt_sealed_blob_with_fake_identity(&key_enc, tampered);
    assert!(result.is_err(), "tampered blob should fail decryption");

    fs::remove_dir_all(&icebox_home).expect("temp ICEBOX_HOME cleanup should succeed");
}
