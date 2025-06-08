pub(crate) type TestResult = anyhow::Result<()>;
pub(crate) use assert_cmd::Command;
pub(crate) use assert_fs::TempDir;
pub(crate) use datashed::{Config, Datashed};

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

    Ok(temp_dir)
}
