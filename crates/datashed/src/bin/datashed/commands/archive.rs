use std::fs::File;
use std::io::{Write, stdout};
use std::path::{Path, PathBuf};

use clap::Parser;
use flate2::Compression;
use flate2::write::GzEncoder;
use indicatif::ProgressIterator;

use crate::prelude::*;

const PBAR_ARCHIVE: &str = "Archive documents: {human_pos} ({percent}%) | \
        elapsed: {elapsed_precise}{msg}";

/// Create an archive of the index, config and all documents.
///
/// By default, the compression is biased towards high compression ratio
/// at expense of speed. To change this setting, use the `--fast` or
/// `--best` flag.
#[derive(Debug, Parser)]
pub(crate) struct Archive {
    /// Uses the lowest compression at the highest speed.
    #[arg(long, conflicts_with = "best")]
    fast: bool,

    /// Uses the best compression at the lowest speed.
    #[arg(long, conflicts_with = "fast")]
    best: bool,

    /// Write the archive to `filename` instead of stdout.
    #[arg(short, long, value_name = "filename")]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Archive {
    pub(crate) fn execute(self) -> CommandResult {
        let datashed = Datashed::discover()?;
        let base_dir = datashed.base_dir();
        let index = datashed.index()?;

        let paths = index.column("path")?.str()?;

        let level = if self.fast {
            Compression::fast()
        } else if self.best {
            Compression::best()
        } else {
            Compression::default()
        };

        let out: Box<dyn Write> = match self.output {
            Some(path) => Box::new(File::create(path)?),
            None => Box::new(stdout().lock()),
        };

        let gzip = GzEncoder::new(out, level);
        let mut archive = tar::Builder::new(gzip);

        let pbar =
            ProgressBarBuilder::new(PBAR_ARCHIVE, self.common.quiet)
                .len(paths.len() as u64)
                .build();

        paths.iter().progress_with(pbar).try_for_each(|path| {
            let path = path.unwrap();

            let mut file =
                File::open(base_dir.join("data").join(path)).unwrap();
            archive
                .append_file(Path::new("data").join(path), &mut file)
                .unwrap();

            Ok::<(), anyhow::Error>(())
        })?;

        let mut index = File::open(base_dir.join(Datashed::INDEX))?;
        archive.append_file(Datashed::INDEX, &mut index)?;

        let mut config = File::open(base_dir.join(Datashed::CONFIG))?;
        archive.append_file(Datashed::CONFIG, &mut config)?;

        archive.finish()?;
        Ok(SUCCESS)
    }
}
