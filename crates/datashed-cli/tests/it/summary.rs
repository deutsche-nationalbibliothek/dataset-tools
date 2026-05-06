use std::fs::{File, read_to_string};

use crate::prelude::*;

#[test]
fn summary_stdout() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .arg("summary")
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("{\"docs\": 3, \"size\": 3138}\n"))
        .stderr(predicates::str::is_empty());

    datashed_dir.close()?;
    Ok(())
}

#[test]
fn summary_output() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("summary.json");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["summary", "-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    assert_eq!(
        read_to_string(output.as_path())?,
        "{\"docs\": 3, \"size\": 3138}"
    );

    datashed_dir.close()?;
    Ok(())
}

#[test]
fn summary_allow_list() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;

    let allow = datashed_dir.child("tmp/ALLOW.csv");
    allow.write_str("path\n0/dnb.txt\n0/tib.txt\n")?;

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["summary", "-A", allow.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("{\"docs\": 2, \"size\": 2229}\n"))
        .stderr(predicates::str::is_empty());

    datashed_dir.close()?;
    Ok(())
}

#[test]
fn summary_deny_list() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;

    let deny = datashed_dir.child("tmp/DENY.csv");
    deny.write_str("path\n0/dnb.txt\n")?;

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["summary", "-D", deny.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("{\"docs\": 2, \"size\": 2362}\n"))
        .stderr(predicates::str::is_empty());

    datashed_dir.close()?;
    Ok(())
}

#[test]
fn summary_index() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;

    let mut tmp_index =
        IpcReader::new(File::open(&datashed_dir.join("index.ipc"))?)
            .finish()?
            .lazy()
            .filter(col("path").str().starts_with(lit("0/")))
            .collect()?;

    IpcWriter::new(File::create(
        datashed_dir.join("tmp").join("index.ipc"),
    )?)
    .with_compression(Some(IpcCompression::ZSTD(Default::default())))
    .finish(&mut tmp_index)?;

    let index = datashed_dir.child("tmp/index.ipc");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["summary", "-I", index.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("{\"docs\": 2, \"size\": 2229}\n"))
        .stderr(predicates::str::is_empty());

    datashed_dir.close()?;
    Ok(())
}

#[test]
fn summary_where() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .arg("summary")
        .args(["--where", "path IN ('0/dnb.txt', '0/tib.txt')"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("{\"docs\": 2, \"size\": 2229}\n"))
        .stderr(predicates::str::is_empty());

    datashed_dir.close()?;
    Ok(())
}
