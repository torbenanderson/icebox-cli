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
fn e2_18_register_agent_accepts_canonical_name() {
    let icebox_home = temp_path("icebox-e2-18-happy");
    let output = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("register-agent")
        .arg("core-agent1")
        .env("ICEBOX_HOME", &icebox_home)
        .env("ICEBOX_TEST_FAKE_ENCLAVE", "1")
        .output()
        .expect("failed to run icebox-cli register-agent");

    assert!(
        output.status.success(),
        "expected register-agent success; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        icebox_home
            .join("identities")
            .join("core-agent1")
            .join("identity.pub")
            .exists(),
        "identity artifacts should be created for valid name"
    );

    fs::remove_dir_all(&icebox_home).expect("temp ICEBOX_HOME cleanup should succeed");
}

#[test]
fn e2_18_register_agent_rejects_invalid_names_with_safe_error() {
    for invalid in ["-foo", "Claw", "ab", "bad_name", "with space"] {
        let icebox_home = temp_path("icebox-e2-18-failure");
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_icebox-cli"));
        cmd.arg("register-agent");
        if invalid.starts_with('-') {
            cmd.arg("--");
        }
        let output = cmd
            .arg(invalid)
            .env("ICEBOX_HOME", &icebox_home)
            .env("ICEBOX_TEST_FAKE_ENCLAVE", "1")
            .output()
            .expect("failed to run icebox-cli register-agent");

        assert_eq!(
            output.status.code(),
            Some(1),
            "invalid name should fail: {invalid}"
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("ICE-306"),
            "runtime error code should be shown for invalid name"
        );
        assert!(
            !stderr.contains("identity name must match [a-z0-9-]{3,32}"),
            "default mode should keep detail user-safe"
        );

        assert!(
            !icebox_home.join("identities").join(invalid).exists(),
            "identity artifacts should not be created for invalid name"
        );
        if icebox_home.exists() {
            fs::remove_dir_all(&icebox_home).expect("temp ICEBOX_HOME cleanup should succeed");
        }
    }
}
