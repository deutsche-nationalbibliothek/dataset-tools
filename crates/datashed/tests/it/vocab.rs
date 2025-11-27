use std::fs::File;

use assert_fs::prelude::{FileWriteStr, PathChild};

use crate::prelude::*;

#[test]
fn vocab_default() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("vocab.ipc");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["vocab", "-q", "-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.height(), 232);

    let terms = df.column("term")?.str()?;
    let tfs = df.column("tf")?.u64()?;
    let dfs = df.column("df")?.u64()?;

    assert_eq!(terms.get(0).unwrap(), "die");
    assert_eq!(tfs.get(0).unwrap(), 16);
    assert_eq!(dfs.get(0).unwrap(), 2);

    assert_eq!(terms.get(1).unwrap(), "und");
    assert_eq!(tfs.get(1).unwrap(), 16);
    assert_eq!(dfs.get(1).unwrap(), 2);

    assert_eq!(terms.get(2).unwrap(), "deutsche");
    assert_eq!(tfs.get(2).unwrap(), 8);
    assert_eq!(dfs.get(2).unwrap(), 3);

    Ok(())
}

#[test]
fn vocab_bigrams() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("vocab.ipc");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["vocab", "-q", "--bigrams"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.height(), 358);

    let terms = df.column("term")?.str()?;
    let tfs = df.column("tf")?.u64()?;
    let dfs = df.column("df")?.u64()?;

    assert_eq!(terms.get(0).unwrap(), "die tib");
    assert_eq!(tfs.get(0).unwrap(), 5);
    assert_eq!(dfs.get(0).unwrap(), 1);

    assert_eq!(terms.get(1).unwrap(), "die deutsche");
    assert_eq!(tfs.get(1).unwrap(), 4);
    assert_eq!(dfs.get(1).unwrap(), 2);

    Ok(())
}

#[test]
fn vocab_trigrams() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("vocab.ipc");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["vocab", "-q", "--trigrams"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.height(), 376);

    let terms = df.column("term")?.str()?;
    let tfs = df.column("tf")?.u64()?;
    let dfs = df.column("df")?.u64()?;

    assert_eq!(
        terms.get(0).unwrap(),
        "leibniz informationszentrum wirtschaft"
    );
    assert_eq!(tfs.get(0).unwrap(), 2);
    assert_eq!(dfs.get(0).unwrap(), 2);

    assert_eq!(
        terms.get(1).unwrap(),
        "zbw leibniz informationszentrum"
    );
    assert_eq!(tfs.get(1).unwrap(), 2);
    assert_eq!(dfs.get(1).unwrap(), 2);

    Ok(())
}

#[test]
fn vocab_stopwords() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("vocab.ipc");

    let stopwords = datashed_dir.child("STOPWORDS.txt");
    stopwords.write_str("DIE\nund\n")?;

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["vocab", "-q"])
        .args(["-S", stopwords.to_str().unwrap()])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.height(), 230);

    let terms = df.column("term")?.str()?;
    let tfs = df.column("tf")?.u64()?;
    let dfs = df.column("df")?.u64()?;

    assert_eq!(terms.get(0).unwrap(), "deutsche");
    assert_eq!(tfs.get(0).unwrap(), 8);
    assert_eq!(dfs.get(0).unwrap(), 3);

    assert_eq!(terms.get(1).unwrap(), "in");
    assert_eq!(tfs.get(1).unwrap(), 8);
    assert_eq!(dfs.get(1).unwrap(), 3);

    assert_eq!(terms.get(2).unwrap(), "the");
    assert_eq!(tfs.get(2).unwrap(), 8);
    assert_eq!(dfs.get(2).unwrap(), 1);

    Ok(())
}

#[test]
fn vocab_ucs_category_letter() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("vocab.ipc");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["vocab", "-q", "-La"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.height(), 227);

    let terms = df.column("term")?.str()?;
    let tfs = df.column("tf")?.u64()?;
    let dfs = df.column("df")?.u64()?;

    assert_eq!(terms.get(0).unwrap(), "die");
    assert_eq!(tfs.get(0).unwrap(), 16);
    assert_eq!(dfs.get(0).unwrap(), 2);

    assert_eq!(terms.get(1).unwrap(), "und");
    assert_eq!(tfs.get(1).unwrap(), 16);
    assert_eq!(dfs.get(1).unwrap(), 2);

    Ok(())
}

#[test]
fn vocab_ucs_category_lower() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("vocab.ipc");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["vocab", "-q", "-Ll"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.height(), 220);

    let terms = df.column("term")?.str()?;
    let tfs = df.column("tf")?.u64()?;
    let dfs = df.column("df")?.u64()?;

    assert_eq!(terms.get(0).unwrap(), "die");
    assert_eq!(tfs.get(0).unwrap(), 16);
    assert_eq!(dfs.get(0).unwrap(), 2);

    assert_eq!(terms.get(1).unwrap(), "und");
    assert_eq!(tfs.get(1).unwrap(), 16);
    assert_eq!(dfs.get(1).unwrap(), 2);

    Ok(())
}

#[test]
fn vocab_ucs_category_upper() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("vocab.ipc");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["vocab", "-q", "-Lu"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.height(), 115);

    let terms = df.column("term")?.str()?;
    let tfs = df.column("tf")?.u64()?;
    let dfs = df.column("df")?.u64()?;

    assert_eq!(terms.get(0).unwrap(), "deutsche");
    assert_eq!(tfs.get(0).unwrap(), 8);
    assert_eq!(dfs.get(0).unwrap(), 3);

    assert_eq!(terms.get(1).unwrap(), "leibniz");
    assert_eq!(tfs.get(1).unwrap(), 7);
    assert_eq!(dfs.get(1).unwrap(), 2);

    Ok(())
}

#[test]
fn vocab_ucs_category_title() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("vocab.ipc");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["vocab", "-q", "-Lt"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.height(), 0);

    Ok(())
}

#[test]
fn vocab_ucs_category_modifier() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("vocab.ipc");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["vocab", "-q", "-Lm"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.height(), 0);

    Ok(())
}

#[test]
fn vocab_ucs_category_other() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("vocab.ipc");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["vocab", "-q", "-Lo"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.height(), 0);

    Ok(())
}

#[test]
fn vocab_min_term_length() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("vocab.ipc");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["vocab", "-q", "--min-term-length", "7"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.height(), 133);

    let terms = df.column("term")?.str()?;
    let tfs = df.column("tf")?.u64()?;
    let dfs = df.column("df")?.u64()?;

    assert_eq!(terms.get(0).unwrap(), "deutsche");
    assert_eq!(tfs.get(0).unwrap(), 8);
    assert_eq!(dfs.get(0).unwrap(), 3);

    assert_eq!(terms.get(1).unwrap(), "leibniz");
    assert_eq!(tfs.get(1).unwrap(), 7);
    assert_eq!(dfs.get(1).unwrap(), 2);

    Ok(())
}

#[test]
fn vocab_min_term_freq() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("vocab.ipc");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["vocab", "-q", "--min-term-freq", "7"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.height(), 10);

    let terms = df.column("term")?.str()?;
    let tfs = df.column("tf")?.u64()?;
    let dfs = df.column("df")?.u64()?;

    assert_eq!(terms.get(0).unwrap(), "die");
    assert_eq!(tfs.get(0).unwrap(), 16);
    assert_eq!(dfs.get(0).unwrap(), 2);

    assert_eq!(terms.get(1).unwrap(), "und");
    assert_eq!(tfs.get(1).unwrap(), 16);
    assert_eq!(dfs.get(1).unwrap(), 2);

    Ok(())
}

#[test]
fn vocab_min_doc_freq() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("vocab.ipc");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["vocab", "-q", "--min-doc-freq", "3"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.height(), 3);

    let terms = df.column("term")?.str()?;
    let tfs = df.column("tf")?.u64()?;
    let dfs = df.column("df")?.u64()?;

    assert_eq!(terms.get(0).unwrap(), "deutsche");
    assert_eq!(tfs.get(0).unwrap(), 8);
    assert_eq!(dfs.get(0).unwrap(), 3);

    assert_eq!(terms.get(1).unwrap(), "in");
    assert_eq!(tfs.get(1).unwrap(), 8);
    assert_eq!(dfs.get(1).unwrap(), 3);

    assert_eq!(terms.get(2).unwrap(), "fu\u{308}r");
    assert_eq!(tfs.get(2).unwrap(), 4);
    assert_eq!(dfs.get(2).unwrap(), 3);

    Ok(())
}

#[test]
fn vocab_limit() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;
    let output = datashed_dir.join("tmp").join("vocab.ipc");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["vocab", "-q", "--limit", "2"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let df = IpcReader::new(File::open(&output)?).finish()?;
    assert_eq!(df.height(), 2);

    let terms = df.column("term")?.str()?;
    let tfs = df.column("tf")?.u64()?;
    let dfs = df.column("df")?.u64()?;

    assert_eq!(terms.get(0).unwrap(), "die");
    assert_eq!(tfs.get(0).unwrap(), 16);
    assert_eq!(dfs.get(0).unwrap(), 2);

    assert_eq!(terms.get(1).unwrap(), "und");
    assert_eq!(tfs.get(1).unwrap(), 16);
    assert_eq!(dfs.get(1).unwrap(), 2);

    Ok(())
}

#[test]
fn vocab_allow_list() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;

    let allow = datashed_dir.child("tmp/ALLOW.csv");
    allow.write_str("path\n0/dnb.txt")?;

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["vocab", "-q", "--limit", "1"])
        .args(["-A", allow.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("term,tf,df\ndie,7,1\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn vocab_deny_list() -> TestResult {
    let datashed_dir = create_datashed_with_index()?;

    let deny = datashed_dir.child("tmp/DENY.csv");
    deny.write_str("path\n0/tib.txt\n1/zbw.txt")?;

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["vocab", "-q", "--limit", "1"])
        .args(["-D", deny.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("term,tf,df\ndie,7,1\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn vocab_index() -> TestResult {
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
    .with_compression(Some(IpcCompression::ZSTD))
    .finish(&mut tmp_index)?;

    let index = datashed_dir.child("tmp/index.ipc");

    let assert = datashed_cmd()
        .current_dir(&datashed_dir)
        .args(["vocab", "-q", "--limit", "1"])
        .args(["-I", index.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("term,tf,df\ndie,16,2\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}
