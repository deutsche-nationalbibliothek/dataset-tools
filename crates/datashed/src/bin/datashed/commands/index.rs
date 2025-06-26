use std::fs::File;
use std::path::PathBuf;

use datashed::Document;
use indicatif::ParallelProgressIterator;
use walkdir::WalkDir;

use crate::prelude::*;

/// Create an index of all available documents
#[derive(Debug, clap::Parser)]
pub(crate) struct Index {
    #[arg(long, short)]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

const PBAR_COLLECT: &str = "Collecting documents: {human_pos} | \
        elapsed: {elapsed_precise}{msg}";

const PBAR_INDEX: &str = "Indexing documents: {human_pos} ({percent}%) | \
        elapsed: {elapsed_precise}{msg}";

impl Index {
    pub(crate) fn execute(self) -> CommandResult {
        let datashed = Datashed::discover()?;
        let data_dir = datashed.data_dir();
        let base_dir = datashed.base_dir();

        let pbar =
            ProgressBarBuilder::new(PBAR_COLLECT, self.common.quiet)
                .build();

        let files = WalkDir::new(&data_dir)
            .into_iter()
            .filter_map(Result::ok)
            .map(|dirent| dirent.into_path())
            .filter(|path| {
                path.to_str()
                    .map(|s| s.ends_with(".txt"))
                    .unwrap_or(false)
            })
            .progress_with(pbar)
            .collect::<Vec<_>>();

        let pbar =
            ProgressBarBuilder::new(PBAR_INDEX, self.common.quiet)
                .len(files.len() as u64)
                .build();

        let docs = files
            .par_iter()
            .progress_with(pbar)
            .filter_map(|path| {
                if let Ok((doc, _)) =
                    Document::from_path(path, &data_dir)
                {
                    Some(doc)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let mut paths: Vec<String> = vec![];
        let mut hashes: Vec<String> = vec![];
        #[cfg(feature = "dnb")]
        let mut ppns: Vec<String> = vec![];
        let mut sizes: Vec<u64> = vec![];
        let mut mtimes: Vec<u64> = vec![];

        for doc in docs.into_iter() {
            paths.push(doc.path);
            hashes.push(doc.hash);
            #[cfg(feature = "dnb")]
            ppns.push(doc.ppn);
            sizes.push(doc.size);
            mtimes.push(doc.mtime);
        }

        let mut df = DataFrame::new(vec![
            Column::new("path".into(), paths),
            Column::new("hash".into(), hashes),
            #[cfg(feature = "dnb")]
            Column::new("ppn".into(), ppns),
            Column::new("size".into(), sizes),
            Column::new("mtime".into(), mtimes),
        ])?
        .lazy()
        .select([col("*").shrink_dtype()])
        .sort(["path"], Default::default())
        .collect()?;

        let path_str = if let Some(ref path) = self.output {
            path.to_str().unwrap_or_default()
        } else {
            ""
        };

        match self.output {
            Some(path) if path_str.ends_with(".csv") => {
                let mut writer = CsvWriter::new(File::create(path)?);
                writer.finish(&mut df)?;
            }
            Some(path) if path_str.ends_with(".tsv") => {
                let mut writer = CsvWriter::new(File::create(path)?)
                    .with_separator(b'\t');
                writer.finish(&mut df)?;
            }
            Some(path) => {
                let mut writer = IpcWriter::new(File::create(path)?)
                    .with_compression(Some(IpcCompression::ZSTD))
                    .with_parallel(true);

                writer.finish(&mut df)?;
            }
            None => {
                let mut writer = IpcWriter::new(File::create(
                    base_dir.join(Datashed::INDEX),
                )?)
                .with_compression(Some(IpcCompression::ZSTD))
                .with_parallel(true);

                writer.finish(&mut df)?;
            }
        }

        Ok(SUCCESS)
    }
}
