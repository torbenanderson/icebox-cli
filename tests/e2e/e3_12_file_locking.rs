use serde_json::Value;
use std::fs;
use std::process::Command;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

fn temp_path(prefix: &str) -> std::path::PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{}-{nanos}", std::process::id()))
}

#[test]
fn e3_12_concurrent_add_is_serialized_by_vault_lock() {
    let icebox_home = temp_path("icebox-e3-12-serialized");

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

    // Hold lock in first add so second add must wait on advisory flock.
    let mut first = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("add")
        .arg("openai")
        .arg("sk-first")
        .env("ICEBOX_HOME", &icebox_home)
        .env("ICEBOX_TEST_HOLD_VAULT_LOCK_MS", "700")
        .spawn()
        .expect("failed to spawn first add");

    std::thread::sleep(std::time::Duration::from_millis(120));

    let started = Instant::now();
    let second = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("add")
        .arg("openai")
        .arg("sk-second")
        .env("ICEBOX_HOME", &icebox_home)
        .output()
        .expect("failed to run second add");
    let elapsed = started.elapsed();

    let first_status = first.wait().expect("failed to wait for first add");
    assert!(first_status.success(), "first add should succeed");
    assert!(
        second.status.success(),
        "second add should succeed: {}",
        String::from_utf8_lossy(&second.stderr)
    );
    assert!(
        elapsed >= std::time::Duration::from_millis(500),
        "second add should wait for lock; elapsed={elapsed:?}"
    );

    let vault_path = icebox_home
        .join("identities")
        .join("claw")
        .join("vault.enc");
    let vault_bytes = fs::read(&vault_path).expect("vault.enc should be readable");
    let vault: Value =
        serde_json::from_slice(&vault_bytes).expect("vault.enc should be valid json");
    let entries = vault["entries"]
        .as_array()
        .expect("entries should be an array");
    assert_eq!(
        entries.len(),
        1,
        "single service should remain single entry"
    );
    assert_eq!(entries[0]["service"].as_str(), Some("openai"));
    assert!(
        !icebox_home
            .join("identities")
            .join("claw")
            .join("vault.enc.tmp")
            .exists(),
        "vault.enc.tmp should not linger after writes"
    );

    fs::remove_dir_all(&icebox_home).expect("temp ICEBOX_HOME cleanup should succeed");
}

#[test]
fn e3_12_add_fails_when_vault_lock_path_is_unopenable() {
    let icebox_home = temp_path("icebox-e3-12-lock-open-fail");

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

    let lock_path = icebox_home
        .join("identities")
        .join("claw")
        .join("vault.enc.lock");
    fs::create_dir_all(&lock_path).expect("lock-path directory blocker should be creatable");

    let add = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("add")
        .arg("openai")
        .arg("sk-test")
        .env("ICEBOX_HOME", &icebox_home)
        .output()
        .expect("failed to run add");

    assert_eq!(add.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&add.stderr);
    assert!(stderr.contains("ICE-201"));
    assert!(stderr.contains("failed to open vault.enc.lock"));

    fs::remove_dir_all(&icebox_home).expect("temp ICEBOX_HOME cleanup should succeed");
}
