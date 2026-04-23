use std::ffi::OsStr;
use std::fs::read_to_string;
use std::path::PathBuf;

use actix_web::rt::task::spawn_blocking;
use datashed::{
    Doctype, DoctypeRefinements, Document, GenreRefinements,
    GroupRefinements, doctype_dtype, genre_dtype, group_dtype,
    iso6392b_dtype, translit,
};
use indicatif::ParallelProgressIterator;
use pica_record::prelude::*;
use walkdir::WalkDir;

use crate::prelude::*;

/// Create an index of all available documents
#[derive(Debug, clap::Parser)]
pub(crate) struct Index {
    /// Write the filename into the specified column.
    #[arg(long)]
    filename_column: Option<String>,

    #[arg(long, default_value = "none")]
    doctype: String,

    #[arg(long, default_value = "none")]
    genre: String,

    #[arg(long)]
    with_genre: bool,

    #[arg(long, default_value = "none")]
    group: String,

    #[arg(long)]
    with_group: bool,

    #[arg(long, short, default_value = "0")]
    limit: usize,

    #[arg(long, short)]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,

    metadata: Option<PathBuf>,
}

const PBAR_METADATA: &str = "Processing metadata: {human_pos} | \
        elapsed: {elapsed_precise}{msg}";

const PBAR_COLLECT: &str = "Enumerating documents: {human_pos} | \
        elapsed: {elapsed_precise}{msg}";

const PBAR_INDEX: &str = "Indexing documents: {human_pos} ({percent}%) | \
        elapsed: {elapsed_precise}{msg}";

impl Index {
    pub(crate) async fn execute(self) -> CommandResult {
        let datashed = Datashed::discover()?;
        let data_dir = datashed.data_dir();
        let base_dir = datashed.base_dir();
        let config = datashed.config()?;

        let mut doctype_refinements = DoctypeRefinements::default();
        let mut genre_refinements = GenreRefinements::default();
        let mut group_refinements = GroupRefinements::default();

        if let Some(refinements) = config.refinements {
            if let Some(path) = refinements.genre {
                let mut content = read_to_string(path)?;
                if let Some(ref runtime) = config.runtime {
                    content = translit(runtime.normalization)(content);
                }

                genre_refinements = toml_edit::de::from_str(&content)?;
            }

            if let Some(path) = refinements.group {
                let mut content = read_to_string(path)?;
                if let Some(ref runtime) = config.runtime {
                    content = translit(runtime.normalization)(content);
                }

                group_refinements = toml_edit::de::from_str(&content)?;
            }

            if let Some(path) = refinements.doctype {
                let mut content = read_to_string(path)?;
                if let Some(ref runtime) = config.runtime {
                    content = translit(runtime.normalization)(content);
                }

                doctype_refinements =
                    toml_edit::de::from_str(&content)?;
            }
        }

        if let Some(path) = self.metadata {
            let pbar = ProgressBarBuilder::new(
                PBAR_METADATA,
                self.common.quiet,
            )
            .build();

            let mut reader = ReaderBuilder::new().from_path(path)?;
            while let Some(result) = reader.next_byte_record() {
                let Ok(ref record) = result else {
                    continue;
                };

                if self.with_genre {
                    genre_refinements
                        .process_record(record, &Default::default());
                }

                if self.with_group {
                    group_refinements
                        .process_record(record, &Default::default());
                }

                doctype_refinements
                    .process_record(record, &Default::default());

                pbar.inc(1);
            }

            pbar.finish_using_style();
        }

        let mut doctype_map = doctype_refinements.finish();
        let mut genre_map = genre_refinements.finish();
        let mut group_map = group_refinements.finish();

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
            .take(if self.limit > 0 {
                self.limit
            } else {
                usize::MAX
            })
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
        let mut genres: Vec<String> = vec![];
        let mut groups: Vec<String> = vec![];
        let mut doctypes: Vec<String> = vec![];
        let mut lang_codes: Vec<Option<String>> = vec![];
        let mut lang_scores: Vec<Option<f64>> = vec![];
        let mut chars: Vec<u64> = vec![];
        let mut sizes: Vec<u64> = vec![];
        let mut lfreqs: Vec<Option<f64>> = vec![];
        let mut alphas: Vec<f64> = vec![];
        let mut mtimes: Vec<u64> = vec![];

        for doc in docs.into_iter() {
            let name = doc.name;

            let doctype = doc
                .doctype
                .map(|dt: Doctype| dt.to_string())
                .or(doctype_map.remove(&name))
                .unwrap_or(self.doctype.clone());

            if self.with_genre {
                let genre = genre_map
                    .remove(&name)
                    .unwrap_or(self.genre.clone());
                genres.push(genre);
            }

            if self.with_group {
                groups.push(
                    group_map
                        .remove(&name)
                        .unwrap_or(self.group.clone()),
                );
            }

            paths.push(doc.path);
            hashes.push(doc.hash);
            names.push(name);
            doctypes.push(doctype);
            lang_codes.push(doc.lang_code);
            lang_scores.push(doc.lang_score);
            chars.push(doc.chars);
            sizes.push(doc.size);
            lfreqs.push(doc.lfreq);
            alphas.push(doc.alpha);
            mtimes.push(doc.mtime);
        }

        let mut columns = vec![];
        let len = paths.len();

        columns.push(col!("path", paths));
        columns.push(col!("hash", hashes));

        if let Some(name) = self.filename_column {
            columns.push(col!(name, names));
        }

        if self.with_genre {
            columns.push(col!("genre", genres).cast(&genre_dtype())?);
        }

        if self.with_group {
            columns.push(col!("group", groups).cast(&group_dtype())?);
        }

        columns.push(col!("doctype", doctypes).cast(&doctype_dtype())?);
        columns.push(col!("chars", chars));
        columns.push(col!("size", sizes));

        let lang = DataFrame::new(
            len,
            vec![
                col!("code", lang_codes).cast(&iso6392b_dtype())?,
                col!("score", lang_scores),
            ],
        )?
        .into_struct("lang".into());

        columns.push(col!("lang", lang));
        columns.push(col!("lfreq", lfreqs));
        columns.push(col!("alpha", alphas));
        columns.push(col!("mtime", mtimes));

        let mut index = DataFrame::new(len, columns)?.lazy();
        if !is_arrow {
            index = unnest_index(index, self.with_genre);
        }

        index = index.sort(["path"], Default::default());

        let mut df = spawn_blocking(move || index.collect()).await??;
        let output =
            self.output.or(Some(base_dir.join(Datashed::INDEX)));
        write_df(&mut df, output)?;

        Ok(SUCCESS)
    }
}
