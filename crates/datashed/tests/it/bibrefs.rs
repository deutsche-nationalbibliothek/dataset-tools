use std::fs::{self, File};

use crate::prelude::*;

const DNB_ISBN: &str = "Deutsche Nationalbibliothek (Hrsg.): \
    Deutsche Nationalbibliothek: Bewahren für die Zukunft. \
    Vlg. Deutsche Nationalbibliothek, \
    Leipzig/Frankfurt am Main/Berlin 2008, \
    ISBN 978-3-933641-89-2 ...";

#[test]
fn bibrefs_default() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["bibrefs", "-q"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("path,hash,kind,value,start,end\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn bibrefs_output_csv() -> TestResult {
    let datashed_dir = create_datashed()?;
    let output = datashed_dir.join("tmp").join("bibrefs.csv");

    fs::write(datashed_dir.join("data").join("0/dnb.txt"), DNB_ISBN)?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["index", "-q"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["bibrefs", "-q", "-o", output.to_str().unwrap()])
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
    assert_eq!(df.height(), 1);

    let paths: Vec<_> = df.column("path")?.str()?.iter().collect();
    assert_eq!(paths[0], Some("0/dnb.txt"));

    let values: Vec<_> = df.column("value")?.str()?.iter().collect();
    assert_eq!(values[0], Some("978-3-933641-89-2"));

    let starts: Vec<_> = df
        .column("start")?
        .cast(&DataType::UInt64)?
        .u64()?
        .iter()
        .collect();
    assert_eq!(starts[0], Some(166));

    let ends: Vec<_> = df
        .column("end")?
        .cast(&DataType::UInt64)?
        .u64()?
        .iter()
        .collect();
    assert_eq!(ends[0], Some(188));

    Ok(())
}

#[test]
fn bibrefs_output_ipc() -> TestResult {
    let datashed_dir = create_datashed()?;
    let output = datashed_dir.join("tmp").join("bibrefs.ipc");

    fs::write(datashed_dir.join("data").join("0/dnb.txt"), DNB_ISBN)?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["index", "-q"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["bibrefs", "-q", "-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(output)?)
        .finish()?
        .unnest(["span"])?;

    assert_eq!(df.height(), 1);

    let paths: Vec<_> = df.column("path")?.str()?.iter().collect();
    assert_eq!(paths[0], Some("0/dnb.txt"));

    let values: Vec<_> = df.column("value")?.str()?.iter().collect();
    assert_eq!(values[0], Some("978-3-933641-89-2"));

    let starts: Vec<_> = df
        .column("start")?
        .cast(&DataType::UInt64)?
        .u64()?
        .iter()
        .collect();
    assert_eq!(starts[0], Some(166));

    let ends: Vec<_> = df
        .column("end")?
        .cast(&DataType::UInt64)?
        .u64()?
        .iter()
        .collect();
    assert_eq!(ends[0], Some(188));

    Ok(())
}

#[test]
fn bibrefs_isbn() -> TestResult {
    let datashed_dir = create_datashed()?;

    fs::write(datashed_dir.join("data").join("0/dnb.txt"), DNB_ISBN)?;

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["index", "-q"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut cmd = Command::cargo_bin("datashed")?;
    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["bibrefs", "-q"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(
            "path,hash,kind,value,start,end\n\
            0/dnb.txt,2bd6ab7dfafa,isbn,978-3-933641-89-2,166,188\n",
        ))
        .stderr(predicates::str::is_empty());

    Ok(())
}
