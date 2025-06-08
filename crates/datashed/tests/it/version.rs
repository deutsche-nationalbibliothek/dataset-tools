use crate::prelude::*;

#[test]
fn version_default() -> TestResult {
    let mut cmd = Command::cargo_bin("datashed")?;
    let temp_dir = create_datashed()?;

    let assert = cmd.current_dir(&temp_dir).arg("version").assert();
    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("0.1.0\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn version_set_version() -> TestResult {
    let temp_dir = create_datashed()?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&temp_dir)
        .arg("version")
        .arg("0.2.0")
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd.current_dir(&temp_dir).arg("version").assert();
    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("0.2.0\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn version_reject_smaller() -> TestResult {
    let temp_dir = create_datashed()?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&temp_dir)
        .arg("version")
        .arg("0.0.1")
        .assert();

    assert
        .failure()
        .code(1)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::ord::eq(
            "error: 0.0.1 must be greater than 0.1.0\n",
        ));

    Ok(())
}

#[test]
fn version_reject_invalid() -> TestResult {
    let temp_dir = create_datashed()?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&temp_dir)
        .arg("version")
        .arg("X.2.0")
        .assert();

    assert
        .failure()
        .code(2)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::starts_with(
            "error: invalid value 'X.2.0",
        ));

    Ok(())
}

#[test]
fn version_bump_major() -> TestResult {
    let temp_dir = create_datashed()?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&temp_dir)
        .arg("version")
        .args(["--bump", "major"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd.current_dir(&temp_dir).arg("version").assert();
    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("1.0.0\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn version_bump_minor() -> TestResult {
    let temp_dir = create_datashed()?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&temp_dir)
        .arg("version")
        .args(["--bump", "minor"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd.current_dir(&temp_dir).arg("version").assert();
    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("0.2.0\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn version_bump_patch() -> TestResult {
    let temp_dir = create_datashed()?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&temp_dir)
        .arg("version")
        .args(["--bump", "patch"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd.current_dir(&temp_dir).arg("version").assert();
    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("0.1.1\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}
