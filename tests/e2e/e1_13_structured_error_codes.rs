use std::process::Command;

#[test]
fn e1_13_default_mode_emits_structured_code() {
    let output = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("--not-a-real-flag")
        .output()
        .expect("failed to run icebox-cli with invalid flag");

    assert_eq!(output.status.code(), Some(2));

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ICE-701"));
    assert!(!stderr.contains("--not-a-real-flag"));
}

#[test]
fn e1_13_debug_mode_emits_structured_code_and_detail() {
    let output = Command::new(env!("CARGO_BIN_EXE_icebox-cli"))
        .arg("--debug")
        .arg("--not-a-real-flag")
        .output()
        .expect("failed to run icebox-cli with invalid flag and --debug");

    assert_eq!(output.status.code(), Some(2));

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ICE-701"));
    assert!(stderr.contains("--not-a-real-flag"));
}
