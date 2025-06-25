use crate::prelude::*;

#[test]
fn config_default() -> TestResult {
    let datashed_dir = create_datashed()?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["config", "runtime.num-jobs"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("runtime.num-jobs = None\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn config_get() -> TestResult {
    let datashed_dir = create_datashed()?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["config", "--get", "runtime.num-jobs"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("runtime.num-jobs = None\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn config_set() -> TestResult {
    let datashed_dir = create_datashed()?;

    // verify option is unset
    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["config", "--get", "runtime.num-jobs"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("runtime.num-jobs = None\n"))
        .stderr(predicates::str::is_empty());

    // set to new value
    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["config", "--set", "runtime.num-jobs", "12"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    // get new value
    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["config", "--get", "runtime.num-jobs"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("runtime.num-jobs = 12\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn config_unset() -> TestResult {
    let datashed_dir = create_datashed()?;

    // set to new value
    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["config", "--set", "runtime.num-jobs", "12"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    // verify option is set
    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["config", "--get", "runtime.num-jobs"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("runtime.num-jobs = 12\n"))
        .stderr(predicates::str::is_empty());

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["config", "--unset", "runtime.num-jobs"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    // verify option is unset
    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["config", "runtime.num-jobs"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("runtime.num-jobs = None\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}
