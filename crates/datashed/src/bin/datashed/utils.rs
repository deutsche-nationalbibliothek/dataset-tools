use std::ffi::OsStr;
use std::fs::File;
use std::io::stdout;
use std::path::{Path, PathBuf};

use datashed::DatashedResult;
use polars::prelude::*;

use crate::prelude::bail;

pub(crate) fn read_df<P: AsRef<Path>>(
    path: P,
) -> DatashedResult<DataFrame> {
    let path = path.as_ref().to_path_buf();

    Ok(match path.extension().and_then(OsStr::to_str) {
        Some("ipc") => IpcReader::new(File::open(path)?)
            .memory_mapped(None)
            .finish()?,
        _ => CsvReadOptions::default()
            .with_has_header(true)
            .with_infer_schema_length(Some(0))
            .try_into_reader_with_file_path(Some(path))?
            .finish()?,
    })
}

pub(crate) fn write_df(
    df: &mut DataFrame,
    path: Option<PathBuf>,
) -> DatashedResult<()> {
    if let Some(path) = path {
        match path.extension().and_then(OsStr::to_str) {
            Some("csv") => {
                let mut writer = CsvWriter::new(File::create(path)?);
                writer.finish(df)?;
            }
            _ => {
                let mut writer = IpcWriter::new(File::create(path)?)
                    .with_compression(Some(IpcCompression::ZSTD));
                writer.finish(df)?;
            }
        }
    } else {
        let mut writer = CsvWriter::new(stdout().lock());
        writer.finish(df)?;
    }
    Ok(())
}

pub(crate) fn apply_allow_list<P: AsRef<Path>>(
    df: LazyFrame,
    allow: Option<P>,
) -> DatashedResult<LazyFrame> {
    let Some(path) = allow else { return Ok(df) };
    let allow_list = read_df(path)?;

    let df = if allow_list.column("path").is_ok() {
        if allow_list.column("hash").is_ok() {
            df.join(
                allow_list.lazy(),
                [col("path"), col("hash")],
                [col("path"), col("hash")],
                JoinArgs::new(JoinType::Semi),
            )
        } else {
            df.semi_join(allow_list.lazy(), col("path"), col("path"))
        }
    } else if allow_list.column("ppn").is_ok() {
        df.semi_join(allow_list.lazy(), col("ppn"), col("ppn"))
    } else {
        bail!("missing `path` or `ppn` column.")
    };

    Ok(df)
}

pub(crate) fn apply_deny_list<P: AsRef<Path>>(
    df: LazyFrame,
    deny: Option<P>,
) -> DatashedResult<LazyFrame> {
    let Some(path) = deny else { return Ok(df) };
    let deny_list = read_df(path)?;

    let df = if deny_list.column("path").is_ok() {
        if deny_list.column("hash").is_ok() {
            df.join(
                deny_list.lazy(),
                [col("path"), col("hash")],
                [col("path"), col("hash")],
                JoinArgs::new(JoinType::Anti),
            )
        } else {
            df.anti_join(deny_list.lazy(), col("path"), col("path"))
        }
    } else if deny_list.column("ppn").is_ok() {
        df.anti_join(deny_list.lazy(), col("ppn"), col("ppn"))
    } else {
        bail!("missing `path` or `ppn` column.")
    };

    Ok(df)
}
