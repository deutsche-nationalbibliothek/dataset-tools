use std::fmt::{self, Display};
use std::fs;
use std::path::PathBuf;

use isbn::IsbnMatcher;

use crate::cli::FilterOpts;
use crate::prelude::*;

mod isbn;

const PBAR_PROCESS: &str = "Processing documents: {human_pos} ({percent}%) | \
        elapsed: {elapsed_precise}{msg}";

#[derive(Debug, PartialEq)]
pub(crate) struct Reference {
    kind: Kind,
    value: String,
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Kind {
    Isbn,
}

impl Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Isbn => write!(f, "isbn"),
        }
    }
}

pub(crate) trait Matcher {
    fn matches<T: AsRef<[u8]>>(&self, data: T) -> Vec<Reference>;
}

/// Finds bibliographic identifiers in documents
#[derive(Debug, clap::Parser)]
pub(crate) struct Bibrefs {
    /// Write the result to <filename>. By default output will be
    /// written in CSV format to stdout
    #[arg(short, long, value_name = "filename")]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

#[derive(Debug)]
struct Row {
    path: String,
    hash: String,
    kind: Kind,
    value: String,
    start: usize,
    end: usize,
}

impl Bibrefs {
    pub(crate) fn execute(self) -> CommandResult {
        let datashed = Datashed::discover()?;
        let data_dir = datashed.data_dir();

        let index = read_index(&datashed, &self.filter)?;
        let paths = index.column("path")?.str()?;
        let hashes = index.column("hash")?.str()?;

        let pbar =
            ProgressBarBuilder::new(PBAR_PROCESS, self.common.quiet)
                .len(index.height() as u64)
                .build();

        let matchers = [IsbnMatcher { normalize: true }];

        let rows = (0..index.height())
            .into_par_iter()
            .progress_with(pbar)
            .flat_map(|idx| {
                let path = paths.get(idx).unwrap();
                let hash = hashes.get(idx).unwrap();
                let data = fs::read(data_dir.join(path)).unwrap();

                matchers
                    .iter()
                    .flat_map(|m| m.matches(&data))
                    .map(|m| Row {
                        path: path.to_string(),
                        hash: hash.to_string(),
                        kind: m.kind,
                        value: m.value,
                        start: m.start,
                        end: m.end,
                    })
                    .collect::<Vec<Row>>()
            })
            .collect::<Vec<Row>>();

        let mut paths = vec![];
        let mut hashes = vec![];
        let mut kinds = vec![];
        let mut values = vec![];
        let mut starts = vec![];
        let mut ends = vec![];

        for row in rows.into_iter() {
            paths.push(row.path);
            hashes.push(row.hash);
            kinds.push(row.kind.to_string());
            values.push(row.value);
            starts.push(row.start as u64);
            ends.push(row.end as u64);
        }

        let mut columns = vec![
            col!("path", paths),
            col!("hash", hashes),
            col!("kind", kinds),
            col!("value", values),
        ];

        if is_arrow(&self.output).unwrap_or_default() {
            columns.push(Column::new(
                "span".into(),
                DataFrame::new(vec![
                    col!("start", starts),
                    col!("end", ends),
                ])?
                .into_struct("span".into()),
            ));
        } else {
            columns.push(col!("start", starts));
            columns.push(col!("end", ends));
        }

        let mut result = DataFrame::new(columns)?
            .lazy()
            .sort(["path"], Default::default())
            .collect()?;

        write_df(&mut result, self.output)?;
        Ok(SUCCESS)
    }
}
