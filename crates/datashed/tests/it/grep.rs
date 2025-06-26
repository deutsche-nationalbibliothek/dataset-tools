use std::fs::File;

use assert_fs::prelude::{FileWriteStr, PathChild};
use polars::io::SerReader;
use polars::prelude::*;

use crate::prelude::*;

const HEADER: &str = "path,hash,size,mtime\n";

#[test]
fn grep_default() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("grep.ipc");

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["grep", "-q", "\\(DNB\\)"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(output)?).finish()?;
    assert_eq!(df.column("path")?.str()?.get(0), Some("0/dnb.txt"));
    assert_eq!(df.height(), 1);

    Ok(())
}

#[test]
fn grep_max_bytes() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("grep.ipc");

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["grep", "-q", "-n", "36", "\\(DNB\\)"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(HEADER))
        .stderr(predicates::str::is_empty());

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["grep", "-q", "-n", "37", "\\(DNB\\)"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(output)?).finish()?;
    assert_eq!(df.column("path")?.str()?.get(0), Some("0/dnb.txt"));
    assert_eq!(df.height(), 1);

    Ok(())
}

#[test]
fn grep_ignore_case() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("grep.ipc");

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["grep", "-q", "-i", "\\(dnb\\)"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(output)?).finish()?;
    assert_eq!(df.column("path")?.str()?.get(0), Some("0/dnb.txt"));
    assert_eq!(df.height(), 1);

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["grep", "-q", "\\(dnb\\)"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(HEADER))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn grep_invert_match() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("grep.ipc");

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["grep", "-q", "--invert-match", "\\(DNB\\)"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(output)?).finish()?;
    assert_eq!(df.column("path")?.str()?.get(0), Some("0/tib.txt"));
    assert_eq!(df.column("path")?.str()?.get(1), Some("1/zbw.txt"));
    assert_eq!(df.height(), 2);

    Ok(())
}

#[test]
fn grep_multiple_pattern() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("grep.ipc");

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["grep", "-q", "\\(DNB\\)", "Econ(Stor|Biz)"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(output)?).finish()?;
    assert_eq!(df.column("path")?.str()?.get(0), Some("0/dnb.txt"));
    assert_eq!(df.column("path")?.str()?.get(1), Some("1/zbw.txt"));
    assert_eq!(df.height(), 2);

    Ok(())
}

#[test]
fn grep_allow_list() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("grep.ipc");

    // PATH
    let allow = datashed_dir.child("tmp/ALLOW.csv");
    allow.write_str("path\n0/dnb.txt")?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["grep", "-q", "-A", allow.to_str().unwrap()])
        .args(["\\(DNB\\)", "Econ(Stor|Biz)"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.column("path")?.str()?.get(0), Some("0/dnb.txt"));
    assert_eq!(df.height(), 1);

    // PATH + HASH
    let allow = datashed_dir.child("tmp/ALLOW.csv");
    allow.write_str("path,hash\n0/dnb.txt,1fbf52b4")?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["grep", "-q", "-A", allow.to_str().unwrap()])
        .args(["\\(DNB\\)", "Econ(Stor|Biz)"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.column("path")?.str()?.get(0), Some("0/dnb.txt"));
    assert_eq!(df.height(), 1);

    // PATH + HASH (incorrect)
    let allow = datashed_dir.child("tmp/ALLOW.csv");
    allow.write_str("path,hash\n0/dnb.txt,XXXX")?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["grep", "-q", "-A", allow.to_str().unwrap()])
        .args(["\\(DNB\\)", "Econ(Stor|Biz)"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(HEADER))
        .stderr(predicates::str::is_empty());

    // PPN
    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["index", "-q", "--filename-column", "ppn"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let allow = datashed_dir.child("tmp/ALLOW.csv");
    allow.write_str("ppn\ndnb")?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["grep", "-q", "-A", allow.to_str().unwrap()])
        .args(["\\(DNB\\)", "Econ(Stor|Biz)"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.column("path")?.str()?.get(0), Some("0/dnb.txt"));
    assert_eq!(df.height(), 1);

    Ok(())
}

#[test]
fn grep_deny_list() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("grep.ipc");

    // PATH
    let deny = datashed_dir.child("tmp/DENY.csv");
    deny.write_str("path\n0/dnb.txt")?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["grep", "-q", "-D", deny.to_str().unwrap()])
        .args(["\\(DNB\\)", "Econ(Stor|Biz)"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.column("path")?.str()?.get(0), Some("1/zbw.txt"));
    assert_eq!(df.height(), 1);

    // PATH + HASH
    let deny = datashed_dir.child("tmp/DENY.csv");
    deny.write_str("path,hash\n0/dnb.txt,1fbf52b4")?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["grep", "-q", "-D", deny.to_str().unwrap()])
        .args(["\\(DNB\\)", "Econ(Stor|Biz)"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.column("path")?.str()?.get(0), Some("1/zbw.txt"));
    assert_eq!(df.height(), 1);

    // PATH + HASH (incorrect)
    let deny = datashed_dir.child("tmp/DENY.csv");
    deny.write_str("path,hash\n0/dnb.txt,XXXX")?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["grep", "-q", "-D", deny.to_str().unwrap()])
        .args(["\\(DNB\\)", "Econ(Stor|Biz)"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.column("path")?.str()?.get(0), Some("0/dnb.txt"));
    assert_eq!(df.column("path")?.str()?.get(1), Some("1/zbw.txt"));
    assert_eq!(df.height(), 2);

    // PPN
    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["index", "-q", "--filename-column", "ppn"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let deny = datashed_dir.child("tmp/DENY.csv");
    deny.write_str("ppn\ndnb")?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["grep", "-q", "-D", deny.to_str().unwrap()])
        .args(["\\(DNB\\)", "Econ(Stor|Biz)"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.column("path")?.str()?.get(0), Some("1/zbw.txt"));
    assert_eq!(df.height(), 1);

    Ok(())
}
#[test]
fn grep_index() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("grep.ipc");

    let mut tmp_index =
        IpcReader::new(File::open(&datashed_dir.join("index.ipc"))?)
            .finish()?
            .lazy()
            .filter(col("path").str().starts_with(lit("0/")))
            .collect()?;

    IpcWriter::new(File::create(
        datashed_dir.join("tmp").join("index.ipc"),
    )?)
    .with_compression(Some(IpcCompression::ZSTD))
    .finish(&mut tmp_index)?;

    let index = datashed_dir.child("tmp/index.ipc");

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["grep", "-q", "-I", index.to_str().unwrap()])
        .args(["\\(DNB\\)", "Econ(Stor|Biz)"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.column("path")?.str()?.get(0), Some("0/dnb.txt"));
    assert_eq!(df.height(), 1);

    Ok(())
}

#[test]
fn grep_where() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("grep.ipc");

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["grep", "-q", "\\(DNB\\)", "Econ(Stor|Biz)"])
        .args(["--where", "path IN ('0/dnb.txt', '0/tib.txt')"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(output)?).finish()?;
    assert_eq!(df.column("path")?.str()?.get(0), Some("0/dnb.txt"));
    assert_eq!(df.height(), 1);

    Ok(())
}

#[test]
fn grep_translit() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("grep.ipc");

    // nfc pattern
    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["grep", "-q", "erfüllt"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(HEADER))
        .stderr(predicates::str::is_empty());

    // nfd pattern
    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["grep", "-q", "erfu\u{308}llt"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.column("path")?.str()?.get(0), Some("0/dnb.txt"));
    assert_eq!(df.height(), 1);

    // nfc pattern (nfd normalization)
    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["config", "--set", "runtime.normalization", "nfd"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["grep", "-q", "erfüllt"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(output)?).finish()?;
    assert_eq!(df.column("path")?.str()?.get(0), Some("0/dnb.txt"));
    assert_eq!(df.height(), 1);

    Ok(())
}
