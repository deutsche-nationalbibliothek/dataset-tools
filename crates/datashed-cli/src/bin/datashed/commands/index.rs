use std::ffi::OsStr;
use std::fs::read_to_string;
use std::path::PathBuf;

use actix_web::rt::task::spawn_blocking;
use datashed::{
    Doctype, DoctypeRefinements, Document, GenreRefinements,
    GroupRefinements, TagsRefinements, doctype_dtype, genre_dtype,
    group_dtype, iso6392b_dtype, translit,
};
use indicatif::ParallelProgressIterator;
use pica_record::prelude::*;
use walkdir::WalkDir;

use crate::prelude::*;

/// Create an index of all available documents
#[derive(Debug, clap::Parser)]
pub(crate) struct Index {
    /// Optionally, the index can be enriched with the genre, group,
    /// and doctype columns using a metadata extract (PICA+ format).
    metadata: Option<PathBuf>,

    /// Write the filename (without extension) into the specified
    /// column.
    #[arg(long, value_name = "COLUMN")]
    filename_column: Option<String>,

    /// Whether to add a doctype column or not.
    #[arg(long)]
    with_doctype: bool,

    /// The default document type if the metadata could not be used to
    /// determine the document's type.
    #[arg(long, default_value = "none", value_name = "doctype")]
    default_doctype: String,

    /// Whether to add a `genre` column or not.
    #[arg(long)]
    with_genre: bool,

    /// The default genre if the metadata could not be used to
    /// determine the document's genre.
    #[arg(long, default_value = "none", value_name = "genre")]
    default_genre: String,

    /// Whether to add a `group` column or not.
    #[arg(long)]
    with_group: bool,

    /// The default group if the metadata could not be used to
    /// determine the document's group.
    #[arg(long, default_value = "none", value_name = "group")]
    default_group: String,

    /// Whether to add a `tags` column or not.
    #[arg(long)]
    with_tags: bool,

    /// Stop processing after <N> documents.
    #[arg(long, short, default_value = "0", value_name = "N")]
    limit: usize,

    /// Write the index to <OUTPUT> instead to `index.ipc`
    #[arg(long, short)]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
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
        let mut tags_refinements = TagsRefinements::default();

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

            if let Some(path) = refinements.tags {
                let mut content = read_to_string(path)?;
                if let Some(ref runtime) = config.runtime {
                    content = translit(runtime.normalization)(content);
                }

                tags_refinements = toml_edit::de::from_str(&content)?;
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

                if self.with_tags {
                    tags_refinements
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
        let mut tags_map = tags_refinements.finish();

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
        let mut tags: Vec<Vec<String>> = vec![];
        let mut mtimes: Vec<u64> = vec![];

        for doc in docs.into_iter() {
            let name = doc.name;

            let doctype = doc
                .doctype
                .map(|dt: Doctype| dt.to_string())
                .or(doctype_map
                    .remove(&name)
                    .and_then(|list| list.into_iter().next()))
                .unwrap_or(self.default_doctype.clone());

            if self.with_genre {
                let genre: String = genre_map
                    .remove(&name)
                    .map(|list| {
                        list.into_iter()
                            .next()
                            .unwrap_or(self.default_genre.clone())
                    })
                    .unwrap_or(self.default_genre.clone());
                genres.push(genre);
            }

            if self.with_group {
                let group = group_map
                    .remove(&name)
                    .map(|list| {
                        list.into_iter()
                            .next()
                            .unwrap_or(self.default_group.clone())
                    })
                    .unwrap_or(self.default_group.clone());
                groups.push(group);
            }

            if self.with_tags {
                let tags_ = tags_map.remove(&name).unwrap_or_default();
                tags.push(tags_);
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

        if self.with_doctype {
            columns.push(
                col!("doctype", doctypes).cast(&doctype_dtype())?,
            );
        }

        if self.with_genre {
            columns.push(col!("genre", genres).cast(&genre_dtype())?);
        }

        if self.with_group {
            columns.push(col!("group", groups).cast(&group_dtype())?);
        }

        if self.with_tags {
            let tags: Vec<Series> = tags
                .into_iter()
                .map(|tags| tags.into_iter().collect::<Series>())
                .collect();
            columns.push(col!("tags", tags));
        }

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
            index = unnest_index(
                index,
                self.with_doctype,
                self.with_genre,
                self.with_group,
                self.with_tags,
            );
        }

        index = index.sort(["path"], Default::default());

        let mut df = spawn_blocking(move || index.collect()).await??;
        let output =
            self.output.or(Some(base_dir.join(Datashed::INDEX)));
        write_df(&mut df, output)?;

        Ok(SUCCESS)
    }
}
