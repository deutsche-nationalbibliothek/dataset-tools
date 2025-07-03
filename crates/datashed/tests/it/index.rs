use std::fs::File;
use std::path::Path;
use std::time::UNIX_EPOCH;

use approx::assert_abs_diff_eq;
use polars::io::SerReader;
use polars::prelude::*;

use crate::prelude::*;

fn check_index<P>(path: P, filename_column: bool) -> TestResult
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let path_str = path.to_str();

    let data_dir = path.parent().unwrap().join("data");
    let mut is_arrow = true;

    let df = match path_str {
        Some(path_str) if path_str.ends_with(".ipc") => {
            IpcReader::new(File::open(path)?).finish()?
        }
        Some(path_str) if path_str.ends_with(".csv") => {
            is_arrow = false;
            CsvReader::new(File::open(path)?).finish()?
        }
        _ => unreachable!(),
    };

    let df = df.sort(["path"], SortMultipleOptions::default())?;
    assert_eq!(df.height(), 3);
    // eprintln!("{df:?}");

    let columns = df.take_columns();

    let mut idx = 0;

    // PATH
    let paths: Vec<_> = columns[idx].str()?.iter().collect();
    idx += 1;

    assert_eq!(paths[0], Some("0/dnb.txt"));
    assert_eq!(paths[1], Some("0/tib.txt"));
    assert_eq!(paths[2], Some("1/zbw.txt"));

    // HASH
    let hashes: Vec<_> = columns[idx].str()?.iter().collect();
    idx += 1;

    assert_eq!(hashes[0], Some("1fbf52b4febc"));
    assert_eq!(hashes[1], Some("809239e5941a"));
    assert_eq!(hashes[2], Some("a50f7e557482"));

    // FILENAME
    if filename_column {
        let filenames: Vec<_> = columns[idx].str()?.iter().collect();
        idx += 1;

        assert_eq!(filenames[0], Some("dnb"));
        assert_eq!(filenames[1], Some("tib"));
        assert_eq!(filenames[2], Some("zbw"));
    }

    // SIZE
    let sizes: Vec<_> = columns[idx]
        .cast(&DataType::UInt64)?
        .u64()?
        .iter()
        .collect();
    idx += 1;

    assert_eq!(sizes[0], Some(776));
    assert_eq!(sizes[1], Some(1453));
    assert_eq!(sizes[2], Some(909));

    // LANG CODES
    if is_arrow {
        let lang = columns[idx].cast(&DataType::String)?;
        let lang_codes: Vec<_> = lang.str()?.iter().collect();
        idx += 1;

        assert_eq!(lang_codes[0], Some("{\"ger\",1.0}"));
        assert_eq!(lang_codes[1], Some("{\"ger\",1.0}"));
        assert_eq!(lang_codes[2], Some("{\"eng\",1.0}"));
    } else {
        let lang_codes = columns[idx].cast(&DataType::String)?;
        let lang_codes: Vec<_> = lang_codes.str()?.iter().collect();
        idx += 1;

        assert_eq!(lang_codes[0], Some("ger"));
        assert_eq!(lang_codes[1], Some("ger"));
        assert_eq!(lang_codes[2], Some("eng"));

        // LANG SCORES
        let lang_scores: Vec<_> = columns[idx]
            .cast(&DataType::Float64)?
            .f64()?
            .iter()
            .collect();
        idx += 1;

        assert_abs_diff_eq!(
            lang_scores[0].unwrap(),
            1.0,
            epsilon = 1e-4
        );
        assert_abs_diff_eq!(
            lang_scores[1].unwrap(),
            1.0,
            epsilon = 1e-4
        );
        assert_abs_diff_eq!(
            lang_scores[2].unwrap(),
            1.0,
            epsilon = 1e-4
        );
    }

    // ALPHA
    let alphas: Vec<_> = columns[idx]
        .cast(&DataType::Float64)?
        .f64()?
        .iter()
        .collect();
    idx += 1;

    assert_abs_diff_eq!(alphas[0].unwrap(), 0.82529336, epsilon = 1e-4);
    assert_abs_diff_eq!(alphas[1].unwrap(), 0.83240222, epsilon = 1e-4);
    assert_abs_diff_eq!(alphas[2].unwrap(), 0.82079648, epsilon = 1e-4);

    // MTIME
    let mtimes: Vec<_> = columns[idx]
        .cast(&DataType::UInt64)?
        .u64()?
        .iter()
        .collect();

    assert_eq!(
        mtimes[0],
        data_dir
            .join(paths[0].unwrap())
            .metadata()?
            .modified()
            .ok()
            .and_then(|x| x.duration_since(UNIX_EPOCH).ok())
            .map(|x| x.as_secs())
    );

    assert_eq!(
        mtimes[1],
        data_dir
            .join(paths[1].unwrap())
            .metadata()?
            .modified()
            .ok()
            .and_then(|x| x.duration_since(UNIX_EPOCH).ok())
            .map(|x| x.as_secs())
    );

    assert_eq!(
        mtimes[2],
        data_dir
            .join(paths[2].unwrap())
            .metadata()?
            .modified()
            .ok()
            .and_then(|x| x.duration_since(UNIX_EPOCH).ok())
            .map(|x| x.as_secs())
    );

    Ok(())
}

#[test]
fn index_default() -> TestResult {
    let mut cmd = Command::cargo_bin("datashed")?;
    let datashed_dir = create_datashed()?;

    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["index", "-q"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    check_index(datashed_dir.join("index.ipc"), false)?;
    datashed_dir.close()?;

    Ok(())
}

#[test]
fn index_filename_column() -> TestResult {
    let mut cmd = Command::cargo_bin("datashed")?;
    let datashed_dir = create_datashed()?;

    let assert = cmd
        .current_dir(&datashed_dir)
        .args(["index", "-q", "--filename-column", "ppn"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    check_index(datashed_dir.join("index.ipc"), true)?;
    datashed_dir.close()?;

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

    check_index(datashed_dir.join("index.csv"), false)?;
    datashed_dir.close()?;

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

    check_index(datashed_dir.join("index.ipc"), false)?;
    datashed_dir.close()?;

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

    check_index(datashed_dir.join("index.ipc"), false)?;
    datashed_dir.close()?;

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

    check_index(datashed_dir.join("index.ipc"), false)?;
    datashed_dir.close()?;

    Ok(())
}
