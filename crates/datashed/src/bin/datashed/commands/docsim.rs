use std::fs;

use bstr::{BString, ByteSlice};
use hashbrown::HashSet;
use rand::seq::SliceRandom;

use crate::prelude::*;

const PBAR_VOCAB: &str = "Building vocabulary: {human_pos} ({percent}%) | \
        elapsed: {elapsed_precise}{msg}";

const PBAR_PROCESS: &str = "Processing documents: {human_pos} ({percent}%) | \
        elapsed: {elapsed_precise}{msg}";

#[derive(Debug, clap::Parser)]
pub(crate) struct Docsim {
    #[arg(long, short)]
    limit: Option<usize>,

    #[arg(short = 'k')]
    size: usize,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Docsim {
    pub(crate) fn execute(self) -> CommandResult {
        let datashed = Datashed::discover()?;
        let data_dir = datashed.data_dir();

        let mut index = read_index(&datashed, &self.filter)?;
        if let Some(length) = self.limit {
            index = index.head(Some(length));
        }

        let paths = index.column("path")?.str()?;

        let pbar =
            ProgressBarBuilder::new(PBAR_VOCAB, self.common.quiet)
                .len(index.height() as u64)
                .build();

        let vocab: HashSet<BString> = (0..index.height())
            .into_par_iter()
            .progress_with(pbar)
            .map(|idx| -> HashSet<BString> {
                let path = paths.get(idx).unwrap();
                let data = fs::read(data_dir.join(path)).unwrap();
                let normalized = data.to_lowercase().words().fold(
                    String::with_capacity(data.len()),
                    |mut acc, item| {
                        acc.push_str(item);
                        acc.push(' ');
                        acc
                    },
                );

                normalized
                    .as_bytes()
                    .windows(self.size)
                    .map(BString::from)
                    .collect::<HashSet<BString>>()
            })
            .reduce(HashSet::new, |mut acc, other| {
                for shingle in other.iter() {
                    acc.insert(shingle.to_owned());
                }
                acc
            });

        let vocab = vocab.into_iter().collect::<Vec<BString>>();
        let mut indexes: Vec<usize> = (0..vocab.len()).collect();
        let shuffles = Vec::from_iter((0..10).into_iter().map(|_| {
            indexes.shuffle(&mut rand::rng());
            indexes.clone()
        }));

        let pbar =
            ProgressBarBuilder::new(PBAR_PROCESS, self.common.quiet)
                .len(index.height() as u64)
                .build();

        let _signatueres: Vec<Vec<usize>> = (0..index.height())
            .into_par_iter()
            .progress_with(pbar)
            .map(|idx| -> Vec<usize> {
                let path = paths.get(idx).unwrap();
                let data = fs::read(data_dir.join(path)).unwrap();
                let normalized = data.to_lowercase().words().fold(
                    String::with_capacity(data.len()),
                    |mut acc, item| {
                        acc.push_str(item);
                        acc.push(' ');
                        acc
                    },
                );

                let shingles = normalized
                    .as_bytes()
                    .windows(self.size)
                    .map(BString::from)
                    .collect::<HashSet<BString>>();

                let one_hot: Vec<u8> =
                    Vec::from_iter(vocab.iter().map(|value| {
                        if shingles.contains(value) { 1 } else { 0 }
                    }));

                assert!(one_hot.len() == vocab.len());

                let signature: Vec<usize> = (0..10)
                    .into_iter()
                    .map(|i| -> usize {
                        for i in shuffles[i].iter() {
                            if one_hot[*i] == 1 {
                                return *i;
                            }
                        }

                        unreachable!()
                    })
                    .collect();

                signature
            })
            .collect();

        Ok(SUCCESS)
    }
}
