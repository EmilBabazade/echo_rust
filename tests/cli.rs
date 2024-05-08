use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn echo_fails_with_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echo_rust")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn echo_runs_with_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echo_rust")?;
    cmd.arg("poop").assert().success();
    cmd.arg("poop").arg("peep").assert().success();
    Ok(())
}

fn run(args: &[&str], expected_result: &str) -> TestResult {
    Command::cargo_bin("echo_rust")?
        .args(args)
        .assert()
        .success()
        .stdout(String::from(expected_result));
    Ok(())
}

#[test]
fn echo_newline_single_arg() -> TestResult {
    run(&["hello world"], "hello world\n")
}

#[test]
fn echo_newline_multiple_args() -> TestResult {
    run(&["hello", "world"], "hello\nworld\n")
}

#[test]
fn echo_no_newline_single_arg() -> TestResult {
    run(&["hello world", "-n"], "hello world ")
}

#[test]
fn echo_no_newline_multiple_args() -> TestResult {
    run(&["-n", "hello", "world"], "hello world ")
}
