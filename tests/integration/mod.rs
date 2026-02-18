mod e1_01_cargo_init;

#[test]
fn integration_layout_smoke() {
    assert_eq!(crate::common::project_marker(), "icebox-cli");
}
