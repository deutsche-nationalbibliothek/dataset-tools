use std::fs::File;
use std::path::PathBuf;

use datashed::Document;
use indicatif::ParallelProgressIterator;
use walkdir::WalkDir;

use crate::prelude::*;

/// Create an index of all available documents
#[derive(Debug, clap::Parser)]
pub(crate) struct Index {
    #[command(flatten)]
    pub(crate) common: CommonArgs,

    /// Whether to overwrite the current version or not.
    #[arg(short, long)]
    force: bool,

    #[arg(long, short)]
    output: Option<PathBuf>,
}

const PBAR_COLLECT: &str = "Collecting documents: {human_pos} | \
        elapsed: {elapsed_precise}{msg}";

const PBAR_INDEX: &str = "Indexing documents: {human_pos} ({percent}%) | \
        elapsed: {elapsed_precise}{msg}";

struct Row {
    path: PathBuf,
    size: u64,
}

impl TryFrom<&PathBuf> for Row {
    type Error = anyhow::Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let doc = Document::from_path(path)?;

        Ok(Self {
            path: path.into(),
            size: doc.size(),
        })
    }
}

#[inline(always)]
fn is_plaintext(path: &PathBuf) -> bool {
    path.to_str().map(|s| s.ends_with(".txt")).unwrap_or(false)
}

impl Index {
    pub(crate) fn execute(self) -> CommandResult {
        let datashed = Datashed::discover()?;
        let data_dir = datashed.data_dir();
        let base_dir = datashed.base_dir();

        let pbar =
            ProgressBarBuilder::new(PBAR_COLLECT, self.common.quiet)
                .build();

        let files = WalkDir::new(&data_dir)
            .into_iter()
            .filter_map(Result::ok)
            .map(|dirent| dirent.into_path())
            .filter(is_plaintext)
            .progress_with(pbar)
            .collect::<Vec<_>>();

        let pbar =
            ProgressBarBuilder::new(PBAR_INDEX, self.common.quiet)
                .len(files.len() as u64)
                .build();

        let rows = files
            .par_iter()
            .progress_with(pbar)
            .map(Row::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let mut paths: Vec<String> = vec![];
        let mut sizes: Vec<u64> = vec![];

        for row in rows.into_iter() {
            let path = row
                .path
                .strip_prefix(base_dir)
                .unwrap()
                .to_str()
                .unwrap()
                .into();

            paths.push(path);
            sizes.push(row.size);
        }

        let mut df = DataFrame::new(vec![
            Column::new("path".into(), paths),
            Column::new("size".into(), sizes),
        ])?
        .lazy()
        .select([col("*").shrink_dtype()])
        .collect()?;

        match self.output {
            Some(path) => {
                let mut writer = IpcWriter::new(File::create(path)?)
                    .with_compression(Some(IpcCompression::ZSTD))
                    .with_parallel(true);

                writer.finish(&mut df)?;
            }
            None => todo!(),
        }

        Ok(SUCCESS)
    }
}
