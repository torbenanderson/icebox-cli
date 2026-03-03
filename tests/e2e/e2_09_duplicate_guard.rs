use serde_json::Value;
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
fn e2_09_register_agent_rejects_duplicate_name_without_overwrite() {
    let icebox_home = temp_path("icebox-e2-09");
    let first = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("register-agent")
        .arg("claw")
        .env("ICEBOX_HOME", &icebox_home)
        .env("ICEBOX_TEST_FAKE_ENCLAVE", "1")
        .output()
        .expect("first register-agent run should execute");
    assert!(
        first.status.success(),
        "first registration should succeed; stderr={}",
        String::from_utf8_lossy(&first.stderr)
    );

    let second = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("register-agent")
        .arg("claw")
        .env("ICEBOX_HOME", &icebox_home)
        .env("ICEBOX_TEST_FAKE_ENCLAVE", "1")
        .output()
        .expect("duplicate register-agent run should execute");
    assert_eq!(second.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&second.stderr);
    assert!(stderr.contains("ICE-307"));
    assert!(stderr.contains(
        "Agent claw already exists. Choose a different name or remove the existing agent."
    ));

    let config_bytes =
        fs::read(icebox_home.join("config.json")).expect("config.json should be readable");
    let config: Value = serde_json::from_slice(&config_bytes).expect("config should be valid");
    let agents = config["agents"]
        .as_array()
        .expect("agents should be represented as array");
    assert_eq!(
        agents.len(),
        1,
        "duplicate registration must not append agent"
    );
    assert_eq!(agents[0]["name"], "claw");

    fs::remove_dir_all(&icebox_home).expect("temp ICEBOX_HOME cleanup should succeed");
}

#[test]
fn e2_09_register_agent_fails_closed_when_config_has_duplicate_names() {
    let icebox_home = temp_path("icebox-e2-09-load-failure");
    fs::create_dir_all(&icebox_home).expect("ICEBOX_HOME directory should be creatable");
    fs::write(
        icebox_home.join("config.json"),
        r#"{
  "schemaVersion": 1,
  "activeAgentId": "agent-1",
  "agents": [
    { "agentId": "agent-1", "name": "claw", "did": "did:key:one" },
    { "agentId": "agent-2", "name": "claw", "did": "did:key:two" }
  ]
}"#,
    )
    .expect("config fixture should be writable");

    let output = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("register-agent")
        .arg("core")
        .env("ICEBOX_HOME", &icebox_home)
        .env("ICEBOX_TEST_FAKE_ENCLAVE", "1")
        .output()
        .expect("register-agent should run against duplicate config");
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ICE-310"));
    assert!(stderr.contains(
        "Config has duplicate agent names. Resolve duplicates in ~/.icebox/config.json and retry."
    ));
    assert!(
        !icebox_home.join("identities").join("core").exists(),
        "new identity should not be created when config registry is invalid"
    );

    fs::remove_dir_all(&icebox_home).expect("temp ICEBOX_HOME cleanup should succeed");
}
