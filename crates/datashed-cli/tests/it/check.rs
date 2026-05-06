use crate::prelude::*;

#[test]
fn check_default() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let checks = datashed_dir.child("checks.toml");
    checks.write_str("")?;

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["check", "-q"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    datashed_dir.close()?;
    Ok(())
}

#[test]
fn check_pass() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let checks = datashed_dir.child("checks.toml");
    checks.write_str("[check.001]\nquery = 'SELECT COUNT(*) == 0 FROM index WHERE size <= 0'",
    )?;

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["check", "-q"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn check_fail() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let checks = datashed_dir.child("checks.toml");
    checks.write_str("[check.001]\nquery = 'SELECT COUNT(*) == 0 FROM index WHERE size > 0'")?;

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["check", "-q"])
        .assert();

    assert
        .failure()
        .code(1)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    datashed_dir.close()?;
    Ok(())
}

#[test]
fn check_skip() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let checks = datashed_dir.child("checks.toml");
    checks.write_str("[check.001]\nskip = true\nquery = 'SELECT COUNT(*) == 0 FROM index WHERE size > 0'",
    )?;

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["check", "-q"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    datashed_dir.close()?;
    Ok(())
}
