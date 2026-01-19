use std::fs::read_to_string;
use std::path::PathBuf;

use bstr::ByteSlice;
use datashed::{Document, translit};
use hashbrown::HashSet;

use crate::cli::FilterOpts;
use crate::commands::vocab::UnicodeCategory;
use crate::prelude::*;

const PBAR_PROCESS: &str = "Processing documents: {human_pos} ({percent}%) | \
        elapsed: {elapsed_precise}{msg}";

#[derive(Debug, clap::Parser)]
pub(crate) struct Vloss {
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

    #[arg(long, default_value = "0.1")]
    increment: f64,

    /// Ignore tokens with a length less than <n>
    #[arg(long, default_value = "2", value_name = "n")]
    min_term_length: usize,

    /// Write the result to  `filename`. By default output will be
    /// written in CSV format to `stdout`
    #[arg(short, long, value_name = "filename")]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

#[derive(Debug)]
struct Record {
    path: String,
    hash: String,
    vloss: f64,
    n: f64,
}

impl Vloss {
    pub(crate) fn execute(self) -> CommandResult {
        let datashed = Datashed::discover()?;
        let data_dir = datashed.data_dir();
        let config = datashed.config()?;

        let index = read_index(&datashed, &self.filter)?;

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

        let pbar =
            ProgressBarBuilder::new(PBAR_PROCESS, self.common.quiet)
                .len(index.height() as u64)
                .build();

        let paths = index.column("path")?.str()?;

        let records: Vec<Record> = (0..index.height())
            .into_par_iter()
            .progress_with(pbar)
            .map(|idx| -> Vec<Record> {
                let path = paths.get(idx).unwrap();
                let (doc, data) =
                    Document::from_path(data_dir.join(path), &data_dir)
                        .unwrap();

                let vocab: HashSet<String> = data
                    .words()
                    .map(str::to_lowercase)
                    .filter(|word| {
                        if !self.categories.is_empty() {
                            predicates
                                .iter()
                                .any(|f| word.chars().any(f))
                        } else {
                            true
                        }
                    })
                    .filter(|word| {
                        word.chars().count() >= self.min_term_length
                    })
                    .filter(|word| !stopwords.contains(word))
                    .collect();

                let mut result: Vec<Record> = Vec::new();
                let mut n = 0f64;

                if vocab.is_empty() {
                    return result;
                }

                loop {
                    if n >= 1.0 {
                        break;
                    }

                    let end = (data.len() as f64 * n).floor() as usize;
                    let vocab_n: HashSet<String> = data[0..end]
                        .words()
                        .map(str::to_lowercase)
                        .filter(|word| {
                            if !self.categories.is_empty() {
                                predicates
                                    .iter()
                                    .any(|f| word.chars().any(f))
                            } else {
                                true
                            }
                        })
                        .filter(|word| {
                            word.chars().count() >= self.min_term_length
                        })
                        .filter(|word| !stopwords.contains(word))
                        .filter(|word| vocab.contains(word))
                        .collect();

                    let vloss = 1f64
                        - vocab_n.len() as f64 / vocab.len() as f64;

                    result.push(Record {
                        path: doc.path.clone(),
                        hash: doc.hash.clone(),
                        n,
                        vloss,
                    });

                    n += self.increment;
                }

                result
            })
            .flatten()
            .collect();

        let mut paths = Vec::new();
        let mut hashes = Vec::new();
        let mut vloss = Vec::new();
        let mut ns = Vec::new();

        for row in records.into_iter() {
            paths.push(row.path);
            hashes.push(row.hash);
            vloss.push(row.vloss);
            ns.push(row.n);
        }

        let mut df = DataFrame::new(vec![
            col!("path", paths),
            col!("hash", hashes),
            col!("n", ns),
            col!("vloss", vloss),
        ])?;

        df.sort_in_place(["path"], Default::default())?;
        df.shrink_to_fit();

        write_df(&mut df, self.output)?;

        Ok(SUCCESS)
    }
}
