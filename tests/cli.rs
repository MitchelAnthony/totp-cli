use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn runs_with_arg_secret() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("totp").unwrap();
    cmd.arg("--secret=7NY4BTFIYHYGT6OGSAP3XG4VAV7NOGA2");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match("^(\\d){6}").unwrap());

    Ok(())
}

#[test]
fn runs_with_arg_auth_url() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("totp").unwrap();
    cmd.arg(format!(
        "--auth-url={}",
        "otpauth://totp/John%20Doe?secret=7NY4BTFIYHYGT6OGSAP3XG4VAV7NOGA2&issuer=Acme%20Corp"
    ));
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match("^(\\d){6}").unwrap());

    Ok(())
}

#[test]
fn fails_without_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("totp").unwrap();
    cmd.assert().failure();

    Ok(())
}
