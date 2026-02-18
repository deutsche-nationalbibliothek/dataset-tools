use std::path::PathBuf;

use actix_web::rt::task::spawn_blocking;
use bstr::ByteSlice;
use datashed::Document;
use hashbrown::HashMap;
use polars::prelude::*;
use unicode_normalization::UnicodeNormalization;

use crate::prelude::*;

const PBAR_PROCESS: &str = "Processing documents: {human_pos} ({percent}%) | \
        elapsed: {elapsed_precise}{msg}";

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzäöüß";

/// Create a frequency table over a fixed alphabet.
#[derive(Debug, clap::Parser)]
pub(crate) struct Lfreq {
    /// The alphabet used to determine the letter frequencies.
    #[arg(long, default_value = ALPHABET, value_name = "alphabet")]
    alphabet: String,

    /// Write output to `filename` instead of `stdout`.
    #[arg(short, long, value_name = "filename")]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

struct Row {
    path: String,
    hash: String,
    total: u64,
    freqs: HashMap<char, u64>,
}

impl Lfreq {
    pub(crate) async fn execute(self) -> CommandResult {
        let datashed = Datashed::discover()?;
        let data_dir = datashed.data_dir();

        let index = read_index(&datashed, &self.filter).await?;
        let paths = index.column("path")?.str()?;

        let mut alphabet = self
            .alphabet
            .to_lowercase()
            .chars()
            .nfc()
            .collect::<Vec<char>>();
        alphabet.sort_unstable();
        alphabet.dedup();

        let pbar =
            ProgressBarBuilder::new(PBAR_PROCESS, self.common.quiet)
                .len(index.height() as u64)
                .build();

        let rows = (0..index.height())
            .into_par_iter()
            .progress_with(pbar)
            .map(|idx| -> DatashedResult<Row> {
                let path = paths.get(idx).unwrap();
                let (doc, data) = Document::from_path(
                    data_dir.join(path),
                    &data_dir,
                )?;

                let content = data.to_str()?.chars().nfc().to_string();
                let freqs = content
                    .to_lowercase()
                    .chars()
                    .filter(|c| alphabet.contains(c))
                    .fold(
                        HashMap::<char, u64>::with_capacity(
                            alphabet.len(),
                        ),
                        |mut acc, x| {
                            acc.entry(x)
                                .and_modify(|e| *e += 1)
                                .or_insert(1);
                            acc
                        },
                    );

                Ok(Row {
                    path: doc.path,
                    hash: doc.hash,
                    total: freqs.values().sum(),
                    freqs,
                })
            })
            .collect::<Result<Vec<Row>, _>>()?;

        let mut freqs =
            HashMap::<char, Vec<u64>>::with_capacity(alphabet.len());

        let mut paths = vec![];
        let mut hashes = vec![];
        let mut totals = vec![];

        for row in rows.into_iter() {
            for c in alphabet.iter() {
                let count = row.freqs.get(c).unwrap_or(&0);
                freqs
                    .entry(*c)
                    .and_modify(|e| e.push(*count))
                    .or_insert(vec![*count]);
            }

            paths.push(row.path);
            hashes.push(row.hash);
            totals.push(row.total);
        }

        let len = paths.len();

        let mut series = vec![
            Column::new("path".into(), paths),
            Column::new("hash".into(), hashes),
            Column::new("total".into(), totals),
        ];

        for c in alphabet {
            series.push(Column::new(
                c.to_string().into(),
                freqs.get(&c).unwrap(),
            ));
        }

        let lf = DataFrame::new(len, series)?
            .lazy()
            .sort(["path"], Default::default());

        let mut df = spawn_blocking(move || lf.collect()).await??;
        write_df(&mut df, self.output)?;
        Ok(SUCCESS)
    }
}
