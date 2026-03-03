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
fn e2_02_register_agent_creates_enclave_wrapping_key_reference() {
    let icebox_home = temp_path("icebox-e2-02");
    let output = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("register-agent")
        .arg("claw")
        .env("ICEBOX_HOME", &icebox_home)
        .env("ICEBOX_TEST_FAKE_ENCLAVE", "1")
        .output()
        .expect("failed to run icebox-cli register-agent");

    assert!(
        output.status.success(),
        "expected register-agent success; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    let key_ref_path = icebox_home
        .join("identities")
        .join("claw")
        .join("enclave.keyref");
    assert!(key_ref_path.exists(), "enclave.keyref should be created");
    let key_ref =
        fs::read_to_string(&key_ref_path).expect("enclave.keyref should be readable as utf-8");
    assert!(
        !key_ref.trim().is_empty(),
        "enclave key reference should be non-empty"
    );

    fs::remove_dir_all(&icebox_home).expect("temp ICEBOX_HOME cleanup should succeed");
}

#[test]
fn e2_02_register_agent_reports_deterministic_error_when_wrapping_key_creation_fails() {
    let icebox_home = temp_path("icebox-e2-02-failure");
    let output = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("register-agent")
        .arg("claw")
        .env("ICEBOX_HOME", &icebox_home)
        .env("ICEBOX_TEST_FORCE_ENCLAVE_ERROR", "1")
        .output()
        .expect("failed to run icebox-cli register-agent for failure path");

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ICE-306"));
    assert!(!stderr.contains("forced enclave failure"));

    if icebox_home.exists() {
        fs::remove_dir_all(&icebox_home).expect("temp ICEBOX_HOME cleanup should succeed");
    }
}
