use std::process;

use crate::prelude::*;

#[test]
fn init_default() -> TestResult {
    let mut cmd = Command::cargo_bin("datashed")?;
    let temp_dir = TempDir::new()?;

    let assert = cmd
        .current_dir(&temp_dir)
        .arg("init")
        .arg("test-data")
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::ord::eq(format!(
            "Initialized datashed in {}\n",
            temp_dir.join("test-data").to_str().unwrap()
        )));

    let path = temp_dir.join("test-data").join(Datashed::CONFIG);
    let config = Config::from_path(path)?;

    assert_eq!(config.metadata.name, "test-data");
    assert_eq!(config.metadata.version.to_string(), "0.1.0");
    assert!(config.metadata.description.is_none());

    Ok(())
}

#[test]
fn init_set_name() -> TestResult {
    let mut cmd = Command::cargo_bin("datashed")?;
    let temp_dir = TempDir::new()?;

    let assert = cmd
        .current_dir(&temp_dir)
        .arg("init")
        .args(["--name", "foobar"])
        .arg("test-data")
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::ord::eq(format!(
            "Initialized datashed in {}\n",
            temp_dir.join("test-data").to_str().unwrap()
        )));

    let path = temp_dir.join("test-data").join(Datashed::CONFIG);
    let config = Config::from_path(path)?;

    assert_eq!(config.metadata.name, "foobar");
    assert_eq!(config.metadata.version.to_string(), "0.1.0");
    assert!(config.metadata.description.is_none());

    Ok(())
}

#[test]
fn init_set_author() -> TestResult {
    let mut cmd = Command::cargo_bin("datashed")?;
    let temp_dir = TempDir::new()?;

    let assert = cmd
        .current_dir(&temp_dir)
        .arg("init")
        .args(["--author", "Max Mustermann <m.muster@example.com>"])
        .arg("test-data")
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::ord::eq(format!(
            "Initialized datashed in {}\n",
            temp_dir.join("test-data").to_str().unwrap()
        )));

    let path = temp_dir.join("test-data").join(Datashed::CONFIG);
    let config = Config::from_path(path)?;

    assert_eq!(config.metadata.name, "test-data");
    assert_eq!(config.metadata.version.to_string(), "0.1.0");
    assert!(config.metadata.description.is_none());
    assert_eq!(
        config.metadata.authors,
        vec!["Max Mustermann <m.muster@example.com>"]
    );

    Ok(())
}

#[test]
fn init_derive_author() -> TestResult {
    let temp_dir = TempDir::new()?;

    assert!(
        process::Command::new("git")
            .arg("init")
            .current_dir(&temp_dir)
            .status()?
            .success()
    );

    assert!(
        process::Command::new("git")
            .arg("config")
            .args(["user.name", "Max Mustermann"])
            .current_dir(&temp_dir)
            .status()?
            .success()
    );

    assert!(
        process::Command::new("git")
            .arg("config")
            .args(["user.email", "m.muster@example.com"])
            .current_dir(&temp_dir)
            .status()?
            .success()
    );

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&temp_dir)
        .arg("init")
        .arg("--verbose")
        .assert();
    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::contains(
            "Set author to Git identity 'Max Mustermann <m.muster@example.com>"
        ))
        .stderr(predicates::str::contains(format!(
            "Initialized datashed in {}\n",
            temp_dir.to_str().unwrap()
        )));

    let config = Config::from_path(temp_dir.join(Datashed::CONFIG))?;
    assert_eq!(
        config.metadata.authors,
        vec!["Max Mustermann <m.muster@example.com>"]
    );

    Ok(())
}

#[test]
fn init_set_description() -> TestResult {
    let mut cmd = Command::cargo_bin("datashed")?;
    let temp_dir = TempDir::new()?;

    let assert = cmd
        .current_dir(&temp_dir)
        .arg("init")
        .args(["--description", "foobar"])
        .arg("test-data")
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::ord::eq(format!(
            "Initialized datashed in {}\n",
            temp_dir.join("test-data").to_str().unwrap()
        )));

    let path = temp_dir.join("test-data").join(Datashed::CONFIG);
    let config = Config::from_path(path)?;

    assert_eq!(config.metadata.name, "test-data");
    assert_eq!(config.metadata.version.to_string(), "0.1.0");
    assert_eq!(config.metadata.description, Some("foobar".into()));

    Ok(())
}

#[test]
fn init_set_version() -> TestResult {
    let mut cmd = Command::cargo_bin("datashed")?;
    let temp_dir = TempDir::new()?;

    let assert = cmd
        .current_dir(&temp_dir)
        .arg("init")
        .args(["--version", "0.2.0"])
        .arg("test-data")
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::ord::eq(format!(
            "Initialized datashed in {}\n",
            temp_dir.join("test-data").to_str().unwrap()
        )));

    let path = temp_dir.join("test-data").join(Datashed::CONFIG);
    let config = Config::from_path(path)?;

    assert_eq!(config.metadata.name, "test-data");
    assert_eq!(config.metadata.version.to_string(), "0.2.0");
    assert!(config.metadata.description.is_none());

    // invalid version
    let mut cmd = Command::cargo_bin("datashed")?;
    let temp_dir = TempDir::new()?;

    let assert = cmd
        .current_dir(&temp_dir)
        .arg("init")
        .args(["--version", "X.2.0"])
        .arg("test-data")
        .assert();

    assert
        .failure()
        .code(2)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::starts_with(
            "error: invalid value 'X.2.0'",
        ));

    Ok(())
}

#[test]
fn init_vcs_git() -> TestResult {
    let mut cmd = Command::cargo_bin("datashed")?;
    let temp_dir = TempDir::new()?;

    let assert = cmd
        .current_dir(&temp_dir)
        .arg("init")
        .args(["--vcs", "git"])
        .arg("test-data")
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::contains(format!(
            "Initialized datashed in {}\n",
            temp_dir.join("test-data").to_str().unwrap()
        )));

    assert!(temp_dir.join("test-data").join(".git").exists());
    assert!(temp_dir.join("test-data").join(".gitignore").exists());

    let mut cmd = Command::cargo_bin("datashed")?;
    let temp_dir = TempDir::new()?;

    let assert = cmd
        .current_dir(&temp_dir)
        .arg("init")
        .arg("test-data")
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::ord::eq(format!(
            "Initialized datashed in {}\n",
            temp_dir.join("test-data").to_str().unwrap()
        )));

    assert!(temp_dir.join("test-data").join(".git").exists());
    assert!(temp_dir.join("test-data").join(".gitignore").exists());

    Ok(())
}

#[test]
fn init_vcs_none() -> TestResult {
    let mut cmd = Command::cargo_bin("datashed")?;
    let temp_dir = TempDir::new()?;

    let assert = cmd
        .current_dir(&temp_dir)
        .arg("init")
        .args(["--vcs", "none"])
        .arg("test-data")
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::ord::eq(format!(
            "Initialized datashed in {}\n",
            temp_dir.join("test-data").to_str().unwrap()
        )));

    assert!(!temp_dir.join("test-data").join(".git").exists());
    assert!(!temp_dir.join("test-data").join(".gitignore").exists());

    Ok(())
}

#[test]
fn init_force() -> TestResult {
    let mut cmd = Command::cargo_bin("datashed")?;
    let temp_dir = TempDir::new()?;

    let assert = cmd
        .current_dir(&temp_dir)
        .arg("init")
        .arg("test-data")
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::ord::eq(format!(
            "Initialized datashed in {}\n",
            temp_dir.join("test-data").to_str().unwrap()
        )));

    let path = temp_dir.join("test-data").join(Datashed::CONFIG);
    let config = Config::from_path(path)?;

    assert_eq!(config.metadata.name, "test-data");

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&temp_dir)
        .arg("init")
        .args(["--name", "foobar"])
        .arg("--force")
        .arg("test-data")
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::ord::eq(format!(
            "Initialized datashed in {}\n",
            temp_dir.join("test-data").to_str().unwrap()
        )));

    let path = temp_dir.join("test-data").join(Datashed::CONFIG);
    let config = Config::from_path(path)?;

    assert_eq!(config.metadata.name, "foobar");

    Ok(())
}

#[test]
fn init_quiet() -> TestResult {
    let mut cmd = Command::cargo_bin("datashed")?;
    let temp_dir = TempDir::new()?;

    let assert = cmd
        .current_dir(&temp_dir)
        .arg("init")
        .arg("--quiet")
        .arg("test-data")
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let path = temp_dir.join("test-data").join(Datashed::CONFIG);
    let config = Config::from_path(path)?;

    assert_eq!(config.metadata.name, "test-data");
    assert_eq!(config.metadata.version.to_string(), "0.1.0");
    assert!(config.metadata.description.is_none());

    Ok(())
}
