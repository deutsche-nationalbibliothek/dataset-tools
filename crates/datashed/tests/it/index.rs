use std::fs::File;
use std::path::Path;

use polars::io::SerReader;
use polars::prelude::*;

use crate::prelude::*;

fn check_index<P: AsRef<Path>>(path: P) -> TestResult {
    let path_str = path.as_ref().to_str();

    let df = match path_str {
        Some(path_str) if path_str.ends_with(".ipc") => {
            IpcReader::new(File::open(path)?).finish()?
        }
        Some(path_str) if path_str.ends_with(".csv") => {
            CsvReader::new(File::open(path)?).finish()?
        }
        _ => unreachable!(),
    };

    let df = df.sort(["path"], SortMultipleOptions::default())?;
    assert_eq!(df.height(), 3);

    let columns = df.take_columns();
    let paths: Vec<_> = columns[0].str()?.iter().collect();
    let sizes: Vec<_> =
        columns[1].cast(&DataType::UInt64)?.u64()?.iter().collect();

    // DNB
    assert_eq!(paths[0], Some("0/dnb.txt"));
    assert_eq!(sizes[0], Some(769));

    // TIB
    assert_eq!(paths[1], Some("0/tib.txt"));
    assert_eq!(sizes[1], Some(1443));

    // ZBW
    assert_eq!(paths[2], Some("1/zbw.txt"));
    assert_eq!(sizes[2], Some(908));

    Ok(())
}

#[test]
fn index_default() -> TestResult {
    let mut cmd = Command::cargo_bin("datashed")?;
    let datashed_dir = create_datashed()?;

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

    check_index(datashed_dir.join("index.ipc"))?;

    Ok(())
}

#[test]
fn index_output_csv() -> TestResult {
    let mut cmd = Command::cargo_bin("datashed")?;
    let datashed_dir = create_datashed()?;

    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["index", "-q"])
        .args(["-o", datashed_dir.join("index.csv").to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    check_index(datashed_dir.join("index.csv"))?;

    Ok(())
}

#[test]
fn index_output_ipc() -> TestResult {
    let mut cmd = Command::cargo_bin("datashed")?;
    let datashed_dir = create_datashed()?;

    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["index", "-q"])
        .args(["-o", datashed_dir.join("index.ipc").to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    check_index(datashed_dir.join("index.ipc"))?;

    Ok(())
}

#[test]
fn index_num_threads_1() -> TestResult {
    let mut cmd = Command::cargo_bin("datashed")?;
    let datashed_dir = create_datashed()?;

    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["-j", "1", "index", "-q"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    check_index(datashed_dir.join("index.ipc"))?;

    Ok(())
}

#[test]
fn index_num_threads_2() -> TestResult {
    let mut cmd = Command::cargo_bin("datashed")?;
    let datashed_dir = create_datashed()?;

    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["-j", "1", "index", "-q"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    check_index(datashed_dir.join("index.ipc"))?;

    Ok(())
}
