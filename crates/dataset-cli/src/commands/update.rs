use clap::Parser;

use crate::prelude::*;

/// Create a new or update an existing corpus.
#[derive(Debug, Parser)]
#[clap(visible_alias = "create")]
pub(crate) struct Update {
    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Update {
    pub(crate) fn execute(self) -> CommandResult {
        let dataset = Dataset::discover()?;
        let base_dir = dataset.base_dir();
        let params = dataset.params()?;

        if let Some(vocab) = params.vocabulary() {
            vocab.save(base_dir)?;
        }

        Ok(SUCCESS)
    }
}
