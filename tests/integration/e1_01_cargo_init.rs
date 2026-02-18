use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn e1_01_scaffold_contains_manifest_and_main() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest_path = repo_root.join("Cargo.toml");
    let main_path = repo_root.join("src/main.rs");

    assert!(manifest_path.exists(), "expected Cargo.toml to exist");
    assert!(main_path.exists(), "expected src/main.rs to exist");

    let manifest = fs::read_to_string(manifest_path).expect("failed to read Cargo.toml");
    assert!(manifest.contains("[package]"));
    assert!(manifest.contains("name = \"icebox-cli\""));
}

#[test]
fn e1_01_missing_manifest_path_fails_metadata() {
    let output = Command::new("cargo")
        .args([
            "metadata",
            "--format-version",
            "1",
            "--no-deps",
            "--manifest-path",
            "/tmp/icebox-missing-manifest/Cargo.toml",
        ])
        .output()
        .expect("failed to invoke cargo metadata");

    assert!(
        !output.status.success(),
        "expected cargo metadata to fail for missing manifest path"
    );
}
