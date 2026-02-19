use icebox_cli::{agent, config, did, runner, vault};

#[test]
fn e1_03_modules_compile_and_expose_expected_scaffolds() {
    let parsed_name = agent::IdentityName::parse("core-agent").expect("identity name should parse");
    assert_eq!(parsed_name.as_str(), "core-agent");

    let cfg = config::RuntimeConfig::default();
    assert_eq!(cfg.schema_version, 1);

    let vault_ref = vault::VaultRef::for_identity("id-123").expect("vault ref should build");
    assert_eq!(vault_ref.identity_id, "id-123");

    let request = runner::RunRequest::new("id-123", "openai").expect("run request should build");
    assert_eq!(request.service, "openai");

    #[cfg(target_os = "macos")]
    assert_eq!(did::enclave_backend_name(), "secure-enclave");
    #[cfg(not(target_os = "macos"))]
    assert_eq!(did::enclave_backend_name(), "stub");
}

#[test]
fn e1_03_identity_validation_rejects_empty_name() {
    let err = agent::IdentityName::parse("   ").expect_err("empty names must fail");
    assert_eq!(err.to_string(), "identity name cannot be empty");
}
