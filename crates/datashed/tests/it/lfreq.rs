use std::fs::File;

use assert_fs::prelude::{FileWriteStr, PathChild};

use crate::prelude::*;

#[test]
fn lfreq_default() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["lfreq", "-q"])
        .assert();
    assert
        .success()
        .code(0)
        .stdout(predicates::str::contains("path,hash,total,a,b,c"))
        .stdout(predicates::str::contains("0/dnb.txt,1fbf52b4,633,38"))
        .stdout(predicates::str::contains("0/tib.txt,809239e5,1192,56"))
        .stdout(predicates::str::contains("1/zbw.txt,a50f7e55,742,59"))
        .stderr(predicates::str::is_empty());

    datashed_dir.close()?;
    Ok(())
}

#[test]
fn lfreq_output_csv() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.child("lfreq.csv");

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["lfreq", "-q", "-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(output.to_path_buf()))?
        .finish()?;
    assert_eq!(df.height(), 3);

    let totals = df.column("total")?.i64().unwrap();
    assert_eq!(totals.get(0), Some(633));

    let r#as = df.column("a")?.i64().unwrap();
    assert_eq!(r#as.get(0), Some(38));

    datashed_dir.close()?;
    Ok(())
}

#[test]
fn lfreq_output_ipc() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.child("lfreq.ipc");

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["lfreq", "-q", "-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(output)?).finish()?;
    assert_eq!(df.height(), 3);

    let totals = df.column("total")?.u64().unwrap();
    assert_eq!(totals.get(0), Some(633));

    let r#as = df.column("a")?.u64().unwrap();
    assert_eq!(r#as.get(0), Some(38));

    datashed_dir.close()?;
    Ok(())
}

#[test]
fn lfreq_alphabet() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["lfreq", "-q", "--alphabet", "abü"])
        .assert();
    assert
        .success()
        .code(0)
        .stdout(predicates::str::contains("path,hash,total,a,b,ü"))
        .stdout(predicates::str::contains("1fbf52b4,68,38,26,4"))
        .stdout(predicates::str::contains("809239e5,94,56,35,3"))
        .stdout(predicates::str::contains("a50f7e55,79,59,19,1"))
        .stderr(predicates::str::is_empty());

    // empty alphabet
    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["lfreq", "-q", "--alphabet", ""])
        .assert();
    assert
        .success()
        .code(0)
        .stdout(predicates::str::contains("path,hash,total"))
        .stdout(predicates::str::contains("0/dnb.txt,1fbf52b4,0"))
        .stdout(predicates::str::contains("0/tib.txt,809239e5,0"))
        .stdout(predicates::str::contains("1/zbw.txt,a50f7e55,0"))
        .stderr(predicates::str::is_empty());

    datashed_dir.close()?;
    Ok(())
}

#[test]
fn lfreq_index() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;

    let mut tmp_index =
        IpcReader::new(File::open(&datashed_dir.join("index.ipc"))?)
            .finish()?
            .lazy()
            .filter(col("path").str().starts_with(lit("1/")))
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
        .args(["lfreq", "-q", "--alphabet", "ab"])
        .args(["-I", index.to_str().unwrap()])
        .assert();
    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(
            "path,hash,total,a,b\n1/zbw.txt,a50f7e55,78,59,19\n",
        ))
        .stderr(predicates::str::is_empty());

    datashed_dir.close()?;
    Ok(())
}

#[test]
fn lfreq_allow_list() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;

    let allow = datashed_dir.child("tmp/ALLOW.csv");
    allow.write_str("path\n0/dnb.txt")?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["lfreq", "-q", "--alphabet", "ab"])
        .args(["-A", allow.to_str().unwrap()])
        .assert();
    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(
            "path,hash,total,a,b\n0/dnb.txt,1fbf52b4,64,38,26\n",
        ))
        .stderr(predicates::str::is_empty());

    datashed_dir.close()?;
    Ok(())
}

#[test]
fn lfreq_deny_list() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;

    let deny = datashed_dir.child("tmp/DENY.csv");
    deny.write_str("path\n0/tib.txt\n1/zbw.txt\n")?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["lfreq", "-q", "--alphabet", "ab"])
        .args(["-D", deny.to_str().unwrap()])
        .assert();
    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(
            "path,hash,total,a,b\n0/dnb.txt,1fbf52b4,64,38,26\n",
        ))
        .stderr(predicates::str::is_empty());

    datashed_dir.close()?;
    Ok(())
}

#[test]
fn lfreq_where() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["lfreq", "-q", "--alphabet", "ab"])
        .args(["--where", "path == '0/dnb.txt'"])
        .assert();
    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(
            "path,hash,total,a,b\n0/dnb.txt,1fbf52b4,64,38,26\n",
        ))
        .stderr(predicates::str::is_empty());

    datashed_dir.close()?;
    Ok(())
}
