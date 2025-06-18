use std::process;

use assert_cmd::assert::OutputAssertExt;
use predicates::path::is_file;

use crate::prelude::*;

#[test]
fn completions_bash() -> TestResult {
    let mut cmd = Command::cargo_bin("datashed")?;
    let temp_dir = TempDir::new()?;
    let out = temp_dir.join("out.sh");

    let assert = cmd
        .args(["completions", "bash"])
        .args(["-o", out.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    assert!(is_file().eval(&out));

    let mut cmd = process::Command::new("bash");
    let assert = cmd.args(["-n", out.to_str().unwrap()]).assert();
    assert.success();

    Ok(())
}

#[test]
fn completions_elvish() -> TestResult {
    let mut cmd = Command::cargo_bin("datashed")?;
    let temp_dir = TempDir::new()?;
    let out = temp_dir.join("out.sh");

    let assert = cmd
        .args(["completions", "elvish"])
        .args(["-o", out.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    assert!(is_file().eval(&out));

    Ok(())
}

#[test]
fn completions_fish() -> TestResult {
    let mut cmd = Command::cargo_bin("datashed")?;
    let temp_dir = TempDir::new()?;
    let out = temp_dir.join("out.sh");

    let assert = cmd
        .args(["completions", "fish"])
        .args(["-o", out.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    assert!(is_file().eval(&out));

    Ok(())
}

#[test]
fn completions_zsh() -> TestResult {
    let mut cmd = Command::cargo_bin("datashed")?;
    let temp_dir = TempDir::new()?;
    let out = temp_dir.join("out.sh");

    let assert = cmd
        .args(["completions", "zsh"])
        .args(["-o", out.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    assert!(is_file().eval(&out));

    Ok(())
}
