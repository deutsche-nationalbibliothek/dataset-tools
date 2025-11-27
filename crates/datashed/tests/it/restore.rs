use crate::prelude::*;

#[test]
fn restore_default() -> TestResult {
    let datashed_dir = create_datashed()?;

    // create index
    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .arg("index")
        .assert();
    assert.success();

    // create backup
    let temp_dir = TempDir::new()?;
    let archive = temp_dir.join("archive.tar.gz");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["archive", "-q"])
        .args(["-o", archive.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    // restore backup
    let dest_dir = TempDir::new()?;

    let assert = datashed_cmd()
        .current_dir(&dest_dir)
        .args(["restore", archive.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::starts_with(
            "Successfully restored archive.",
        ));

    // verify
    let assert =
        datashed_cmd().current_dir(&dest_dir).arg("verify").assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    temp_dir.close()?;
    dest_dir.close()?;
    Ok(())
}

#[test]
fn restore_directory() -> TestResult {
    let datashed_dir = create_datashed()?;

    // create index
    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .arg("index")
        .assert();
    assert.success();

    // create backup
    let temp_dir = TempDir::new()?;
    let archive = temp_dir.join("archive.tar.gz");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["archive", "-q"])
        .args(["-o", archive.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    // restore backup
    let dest_dir = TempDir::new()?;
    let out_dir = dest_dir.join("out");

    let assert = datashed_cmd()
        .current_dir(&dest_dir)
        .args(["restore", archive.to_str().unwrap()])
        .args(["--directory", out_dir.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::starts_with(
            "Successfully restored archive.",
        ));

    // verify
    let assert =
        datashed_cmd().current_dir(&out_dir).arg("verify").assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    temp_dir.close()?;
    dest_dir.close()?;
    Ok(())
}

#[test]
fn restore_verbose() -> TestResult {
    let datashed_dir = create_datashed()?;

    // create index
    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .arg("index")
        .assert();
    assert.success();

    // create backup
    let temp_dir = TempDir::new()?;
    let archive = temp_dir.join("archive.tar.gz");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["archive", "-q"])
        .args(["-o", archive.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    // restore backup
    let dest_dir = TempDir::new()?;
    let out_dir = dest_dir.join("out");

    let assert = datashed_cmd()
        .current_dir(&dest_dir)
        .args(["restore", "-v", archive.to_str().unwrap()])
        .args(["-C", out_dir.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::starts_with(format!(
            "Created destination directory '{}'.",
            out_dir.display()
        )))
        .stderr(predicates::str::contains(
            "Successfully restored archive.",
        ));

    // verify
    let assert =
        datashed_cmd().current_dir(&out_dir).arg("verify").assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    temp_dir.close()?;
    dest_dir.close()?;
    Ok(())
}

#[test]
fn restore_quiet() -> TestResult {
    let datashed_dir = create_datashed()?;

    // create index
    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .arg("index")
        .assert();
    assert.success();

    // create backup
    let temp_dir = TempDir::new()?;
    let archive = temp_dir.join("archive.tar.gz");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["archive", "-q"])
        .args(["-o", archive.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    // restore backup
    let dest_dir = TempDir::new()?;

    let assert = datashed_cmd()
        .current_dir(&dest_dir)
        .args(["restore", "-q", archive.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    // verify
    let assert =
        datashed_cmd().current_dir(&dest_dir).arg("verify").assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    temp_dir.close()?;
    dest_dir.close()?;
    Ok(())
}
