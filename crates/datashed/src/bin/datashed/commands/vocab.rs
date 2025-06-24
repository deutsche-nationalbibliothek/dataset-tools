use std::fs::{self, File};

use bstr::ByteSlice;
use clap::ValueEnum;
use hashbrown::HashMap;
use polars::sql::SQLContext;

use crate::cli::FilterOpts;
use crate::prelude::*;

const PBAR_PROCESS: &str = "Processing documents: {human_pos} ({percent}%) | \
        elapsed: {elapsed_precise}{msg}";

type VocabMap = HashMap<String, (u64, u64)>;

#[derive(Debug, Clone, PartialEq, Eq, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub enum UnicodeCategory {
    #[clap(alias = "a")]
    All,
    #[clap(alias = "l")]
    Lowercase,
    #[clap(alias = "u")]
    Uppercase,
    #[clap(alias = "t")]
    Titlecase,
    #[clap(alias = "m")]
    Modifier,
    #[clap(alias = "o")]
    Other,
}

/// Create vocabulary (set of terms) statistics
#[derive(Debug, clap::Parser)]
pub(crate) struct Vocab {
    /// Use bigrams as terms
    #[arg(long, short, conflicts_with = "trigrams")]
    bigrams: bool,

    /// Use trigrams as terms
    #[arg(long, short, conflicts_with = "bigrams")]
    trigrams: bool,

    /// Includes only those tokens in the vocabulary where at least one
    /// character belongs to one of the specified unicode categories
    #[arg(
        long = "category",
        short = 'L',
        value_name = "name",
        hide_possible_values = true
    )]
    categories: Vec<UnicodeCategory>,

    /// Ignore tokens with a length less than `n`.
    #[arg(long, short = 'n', default_value = "2", value_name = "n")]
    min_token_length: usize,

    /// Limits the output to the n most frequent tokens
    #[arg(long, short = 'l', value_name = "n")]
    limit: Option<usize>,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

/// Includes only those tokens in the vocabulary where at least one
/// character belongs to one of the specified unicode categories
// #[arg(
//     short = 'L',
//     value_name = "category",
//     hide_possible_values = true
// )]
// categories: Vec<UnicodeCategory>,

impl Vocab {
    pub(crate) fn execute(self) -> CommandResult {
        let datashed = Datashed::discover()?;
        let data_dir = datashed.data_dir();

        let size = if self.bigrams {
            2
        } else if self.trigrams {
            3
        } else {
            1
        };

        let predicates: Vec<fn(char) -> bool> = self
            .categories
            .iter()
            .map(|category| {
                use UnicodeCategory::*;
                use unicode_categories::UnicodeCategories as UC;
                match category {
                    All => UC::is_letter,
                    Lowercase => UC::is_letter_lowercase,
                    Uppercase => UC::is_letter_uppercase,
                    Titlecase => UC::is_letter_titlecase,
                    Modifier => UC::is_letter_modifier,
                    Other => UC::is_letter_other,
                }
            })
            .collect();

        let mut index = if let Some(ref path) = self.filter.index {
            IpcReader::new(File::open(path)?)
                .memory_mapped(None)
                .finish()?
                .lazy()
        } else {
            datashed.index()?.lazy()
        };

        index = apply_allow_list(index, self.filter.allow)?;
        index = apply_deny_list(index, self.filter.deny)?;

        if let Some(predicate) = self.filter.predicate {
            let mut ctx = SQLContext::new();
            ctx.register("df", index);
            index = ctx.execute(&format!(
                "SELECT * FROM df WHERE {predicate}"
            ))?;
        }

        let index = index.collect()?;

        let pbar =
            ProgressBarBuilder::new(PBAR_PROCESS, self.common.quiet)
                .len(index.height() as u64)
                .build();

        let paths = index.column("path")?.str()?;

        let vocab: VocabMap = (0..index.height())
            .into_par_iter()
            .progress_with(pbar)
            .map(|idx| {
                let path = paths.get(idx).unwrap();
                let data = fs::read(data_dir.join(path)).unwrap();
                let tokens: Vec<String> = data
                    .words()
                    .filter(|word| {
                        word.chars().count() >= self.min_token_length
                    })
                    .filter(|word| {
                        if !self.categories.is_empty() {
                            predicates
                                .iter()
                                .any(|f| word.chars().any(f))
                        } else {
                            true
                        }
                    })
                    .map(str::to_lowercase)
                    .collect();

                tokens.windows(size).fold(
                    VocabMap::new(),
                    |mut vocab, window| {
                        let token = window.join(" ");
                        vocab
                            .entry(token)
                            .and_modify(|(tf, _)| *tf += 1)
                            .or_insert((1, 1));
                        vocab
                    },
                )
            })
            .reduce(VocabMap::new, |mut acc, vocab| {
                for (token, counts) in vocab.into_iter() {
                    acc.entry(token)
                        .and_modify(|(tf, df)| {
                            *tf += counts.0;
                            *df += counts.1;
                        })
                        .or_insert(counts);
                }
                acc
            });

        let mut tokens = Vec::with_capacity(vocab.len());
        let mut freqs = Vec::with_capacity(vocab.len());
        let mut docs = Vec::with_capacity(vocab.len());

        for (token, (tf, df)) in vocab.into_iter() {
            tokens.push(token);
            freqs.push(tf);
            docs.push(df);
        }

        let df = DataFrame::new(vec![
            Column::new("token".into(), tokens),
            Column::new("tf".into(), freqs),
            Column::new("df".into(), docs),
        ])?;

        let mut df = df.sort(
            ["tf", "df", "token"],
            SortMultipleOptions::default()
                .with_order_descending_multi([true, true, false]),
        )?;

        if self.limit.is_some() {
            df = df.head(self.limit);
        }

        // TODO: shrink dtypes
        // TODO: write df

        println!("{df}");

        Ok(SUCCESS)
    }
}
