use clap::ValueEnum;
use datashed::Document;

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
    /// Set the verify mode: permissive, strict (default), or
    /// pedantic.
    #[arg(
        long,
        default_value = "strict",
        value_name = "mode",
        hide_possible_values = true,
        hide_default_value = true
    )]
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

                let Ok(doc) = Document::from_path(&path, &data_dir)
                else {
                    bail!(
                        "verification failed: file not found (path = {}).",
                        path.display()
                    );
                };

                let expected = hashes.get(idx).unwrap();
                let actual = doc.hash;

                if !actual.starts_with(expected) {
                    bail!(
                        "verification failed: hash mismatch (path = {}).",
                        path.display()
                    );
                }

                if self.mode >= VerifyMode::Strict
                    && doc.mtime != mtimes.get(idx).unwrap()
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
