use icebox_cli::agent;

#[test]
fn e2_18_identity_validation_enforces_canonical_name_rules() {
    let err = agent::IdentityName::parse("Claw").expect_err("uppercase names must fail");
    assert_eq!(err.to_string(), "identity name must match [a-z0-9-]{3,32}");

    let err = agent::IdentityName::parse("-foo").expect_err("leading hyphen names must fail");
    assert_eq!(err.to_string(), "identity name must match [a-z0-9-]{3,32}");

    let err = agent::IdentityName::parse("ab").expect_err("too-short names must fail");
    assert_eq!(err.to_string(), "identity name must match [a-z0-9-]{3,32}");
}
