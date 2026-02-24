mod e1_02_cli_scaffolding;
mod e1_13_structured_error_codes;
mod e2_01_generate_keypair;

#[test]
fn e2e_layout_smoke() {
    assert_eq!(crate::common::project_marker(), "icebox-cli");
}
