use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_path(prefix: &str) -> std::path::PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{}-{nanos}", std::process::id()))
}

#[test]
fn e3_01_first_add_creates_vault_with_versioned_entries() {
    let icebox_home = temp_path("icebox-e3-01");

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
        .arg("sk-test-123")
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
    assert!(
        vault_path.exists(),
        "vault.enc should be created on first add"
    );
    let vault_bytes = fs::read(&vault_path).expect("vault.enc should be readable");
    let vault_json: serde_json::Value =
        serde_json::from_slice(&vault_bytes).expect("vault.enc should be valid json");
    assert_eq!(
        vault_json.get("format").and_then(|v| v.as_str()),
        Some("icebox.vault.legacy-v1")
    );
    assert_eq!(vault_json.get("version").and_then(|v| v.as_u64()), Some(1));
    let entries = vault_json
        .get("entries")
        .and_then(|v| v.as_array())
        .expect("entries should be an array");
    assert_eq!(entries.len(), 1);
    assert_eq!(
        entries[0].get("service").and_then(|v| v.as_str()),
        Some("openai")
    );
    let sealed_blob = entries[0]
        .get("sealedBlob")
        .and_then(|v| v.as_str())
        .expect("sealedBlob should exist");
    assert!(!sealed_blob.is_empty());
    assert!(
        !sealed_blob.contains("sk-test-123"),
        "sealed blob should not expose plaintext"
    );

    fs::remove_dir_all(&icebox_home).expect("temp ICEBOX_HOME cleanup should succeed");
}

#[test]
fn e3_01_add_fails_when_identity_pub_is_missing() {
    let icebox_home = temp_path("icebox-e3-01-missing-identity");

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

    let identity_pub = icebox_home
        .join("identities")
        .join("claw")
        .join("identity.pub");
    fs::remove_file(&identity_pub).expect("identity.pub should be removable for failure path");

    let add = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("add")
        .arg("openai")
        .arg("sk-test-123")
        .env("ICEBOX_HOME", &icebox_home)
        .output()
        .expect("failed to run add");
    assert_eq!(add.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&add.stderr);
    assert!(stderr.contains("ICE-201"));
    assert!(stderr.contains("missing identity public key"));

    if icebox_home.exists() {
        fs::remove_dir_all(&icebox_home).expect("temp ICEBOX_HOME cleanup should succeed");
    }
}

#[test]
fn e3_01_add_fails_when_no_active_agent_is_configured() {
    let icebox_home = temp_path("icebox-e3-01-no-active-agent");
    fs::create_dir_all(&icebox_home).expect("ICEBOX_HOME should be creatable");

    let add = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("add")
        .arg("openai")
        .arg("sk-test-123")
        .env("ICEBOX_HOME", &icebox_home)
        .output()
        .expect("failed to run add");

    assert_eq!(add.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&add.stderr);
    assert!(stderr.contains("ICE-201"));
    assert!(stderr.contains("No active agent configured"));

    fs::remove_dir_all(&icebox_home).expect("temp ICEBOX_HOME cleanup should succeed");
}
