use std::fs::{File, read_to_string};

use flate2::read::GzDecoder;
use predicates::path::is_file;
use tar::Archive;

use crate::prelude::*;

#[test]
fn archive_default() -> TestResult {
    let datashed_dir = create_datashed()?;
    let dest_dir = TempDir::new()?;
    let temp_dir = TempDir::new()?;

    let archive = temp_dir.join("archive.tar.gz");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .arg("index")
        .assert();

    assert.success();

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

    let reader = GzDecoder::new(File::open(archive)?);
    let mut archive = Archive::new(reader);
    archive.unpack(&dest_dir)?;

    let expected = read_to_string(datashed_dir.join("data/0/dnb.txt"))?;
    let actual = read_to_string(dest_dir.join("data/0/dnb.txt"))?;
    assert_eq!(expected, actual);

    let expected = read_to_string(datashed_dir.join("data/0/tib.txt"))?;
    let actual = read_to_string(dest_dir.join("data/0/tib.txt"))?;
    assert_eq!(expected, actual);

    let expected = read_to_string(datashed_dir.join("data/1/zbw.txt"))?;
    let actual = read_to_string(dest_dir.join("data/1/zbw.txt"))?;
    assert_eq!(expected, actual);

    let expected = read_to_string(datashed_dir.join(Datashed::CONFIG))?;
    let actual = read_to_string(dest_dir.join(Datashed::CONFIG))?;
    assert_eq!(expected, actual);

    assert!(is_file().eval(&dest_dir.join(Datashed::INDEX)));

    Ok(())
}

#[test]
fn archive_fast() -> TestResult {
    let datashed_dir = create_datashed()?;
    let temp_dir = TempDir::new()?;
    let dest_dir = TempDir::new()?;

    let archive = temp_dir.join("archive.tar.gz");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .arg("index")
        .assert();

    assert.success();

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["archive", "-q", "--fast"])
        .args(["-o", archive.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let reader = GzDecoder::new(File::open(archive)?);
    let mut archive = Archive::new(reader);
    archive.unpack(&dest_dir)?;

    let expected = read_to_string(datashed_dir.join("data/0/dnb.txt"))?;
    let actual = read_to_string(dest_dir.join("data/0/dnb.txt"))?;
    assert_eq!(expected, actual);

    let expected = read_to_string(datashed_dir.join("data/0/tib.txt"))?;
    let actual = read_to_string(dest_dir.join("data/0/tib.txt"))?;
    assert_eq!(expected, actual);

    let expected = read_to_string(datashed_dir.join("data/1/zbw.txt"))?;
    let actual = read_to_string(dest_dir.join("data/1/zbw.txt"))?;
    assert_eq!(expected, actual);

    let expected = read_to_string(datashed_dir.join(Datashed::CONFIG))?;
    let actual = read_to_string(dest_dir.join(Datashed::CONFIG))?;
    assert_eq!(expected, actual);

    assert!(is_file().eval(&dest_dir.join(Datashed::INDEX)));

    Ok(())
}

#[test]
fn archive_best() -> TestResult {
    let datashed_dir = create_datashed()?;
    let temp_dir = TempDir::new()?;
    let dest_dir = TempDir::new()?;
    let archive = temp_dir.join("archive.tar.gz");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .arg("index")
        .assert();

    assert.success();

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["archive", "-q", "--best"])
        .args(["-o", archive.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let reader = GzDecoder::new(File::open(archive)?);
    let mut archive = Archive::new(reader);
    archive.unpack(&dest_dir)?;

    let expected = read_to_string(datashed_dir.join("data/0/dnb.txt"))?;
    let actual = read_to_string(dest_dir.join("data/0/dnb.txt"))?;
    assert_eq!(expected, actual);

    let expected = read_to_string(datashed_dir.join("data/0/tib.txt"))?;
    let actual = read_to_string(dest_dir.join("data/0/tib.txt"))?;
    assert_eq!(expected, actual);

    let expected = read_to_string(datashed_dir.join("data/1/zbw.txt"))?;
    let actual = read_to_string(dest_dir.join("data/1/zbw.txt"))?;
    assert_eq!(expected, actual);

    let expected = read_to_string(datashed_dir.join(Datashed::CONFIG))?;
    let actual = read_to_string(dest_dir.join(Datashed::CONFIG))?;
    assert_eq!(expected, actual);

    assert!(is_file().eval(&dest_dir.join(Datashed::INDEX)));

    Ok(())
}
