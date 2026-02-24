use std::process::Command;

#[test]
fn e1_02_help_path_returns_usage_and_zero() {
    let output = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("--help")
        .output()
        .expect("failed to run icebox-cli --help");

    assert!(output.status.success(), "expected --help to exit 0");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage: icebox-cli"));
    assert!(stdout.contains("OpenClaw lobster mode"));
}

#[test]
fn e1_02_unknown_flag_returns_non_zero() {
    let output = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("--not-a-real-flag")
        .output()
        .expect("failed to run icebox-cli with invalid flag");

    assert_eq!(output.status.code(), Some(2));

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ICE-701"));
}
