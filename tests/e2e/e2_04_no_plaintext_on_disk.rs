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

fn decode_fake_wrapped_private_key(key_enc: &[u8]) -> Vec<u8> {
    let prefix = b"fake-enclave-wrap-v1:";
    let encoded = key_enc
        .strip_prefix(prefix)
        .expect("fake wrap prefix should be present");
    encoded.iter().map(|byte| byte ^ 0xAA).collect()
}

#[test]
fn e2_04_register_agent_persists_only_wrapped_private_key_material() {
    let icebox_home = temp_path("icebox-e2-04");
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

    let agent_dir = icebox_home.join("identities").join("claw");
    let mut names: Vec<String> = fs::read_dir(&agent_dir)
        .expect("agent directory should be readable")
        .map(|entry| {
            entry
                .expect("directory entry should be readable")
                .file_name()
                .to_string_lossy()
                .to_string()
        })
        .collect();
    names.sort();
    assert_eq!(names, vec!["enclave.keyref", "identity.pub", "key.enc"]);

    let key_enc = fs::read(agent_dir.join("key.enc")).expect("key.enc should be readable");
    let raw_private_key = decode_fake_wrapped_private_key(&key_enc);

    for file_name in ["enclave.keyref", "identity.pub", "key.enc"] {
        let bytes = fs::read(agent_dir.join(file_name)).expect("artifact should be readable");
        assert!(
            !bytes
                .windows(raw_private_key.len())
                .any(|window| window == raw_private_key),
            "{file_name} should not contain plaintext private key bytes"
        );
    }

    fs::remove_dir_all(&icebox_home).expect("temp ICEBOX_HOME cleanup should succeed");
}

#[test]
fn e2_04_register_agent_cleans_partial_files_on_persistence_failure() {
    let icebox_home = temp_path("icebox-e2-04-failure");
    let output = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("register-agent")
        .arg("claw")
        .env("ICEBOX_HOME", &icebox_home)
        .env("ICEBOX_TEST_FAKE_ENCLAVE", "1")
        .env("ICEBOX_TEST_FORCE_KEY_ENC_PERSIST_ERROR", "1")
        .output()
        .expect("failed to run icebox-cli register-agent for failure path");

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ICE-306"));
    assert!(!stderr.contains("forced key.enc persistence failure"));

    let agent_dir = icebox_home.join("identities").join("claw");
    for file_name in ["enclave.keyref", "identity.pub", "key.enc"] {
        assert!(
            !agent_dir.join(file_name).exists(),
            "{file_name} should not remain after persistence failure"
        );
    }

    if icebox_home.exists() {
        fs::remove_dir_all(&icebox_home).expect("temp ICEBOX_HOME cleanup should succeed");
    }
}
