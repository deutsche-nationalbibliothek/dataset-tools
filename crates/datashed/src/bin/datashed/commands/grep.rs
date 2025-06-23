use std::fs::{self, File};
use std::path::PathBuf;

use polars::sql::SQLContext;
use regex::bytes::RegexSetBuilder;

use crate::prelude::*;

const PBAR_PROCESS: &str = "Processing documents: {human_pos} ({percent}%) | \
        elapsed: {elapsed_precise}{msg}";

/// Find documents matching patterns
#[derive(Debug, clap::Parser)]
pub(crate) struct Grep {
    #[command(flatten)]
    pub(crate) common: CommonArgs,

    /// Use an alternative index
    #[arg(long, short = 'I')]
    index: Option<PathBuf>,

    /// Ignore documents which are not explicitly listed in the given
    /// allow lists.
    #[arg(long = "allow-list", short = 'A')]
    allow: Option<PathBuf>,

    /// Ignore documents which are listed in the given deny lists.
    #[arg(long = "deny-list", short = 'D')]
    deny: Option<PathBuf>,

    /// Use only the first n bytes to search for the given pattern. If
    /// the value is 0 or greater than the document size, the entire
    /// document is used for searching.
    #[arg(long, short = 'n', value_name = "n")]
    max_bytes: Option<usize>,

    /// If set, all patterns will be search case insensitive.
    #[arg(long, short = 'i')]
    ignore_case: bool,

    /// Keep documents that don't match.
    #[arg(long = "invert-match")]
    invert: bool,

    /// Additional regular expressions used for searching
    #[arg(long = "or")]
    patterns: Vec<String>,

    /// An optional predicate to filter the document-set.
    #[arg(long = "where")]
    predicate: Option<String>,

    /// Write the result to  `filename`. By default output will be
    /// written in CSV format to `stdout`.
    #[arg(short, long, value_name = "filename")]
    output: Option<PathBuf>,

    /// A regular expression used for searching
    pattern: Vec<String>,
}

impl Grep {
    pub(crate) fn execute(mut self) -> CommandResult {
        let datashed = Datashed::discover()?;
        let data_dir = datashed.data_dir();

        let mut index = if let Some(ref path) = self.index {
            IpcReader::new(File::open(path)?)
                .memory_mapped(None)
                .finish()?
                .lazy()
        } else {
            datashed.index()?.lazy()
        };

        index = apply_allow_list(index, self.allow)?;
        index = apply_deny_list(index, self.deny)?;

        if let Some(predicate) = self.predicate {
            let mut ctx = SQLContext::new();
            ctx.register("df", index);
            index = ctx.execute(&format!(
                "SELECT * FROM df WHERE {predicate}"
            ))?;
        }

        let index = index.collect()?;

        self.pattern.extend_from_slice(&self.patterns);
        let re = RegexSetBuilder::new(self.pattern)
            .case_insensitive(self.ignore_case)
            .build()?;

        let pbar =
            ProgressBarBuilder::new(PBAR_PROCESS, self.common.quiet)
                .len(index.height() as u64)
                .build();

        let paths = index.column("path")?.str()?;

        let matches: Vec<String> = (0..index.height())
            .into_par_iter()
            .progress_with(pbar)
            .filter_map(|idx| -> Option<String> {
                let path = paths.get(idx).unwrap();
                let mut haystack =
                    fs::read(data_dir.join(path)).unwrap();

                if let Some(n) = self.max_bytes {
                    haystack.truncate(n);
                }

                if re.is_match(&haystack) ^ self.invert {
                    Some(path.to_string())
                } else {
                    None
                }
            })
            .collect();

        let matches =
            DataFrame::new(vec![Column::new("path".into(), &matches)])?
                .lazy();

        let mut result = index
            .lazy()
            .semi_join(matches, col("path"), col("path"))
            .sort(["path"], Default::default())
            .collect()?;

        write_df(&mut result, self.output)?;

        Ok(SUCCESS)
    }
}
