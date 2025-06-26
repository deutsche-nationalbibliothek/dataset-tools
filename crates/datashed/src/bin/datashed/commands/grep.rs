use std::fs;
use std::path::PathBuf;

use datashed::translit;
use regex::bytes::RegexSetBuilder;

use crate::cli::FilterOpts;
use crate::prelude::*;

const PBAR_PROCESS: &str = "Processing documents: {human_pos} ({percent}%) | \
        elapsed: {elapsed_precise}{msg}";

/// Find documents matching patterns
#[derive(Debug, clap::Parser)]
pub(crate) struct Grep {
    /// Use only the first n bytes to search for the given pattern. If
    /// the value is 0 or greater than the document size, the entire
    /// document is used for searching.
    #[arg(long, short = 'n', value_name = "n")]
    max_bytes: Option<usize>,

    /// If set, all patterns will be search case insensitive.
    #[arg(long, short = 'i')]
    ignore_case: bool,

    /// Keep documents that don't match
    #[arg(long = "invert-match")]
    invert: bool,

    /// Write the result to  `filename`. By default output will be
    /// written in CSV format to `stdout`
    #[arg(short, long, value_name = "filename")]
    output: Option<PathBuf>,

    /// Regular expressions used for searching
    patterns: Vec<String>,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Grep {
    pub(crate) fn execute(self) -> CommandResult {
        let datashed = Datashed::discover()?;
        let data_dir = datashed.data_dir();
        let config = datashed.config()?;

        let index = read_index(&datashed, &self.filter)?;

        let patterns: Vec<String> = self
            .patterns
            .iter()
            .map(translit(
                config.runtime.and_then(|rt| rt.normalization),
            ))
            .collect();

        let re = RegexSetBuilder::new(patterns)
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
                    if n > 0 {
                        haystack.truncate(n);
                    }
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
