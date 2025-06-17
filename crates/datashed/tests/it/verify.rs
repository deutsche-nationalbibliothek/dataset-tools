use std::fs::{self, read_to_string, remove_file};
use std::thread::sleep;
use std::time::Duration;

use crate::prelude::*;

#[test]
fn verify_default() -> TestResult {
    let datashed_dir = create_datashed()?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .arg("index")
        .args(["-q"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd.current_dir(&datashed_dir).arg("verify").assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn verify_permissive() -> TestResult {
    let datashed_dir = create_datashed()?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["index", "--quiet"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["verify", "--mode", "permissive"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn verify_strict() -> TestResult {
    let datashed_dir = create_datashed()?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["index", "--quiet"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["verify", "--mode", "strict"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn verify_missing_file() -> TestResult {
    let datashed_dir = create_datashed()?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["index", "--quiet"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    remove_file(datashed_dir.join("data").join("0/dnb.txt"))?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["verify", "--mode", "permissive"])
        .assert();

    let error = format!(
        "error: verification failed: file not found (path = {}/data/0/dnb.txt).\n",
        datashed_dir.display()
    );

    assert
        .failure()
        .code(1)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::ord::eq(error));

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["verify", "--mode", "strict"])
        .assert();

    let error = format!(
        "error: verification failed: file not found (path = {}/data/0/dnb.txt).\n",
        datashed_dir.display()
    );

    assert
        .failure()
        .code(1)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::ord::eq(error));

    Ok(())
}

#[test]
fn verify_hash_mismatch() -> TestResult {
    let datashed_dir = create_datashed()?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["index", "--quiet"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    fs::write(
        datashed_dir.join("data").join("0/dnb.txt"),
        "Hello, world",
    )?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["verify", "--mode", "permissive"])
        .assert();

    let error = format!(
        "error: verification failed: hash mismatch (path = {}/data/0/dnb.txt).\n",
        datashed_dir.display()
    );

    assert
        .failure()
        .code(1)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::ord::eq(error.clone()));

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["verify", "--mode", "strict"])
        .assert();

    assert
        .failure()
        .code(1)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::ord::eq(error));

    Ok(())
}

#[test]
fn verify_mtime_mismatch() -> TestResult {
    let datashed_dir = create_datashed()?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["index", "--quiet"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    sleep(Duration::from_secs(1));

    let contents = read_to_string(data_dir().join("dnb.txt"))?;
    fs::write(datashed_dir.join("data/0/dnb.txt"), contents)?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["verify", "--mode", "permissive"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let error = format!(
        "error: verification failed: mtime mismatch (path = {}/data/0/dnb.txt).\n",
        datashed_dir.display()
    );

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["verify", "--mode", "strict"])
        .assert();

    assert
        .failure()
        .code(1)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::ord::eq(error));

    Ok(())
}
