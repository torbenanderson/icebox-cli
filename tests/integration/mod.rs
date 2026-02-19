mod e1_01_cargo_init;
mod e1_03_project_structure;

#[test]
fn integration_layout_smoke() {
    assert_eq!(crate::common::project_marker(), "icebox-cli");
}
