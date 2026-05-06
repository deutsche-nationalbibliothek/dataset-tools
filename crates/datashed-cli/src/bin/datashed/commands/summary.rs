use std::fs;
use std::path::PathBuf;

use crate::cli::FilterOpts;
use crate::prelude::*;

/// Creates summary statistics in JSON format
#[derive(Debug, clap::Parser)]
pub(crate) struct Summary {
    /// Write the result to  `filename`. By default output will be
    /// written in JSON format to `stdout`
    #[arg(short, long, value_name = "filename")]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Summary {
    pub(crate) async fn execute(self) -> CommandResult {
        let datashed = Datashed::discover()?;
        let index = read_index(&datashed, &self.filter).await?;

        let sizes = index.column("size")?.cast(&DataType::UInt64)?;
        let sizes = sizes.u64()?;

        let docs = index.height();
        let mut size_sum: u64 = 0;

        for idx in 0..docs {
            size_sum += sizes.get(idx).unwrap();
        }

        let content =
            format!("{{\"docs\": {docs}, \"size\": {size_sum}}}");

        if let Some(path) = self.output {
            fs::write(path, content)?;
        } else {
            println!("{content}");
        }

        Ok(SUCCESS)
    }
}
