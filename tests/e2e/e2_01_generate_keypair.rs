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
fn e2_01_register_agent_creates_identity_dir_and_public_key() {
    let icebox_home = temp_path("icebox-e2-01");
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

    let identity_pub = icebox_home
        .join("identities")
        .join("claw")
        .join("identity.pub");
    assert!(identity_pub.exists(), "identity.pub should be created");

    let bytes = fs::read(&identity_pub).expect("identity.pub should be readable");
    assert_eq!(bytes.len(), 32, "Ed25519 public key should be 32 bytes");

    fs::remove_dir_all(&icebox_home).expect("temp ICEBOX_HOME cleanup should succeed");
}

#[test]
fn e2_01_register_agent_fails_when_home_path_is_file() {
    let icebox_home = temp_path("icebox-e2-01-failure");
    fs::write(&icebox_home, "not-a-directory").expect("should create blocking file path");

    let output = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("register-agent")
        .arg("claw")
        .env("ICEBOX_HOME", &icebox_home)
        .output()
        .expect("failed to run icebox-cli register-agent for failure path");

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ICE-306"));
    assert!(!stderr.contains("failed to create agent directory"));

    fs::remove_file(&icebox_home).expect("temp ICEBOX_HOME file cleanup should succeed");
}
