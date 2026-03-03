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
fn e2_11_register_agent_updates_active_agent_registry() {
    let icebox_home = temp_path("icebox-e2-11");
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

    let config_path = icebox_home.join("config.json");
    let config_bytes = fs::read(&config_path).expect("config.json should exist");
    let config: Value = serde_json::from_slice(&config_bytes).expect("config.json should be valid");

    let active = config["activeAgentId"]
        .as_str()
        .expect("activeAgentId should be a string");
    let agents = config["agents"]
        .as_array()
        .expect("agents should be an array");
    assert_eq!(agents.len(), 1);

    let entry = &agents[0];
    assert_eq!(entry["name"], "claw");
    let agent_id = entry["agentId"]
        .as_str()
        .expect("agentId should be a string");
    assert_eq!(active, agent_id, "activeAgentId should target new agent");

    let did = entry["did"].as_str().expect("did should be a string");
    assert!(
        did.starts_with("did:key:ed25519-raw:"),
        "did should be deterministic from public key"
    );

    fs::remove_dir_all(&icebox_home).expect("temp ICEBOX_HOME cleanup should succeed");
}

#[test]
fn e2_11_register_agent_repairs_stale_active_agent_id_before_append() {
    let icebox_home = temp_path("icebox-e2-11-repair");
    fs::create_dir_all(&icebox_home).expect("ICEBOX_HOME directory should be creatable");
    fs::write(
        icebox_home.join("config.json"),
        r#"{
  "schemaVersion": 1,
  "activeAgentId": "ghost-agent-id",
  "agents": [
    { "agentId": "existing-agent-id", "name": "existing", "did": "did:key:existing" }
  ]
}"#,
    )
    .expect("seed config should be writable");

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

    let config_bytes =
        fs::read(icebox_home.join("config.json")).expect("config should be readable");
    let config: Value = serde_json::from_slice(&config_bytes).expect("config should be valid");

    let agents = config["agents"]
        .as_array()
        .expect("agents should be an array");
    assert_eq!(agents.len(), 2, "new agent should be appended to registry");
    assert!(
        agents
            .iter()
            .any(|entry| entry["agentId"] == "existing-agent-id"),
        "existing agent should remain registered"
    );

    let new_entry = agents
        .iter()
        .find(|entry| entry["name"] == "claw")
        .expect("new claw entry should exist");
    let new_id = new_entry["agentId"]
        .as_str()
        .expect("new agentId should be string");
    let active = config["activeAgentId"]
        .as_str()
        .expect("activeAgentId should be string");
    assert_eq!(
        active, new_id,
        "activeAgentId should be repaired and set to the newly registered agent"
    );

    fs::remove_dir_all(&icebox_home).expect("temp ICEBOX_HOME cleanup should succeed");
}

#[test]
fn e2_11_register_agent_fails_closed_on_invalid_config_json() {
    let icebox_home = temp_path("icebox-e2-11-invalid-config");
    fs::create_dir_all(&icebox_home).expect("ICEBOX_HOME directory should be creatable");
    fs::write(icebox_home.join("config.json"), "{").expect("broken config write should succeed");

    let output = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("register-agent")
        .arg("claw")
        .env("ICEBOX_HOME", &icebox_home)
        .env("ICEBOX_TEST_FAKE_ENCLAVE", "1")
        .output()
        .expect("failed to run icebox-cli register-agent");

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ICE-306"));
    assert!(
        !stderr.contains("failed to parse config.json"),
        "default mode should not leak parse internals"
    );

    assert!(
        !icebox_home.join("identities").join("claw").exists(),
        "identity artifacts should not be created when config load fails"
    );

    fs::remove_dir_all(&icebox_home).expect("temp ICEBOX_HOME cleanup should succeed");
}
