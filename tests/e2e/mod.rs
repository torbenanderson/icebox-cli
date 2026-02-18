mod e1_02_cli_scaffolding;

#[test]
fn e2e_layout_smoke() {
    assert_eq!(crate::common::project_marker(), "icebox-cli");
}
