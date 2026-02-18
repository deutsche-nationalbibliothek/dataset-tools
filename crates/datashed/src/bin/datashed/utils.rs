use std::ffi::OsStr;
use std::fs::File;
use std::io::stdout;
use std::path::{Path, PathBuf};

use actix_web::rt::task::spawn_blocking;
use datashed::{Datashed, DatashedResult};
use polars::prelude::*;
use polars::sql::SQLContext;

use crate::cli::FilterOpts;
use crate::prelude::bail;

macro_rules! col {
    ($name:expr, $values:expr) => {
        Column::new($name.into(), $values)
    };
}

pub(crate) use col;

pub(crate) fn is_arrow<P: AsRef<Path>>(
    path: &Option<P>,
) -> Option<bool> {
    if let Some(path) = path {
        path.as_ref()
            .extension()
            .and_then(OsStr::to_str)
            .map(|s| s == "ipc")
    } else {
        Some(false)
    }
}

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
            Some("txt") => {
                if df.column("ppn").is_ok() {
                    let mut writer =
                        CsvWriter::new(File::create(path)?)
                            .include_header(false);
                    writer.finish(&mut df.select(["ppn"])?)?;
                } else {
                    bail!("missing `ppn` column");
                }
            }
            _ => {
                let mut writer = IpcWriter::new(File::create(path)?)
                    .with_compression(Some(IpcCompression::ZSTD(
                        Default::default(),
                    )));
                writer.finish(df)?;
            }
        }
    } else {
        let mut writer = CsvWriter::new(stdout().lock());
        writer.finish(df)?;
    }

    Ok(())
}

#[inline]
pub(crate) fn unnest_index(
    lf: LazyFrame,
    with_genre: bool,
) -> LazyFrame {
    let mut lf = lf
        .unnest(by_name(["lang"], true, false), Some("_".into()))
        .with_columns([
            col("lang_code").cast(DataType::String),
            col("doctype").cast(DataType::String),
        ]);

    if with_genre {
        lf = lf.with_columns([col("genre").cast(DataType::String)]);
    }

    lf
}

pub(crate) async fn read_index(
    datashed: &Datashed,
    filter: &FilterOpts,
) -> DatashedResult<DataFrame> {
    let index = if let Some(ref path) = filter.index {
        IpcReader::new(File::open(path)?)
            .memory_mapped(None)
            .finish()?
    } else {
        datashed.index()?
    };

    let index_has_ppn = index.column("ppn").is_ok();
    let mut index = index.lazy();

    if let Some(ref path) = filter.allow {
        let allow_list = read_df(path)?;

        index = if allow_list.column("path").is_ok() {
            if allow_list.column("hash").is_ok() {
                index.join(
                    allow_list.lazy(),
                    [col("path"), col("hash")],
                    [col("path"), col("hash")],
                    JoinArgs::new(JoinType::Semi),
                )
            } else {
                index.semi_join(
                    allow_list.lazy(),
                    col("path"),
                    col("path"),
                )
            }
        } else if allow_list.column("ppn").is_ok() && index_has_ppn {
            index.semi_join(allow_list.lazy(), col("ppn"), col("ppn"))
        } else if index_has_ppn {
            bail!("missing `path` or `ppn` column.")
        } else {
            bail!("missing `path` column.")
        }
    };

    // DENY LIST
    if let Some(ref path) = filter.deny {
        let deny_list = read_df(path)?;

        index = if deny_list.column("path").is_ok() {
            if deny_list.column("hash").is_ok() {
                index.join(
                    deny_list.lazy(),
                    [col("path"), col("hash")],
                    [col("path"), col("hash")],
                    JoinArgs::new(JoinType::Anti),
                )
            } else {
                index.anti_join(
                    deny_list.lazy(),
                    col("path"),
                    col("path"),
                )
            }
        } else if deny_list.column("ppn").is_ok() && index_has_ppn {
            index.anti_join(deny_list.lazy(), col("ppn"), col("ppn"))
        } else if index_has_ppn {
            bail!("missing `path` or `ppn` column.")
        } else {
            bail!("missing `path` column.")
        };
    };

    if let Some(ref predicate) = filter.predicate {
        let mut ctx = SQLContext::new();
        ctx.register("df", index);

        let query = format!("SELECT * FROM df WHERE {predicate}");
        index = spawn_blocking(move || ctx.execute(&query)).await??;
    }

    Ok(spawn_blocking(move || index.collect()).await??)
}
