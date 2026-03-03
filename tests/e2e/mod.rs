mod e1_02_cli_scaffolding;
mod e1_13_structured_error_codes;
mod e2_01_generate_keypair;
mod e2_02_enclave_wrapping_key;
mod e2_03_wrap_private_key;
mod e2_04_no_plaintext_on_disk;
mod e2_11_active_agent_tracking;

#[test]
fn e2e_layout_smoke() {
    assert_eq!(crate::common::project_marker(), "icebox-cli");
}
