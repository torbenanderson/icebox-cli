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
fn e2_03_register_agent_creates_non_empty_key_enc_blob() {
    let icebox_home = temp_path("icebox-e2-03");
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

    let key_enc_path = icebox_home.join("identities").join("claw").join("key.enc");
    assert!(key_enc_path.exists(), "key.enc should be created");
    let key_enc = fs::read(&key_enc_path).expect("key.enc should be readable");
    assert!(
        !key_enc.is_empty(),
        "key.enc should contain encrypted private-key blob bytes"
    );

    fs::remove_dir_all(&icebox_home).expect("temp ICEBOX_HOME cleanup should succeed");
}

#[test]
fn e2_03_register_agent_reports_deterministic_error_on_wrap_failure() {
    let icebox_home = temp_path("icebox-e2-03-failure");
    let output = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("register-agent")
        .arg("claw")
        .env("ICEBOX_HOME", &icebox_home)
        .env("ICEBOX_TEST_FAKE_ENCLAVE", "1")
        .env("ICEBOX_TEST_FORCE_ENCLAVE_WRAP_ERROR", "1")
        .output()
        .expect("failed to run icebox-cli register-agent for failure path");

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ICE-311"));
    assert!(stderr.contains(
        "Secure Enclave operation failed. Check supported hardware and signing/entitlements."
    ));
    assert!(!stderr.contains("forced enclave failure"));

    let key_enc_path = icebox_home.join("identities").join("claw").join("key.enc");
    assert!(
        !key_enc_path.exists(),
        "key.enc should not be created when wrap fails"
    );

    if icebox_home.exists() {
        fs::remove_dir_all(&icebox_home).expect("temp ICEBOX_HOME cleanup should succeed");
    }
}
