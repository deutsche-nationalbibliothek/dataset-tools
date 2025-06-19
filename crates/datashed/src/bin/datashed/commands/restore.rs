use std::fs::{File, create_dir};
use std::path::PathBuf;

use clap::Parser;
use flate2::read::GzDecoder;
use tar::Archive;

use crate::prelude::*;

/// Restore a datashed archive
#[derive(Debug, Parser)]
pub(crate) struct Restore {
    #[command(flatten)]
    pub(crate) common: CommonArgs,

    /// The destination directory.
    #[arg(short = 'C', long = "directory", default_value = ".")]
    dest: PathBuf,

    /// The datashed archive to be restored.
    archive: PathBuf,
}

impl Restore {
    pub(crate) fn execute(self) -> CommandResult {
        if !self.dest.is_dir() {
            create_dir(&self.dest)?;

            if self.common.verbose {
                eprintln!(
                    "Created destination directory '{}'.",
                    self.dest.display()
                )
            }
        }

        let reader = GzDecoder::new(File::open(self.archive)?);
        let mut archive = Archive::new(reader);
        archive.unpack(&self.dest)?;

        if !self.dest.join(Datashed::DATA_DIR).is_dir() {
            bail!("corrupt archive: missing data dir!");
        }

        if !self.dest.join(Datashed::INDEX).is_file() {
            bail!("corrupt archive: missing index!");
        }

        if !self.dest.join(Datashed::CONFIG).is_file() {
            bail!("corrupt archive: missing config!");
        }

        if !self.common.quiet {
            eprintln!("Successfully restored archive.");
            eprintln!("Verify consistency with `datashed verify`.")
        }

        Ok(SUCCESS)
    }
}
