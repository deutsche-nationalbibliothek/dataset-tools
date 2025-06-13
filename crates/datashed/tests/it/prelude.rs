use std::env::current_dir;
use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;

pub(crate) type TestResult = anyhow::Result<()>;

pub(crate) use assert_cmd::Command;
pub(crate) use assert_fs::TempDir;
pub(crate) use datashed::{Config, Datashed};
// pub(crate) use predicates::prelude::*;

pub(crate) fn data_dir() -> &'static PathBuf {
    static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
        current_dir()
            .unwrap()
            .join("tests/data")
            .canonicalize()
            .unwrap()
            .to_path_buf()
    });

    &DATA_DIR
}

pub(crate) fn create_datashed() -> anyhow::Result<TempDir> {
    let mut cmd = Command::cargo_bin("datashed")?;
    let temp_dir = TempDir::new()?;

    let assert = cmd.current_dir(&temp_dir).arg("init").assert();
    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::starts_with("Initialized datashed"));

    let path = temp_dir.join(Datashed::CONFIG);
    let config = Config::from_path(path)?;

    assert_eq!(config.metadata.version.to_string(), "0.1.0");
    assert!(config.metadata.description.is_none());

    fs::create_dir(temp_dir.join("data").join("0"))?;
    fs::create_dir(temp_dir.join("data").join("1"))?;

    fs::copy(
        data_dir().join("dnb.txt"),
        temp_dir.join("data/0/dnb.txt"),
    )?;

    fs::copy(
        data_dir().join("tib.txt"),
        temp_dir.join("data/0/tib.txt"),
    )?;

    fs::copy(
        data_dir().join("zbw.txt"),
        temp_dir.join("data/1/zbw.txt"),
    )?;

    Ok(temp_dir)
}
