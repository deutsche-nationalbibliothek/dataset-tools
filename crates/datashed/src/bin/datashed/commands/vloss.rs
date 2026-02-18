use std::fs::read_to_string;
use std::path::PathBuf;

use actix_web::rt::task::spawn_blocking;
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

    #[arg(long, requires_all = ["step_width"])]
    absolute: bool,

    #[arg(long)]
    start: Option<usize>,

    #[arg(long)]
    end: Option<usize>,

    #[arg(long, default_value_t = 10)]
    step_width: usize,

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
    threshold: f64,
    vloss: f64,
}

#[derive(Debug)]
struct Step {
    threshold: f64,
    end: usize,
}

impl Vloss {
    pub(crate) async fn execute(self) -> CommandResult {
        let datashed = Datashed::discover()?;
        let data_dir = datashed.data_dir();
        let config = datashed.config()?;

        let index = read_index(&datashed, &self.filter).await?;

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
                let mut steps: Vec<Step> = Vec::new();

                if vocab.is_empty() {
                    return result;
                }

                if self.absolute {
                    let end = self.end.unwrap();
                    let start = self.start.unwrap_or_default();
                    let mut p = if start == 0 {
                        self.step_width
                    } else {
                        start
                    };

                    while p <= end {
                        if p <= data.len() {
                            steps.push(Step {
                                threshold: p as f64,
                                end: p,
                            });
                        } else {
                            steps.push(Step {
                                threshold: p as f64,
                                end: data.len(),
                            });
                        }

                        p += self.step_width;
                    }
                } else {
                    let step_with = 100 / self.step_width;
                    let mut p = step_with;

                    while p < 100 {
                        let threshold = p as f64 / 100f64;

                        steps.push(Step {
                            threshold,
                            end: (threshold * data.len() as f64)
                                as usize,
                        });

                        p += step_with;
                    }
                }

                // eprintln!("{steps:?}");

                for step in steps.into_iter() {
                    let vocab_n: HashSet<String> = data[0..step.end]
                        .words()
                        .map(str::to_lowercase)
                        .filter(|word| vocab.contains(word))
                        .collect();

                    let vloss = 1f64
                        - vocab_n.len() as f64 / vocab.len() as f64;

                    result.push(Record {
                        path: doc.path.clone(),
                        hash: doc.hash.clone(),
                        threshold: step.threshold,
                        vloss,
                    });
                }

                result
            })
            .flatten()
            .collect();

        let mut paths = Vec::new();
        let mut hashes = Vec::new();
        let mut thresholds = Vec::new();
        let mut vloss = Vec::new();

        for row in records.into_iter() {
            paths.push(row.path);
            hashes.push(row.hash);
            thresholds.push(row.threshold);
            vloss.push(row.vloss);
        }

        let mut columns = vec![];

        columns.push(col!("path", paths));
        columns.push(col!("hash", hashes));
        if !self.absolute {
            columns.push(col!("threshold", thresholds));
        } else {
            columns.push(
                col!("threshold", thresholds)
                    .cast(&DataType::UInt64)?,
            );
        }
        columns.push(col!("vloss", vloss));

        let mut lf = DataFrame::new(columns.len(), columns)?
            .lazy()
            .inner_join(index.lazy(), col("path"), col("path"))
            .group_by([col("doctype"), col("threshold")])
            .agg([
                col("vloss").mean().alias("vloss_mean"),
                col("doctype").count().alias("N"),
            ])
            .sort(["doctype", "threshold"], Default::default())
            .select([
                col("doctype"),
                col("N"),
                col("threshold"),
                col("vloss_mean"),
            ]);

        if !is_arrow(&self.output).unwrap_or_default() {
            lf = lf
                .with_columns([col("doctype").cast(DataType::String)]);
        }

        let mut df = spawn_blocking(move || lf.collect()).await??;
        write_df(&mut df, self.output)?;
        Ok(SUCCESS)
    }
}
