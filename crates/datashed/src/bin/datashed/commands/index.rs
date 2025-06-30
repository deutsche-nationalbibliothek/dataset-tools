use std::ffi::OsStr;
use std::path::PathBuf;

use datashed::{Document, iso6392b_dtype};
use indicatif::ParallelProgressIterator;
use walkdir::WalkDir;

use crate::prelude::*;

/// Create an index of all available documents
#[derive(Debug, clap::Parser)]
pub(crate) struct Index {
    /// Write the filename into the specified column.
    #[arg(long)]
    filename_column: Option<String>,

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

        let is_arrow = if let Some(ref path) = self.output {
            path.extension()
                .and_then(OsStr::to_str)
                .map(|s| s == "ipc")
                .unwrap_or_default()
        } else {
            true
        };

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
        let mut names: Vec<String> = vec![];
        let mut lang_codes: Vec<Option<String>> = vec![];
        let mut lang_scores: Vec<Option<f64>> = vec![];
        let mut sizes: Vec<u64> = vec![];
        let mut alphas: Vec<f64> = vec![];
        let mut mtimes: Vec<u64> = vec![];

        for doc in docs.into_iter() {
            paths.push(doc.path);
            hashes.push(doc.hash);
            names.push(doc.name);
            lang_codes.push(doc.lang_code);
            lang_scores.push(doc.lang_score);
            sizes.push(doc.size);
            alphas.push(doc.alpha);
            mtimes.push(doc.mtime);
        }

        let mut columns = vec![];

        columns.push(col!("path", paths));
        columns.push(col!("hash", hashes));

        if let Some(name) = self.filename_column {
            columns.push(col!(name, names));
        }

        columns.push(col!("size", sizes));

        let lang = DataFrame::new(vec![
            col!("lang_code", lang_codes).cast(&iso6392b_dtype())?,
            col!("lang_score", lang_scores),
        ])?
        .into_struct("lang".into());

        columns.push(col!("lang", lang));
        columns.push(col!("alpha", alphas));
        columns.push(col!("mtime", mtimes));

        let mut index = DataFrame::new(columns)?.lazy();
        if !is_arrow {
            index = unnest_index(index);
        }

        let mut df =
            index.sort(["path"], Default::default()).collect()?;

        let output =
            self.output.or(Some(base_dir.join(Datashed::INDEX)));
        write_df(&mut df, output)?;

        Ok(SUCCESS)
    }
}
