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
    cmd.arg("poop").arg("peep").assert().success();
}

#[test]
fn echo_outputs_args_seperated_by_newline() {
    let expected = "hello world\n";
    let mut cmd = Command::cargo_bin("echo_rust").unwrap();
    cmd.arg("hello world").assert().success().stdout(expected);
}

#[test]
fn echo_newline_single_arg() {
    let expected = "hello world\n";
    let mut cmd = Command::cargo_bin("echo_rust").unwrap();
    cmd.arg("hello world").assert().success().stdout(expected);
}

#[test]
fn echo_newline_multiple_args() {
    let expected = "hello\nworld\n";
    let mut cmd = Command::cargo_bin("echo_rust").unwrap();
    cmd.arg("hello")
        .arg("world")
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn echo_no_newline_single_arg() {
    let expected = "hello world ";
    let mut cmd = Command::cargo_bin("echo_rust").unwrap();
    cmd.arg("-n")
        .arg("hello world")
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn echo_no_newline_multiple_args() {
    let expected = "hello world ";
    let mut cmd = Command::cargo_bin("echo_rust").unwrap();
    cmd.arg("-n")
        .arg("hello")
        .arg("world")
        .assert()
        .success()
        .stdout(expected);
}
