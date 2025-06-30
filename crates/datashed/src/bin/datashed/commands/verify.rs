use std::fs;
use std::time::UNIX_EPOCH;

use clap::ValueEnum;
use datashed::document::sha256;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, PartialOrd, Default, ValueEnum)]
pub(crate) enum VerifyMode {
    Permissive,
    #[default]
    Strict,
}

/// Verify whether the metadata conforms to the inventory
#[derive(Debug, clap::Parser)]
pub(crate) struct Verify {
    /// Set the verify mode
    #[arg(long, short, default_value = "strict", value_name = "mode")]
    mode: VerifyMode,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

const PBAR_VERIFY: &str = "Verifying documents: {human_pos} ({percent}%) | \
        elapsed: {elapsed_precise}{msg}";

impl Verify {
    pub(crate) fn execute(self) -> CommandResult {
        let datashed = Datashed::discover()?;
        let data_dir = datashed.data_dir();
        let index = datashed.index()?;

        let paths = index.column("path")?.str()?;
        let hashes = index.column("hash")?.str()?;

        let mtimes = index.column("mtime")?.cast(&DataType::UInt64)?;
        let mtimes = mtimes.u64()?;

        let pbar =
            ProgressBarBuilder::new(PBAR_VERIFY, self.common.quiet)
                .len(index.height() as u64)
                .build();

        (0..index.height())
            .into_par_iter()
            .progress_with(pbar)
            .try_for_each(|idx| -> Result<(), anyhow::Error> {
                let path = data_dir.join(paths.get(idx).unwrap());

                let Ok(data) = fs::read(&path) else {
                    bail!(
                        "verification failed: file not found (path = {}).",
                        path.display()
                    );
                };

                let expected = hashes.get(idx).unwrap();
                let actual = sha256(data);

                if !actual.starts_with(expected) {
                    bail!(
                        "verification failed: hash mismatch (path = {}).",
                        path.display()
                    );
                }

                let expected = mtimes.get(idx).unwrap();
                let metadata = path.metadata()?;
                let actual = metadata
                    .modified()
                    .ok()
                    .and_then(|x| x.duration_since(UNIX_EPOCH).ok())
                    .map(|x| x.as_secs())
                    .expect("valid mtime");

                if self.mode >= VerifyMode::Strict
                    && expected != actual
                {
                    bail!(
                        "verification failed: mtime mismatch (path = {}).",
                        path.display()
                    );
                }

                Ok(())
            })?;

        Ok(SUCCESS)
    }
}
