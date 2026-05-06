use std::fs::{self, read_to_string};
use std::path::PathBuf;

use bstr::ByteSlice;
use clap::ValueEnum;
use datashed::translit;
use hashbrown::{HashMap, HashSet};

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
    /// Use two adjacent words as vocabulary terms
    #[arg(long, short, conflicts_with = "trigrams")]
    bigrams: bool,

    /// Use three adjacent words as vocabulary terms
    #[arg(long, short, conflicts_with = "bigrams")]
    trigrams: bool,

    /// Exclude words that are contained in the stop word list
    #[arg(long, short = 'S', value_name = "filename")]
    stopwords: Option<PathBuf>,

    /// Includes only those terms in the vocabulary where at least one
    /// character belongs to one of the specified unicode categories.
    /// Possible categories: all (a), lowercase (l), uppercase (u),
    /// titlecase (t), modifier (m), or other (o).
    #[arg(
        long = "category",
        short = 'L',
        value_name = "category",
        hide_possible_values = true
    )]
    categories: Vec<UnicodeCategory>,

    /// Ignore tokens with a length less than <n>
    #[arg(long, default_value = "2", value_name = "n")]
    min_term_length: usize,

    /// Ignore tokens with a term frequency less than <n>
    #[arg(
        long,
        default_value = "0",
        value_name = "n",
        hide_default_value = true
    )]
    min_term_freq: u64,

    /// Ignore tokens with a document frequency less than <n>
    #[arg(
        long,
        default_value = "0",
        value_name = "n",
        hide_default_value = true
    )]
    min_doc_freq: u64,

    /// Limits the output to the n most frequent tokens
    #[arg(long, short = 'l', value_name = "n")]
    limit: Option<usize>,

    /// Write the result to <filename>. By default output will be
    /// written in CSV format to stdout
    #[arg(short, long, value_name = "filename")]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Vocab {
    pub(crate) async fn execute(self) -> CommandResult {
        let datashed = Datashed::discover()?;
        let data_dir = datashed.data_dir();
        let config = datashed.config()?;

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

        let translit =
            translit(config.runtime.and_then(|rt| rt.normalization));

        let mut stopwords: HashSet<String> = HashSet::new();
        stopwords.insert("___SENTINEL___".into());

        if let Some(path) = self.stopwords {
            stopwords.extend(
                read_to_string(path)?
                    .lines()
                    .filter(|term| term.len() >= self.min_term_length)
                    .map(str::to_lowercase)
                    .map(translit),
            );
        }

        let index = read_index(&datashed, &self.filter).await?;
        let paths = index.column("path")?.str()?;

        let pbar =
            ProgressBarBuilder::new(PBAR_PROCESS, self.common.quiet)
                .len(index.height() as u64)
                .build();

        let mut vocab: VocabMap = (0..index.height())
            .into_par_iter()
            .progress_with(pbar)
            .map(|idx| {
                let path = paths.get(idx).unwrap();
                let data = fs::read(data_dir.join(path)).unwrap();
                let tokens: Vec<String> = data
                    .words()
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
                    .filter(|word| {
                        word.chars().count() >= self.min_term_length
                    })
                    .filter(|word| !stopwords.contains(word))
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

        if self.min_term_freq > 1 || self.min_doc_freq > 1 {
            vocab.retain(|_, (tf, df)| {
                *tf >= self.min_term_freq && *df >= self.min_doc_freq
            });
        }

        let mut tokens = Vec::with_capacity(vocab.len());
        let mut freqs = Vec::with_capacity(vocab.len());
        let mut docs = Vec::with_capacity(vocab.len());

        for (token, (tf, df)) in vocab.into_iter() {
            tokens.push(token);
            freqs.push(tf);
            docs.push(df);
        }

        let mut result = DataFrame::new(
            tokens.len(),
            vec![
                Column::new("term".into(), tokens),
                Column::new("tf".into(), freqs),
                Column::new("df".into(), docs),
            ],
        )?;

        // result = result.lazy().collect()?;

        let options = SortMultipleOptions::default()
            .with_order_descending_multi([true, true, false]);
        result = result.sort(["tf", "df", "term"], options)?;

        if self.limit.is_some() {
            result = result.head(self.limit);
        }

        write_df(&mut result, self.output)?;
        Ok(SUCCESS)
    }
}
