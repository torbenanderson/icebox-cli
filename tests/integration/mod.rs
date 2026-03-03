mod e1_01_cargo_init;
mod e1_03_project_structure;
mod e2_18_agent_name_validation;

#[test]
fn integration_layout_smoke() {
    assert_eq!(crate::common::project_marker(), "icebox-cli");
}
