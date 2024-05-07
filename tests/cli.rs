use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn echo_fails_no_args() {
    let mut cmd = Command::cargo_bin("echo_rust").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn echo_runs_with_args() {
    let mut cmd = Command::cargo_bin("echo_rust").unwrap();
    cmd.arg("poop").assert().success();
}
