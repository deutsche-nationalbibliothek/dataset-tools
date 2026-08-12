use std::fs::File;
use std::io::Cursor;
use std::time::Duration;

use clap::Parser;
use indicatif::{HumanBytes, HumanCount, ProgressBar};
use polars::prelude::*;
use polars::sql::SQLContext;
use reqwest::ClientBuilder;

use crate::prelude::*;

/// Download indices and metadata from datasheds
#[derive(Debug, Parser)]
pub(crate) struct Fetch {
    /// Show index size, without making any changes.
    #[arg(long)]
    dry_run: bool,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Fetch {
    pub(crate) async fn execute(self) -> CommandResult {
        let dataset = Dataset::discover()?;
        let remotes_dir = dataset.remotes_dir();
        let config = dataset.config()?;
        let remotes = config.remotes;

        let client = ClientBuilder::new().build()?;

        for (name, remote) in remotes.iter() {
            let pbar = if !self.common.quiet {
                ProgressBar::new_spinner()
            } else {
                ProgressBar::hidden()
            };

            pbar.enable_steady_tick(Duration::from_millis(100));
            pbar.set_message(format!("fetching {name} index..."));

            let mut url = remote.url.clone();
            url.set_path("/index.ipc");

            let data = client.get(url).send().await?.bytes().await?;
            if data.is_empty() {
                bail!(
                    "unable to get datashed index from remote '{name}'",
                );
            }

            let mut lf = IpcReader::new(Cursor::new(data))
                .finish()?
                .lazy()
                .with_column(lit(name.as_str()).alias("remote"));

            if let Some(ref predicate) = remote.predicate {
                let query =
                    format!("SELECT * FROM lf WHERE {predicate}");

                let mut ctx = SQLContext::new();
                ctx.register("lf", lf);

                lf = spawn_blocking(move || ctx.execute(&query))
                    .await??;
            }

            lf = lf.select([
                col("remote"),
                col("path"),
                col("hash"),
                col("doctype"),
                col("lang"),
                col("size"),
                col("mtime"),
            ]);

            let mut index =
                spawn_blocking(move || lf.collect()).await??;

            let path = remotes_dir.join(format!("{name}.ipc"));
            let count = index.height() as u64;
            let bytes: u64 =
                index.column("size")?.u64()?.sum().unwrap_or_default();

            let delta: i64 = if path.exists() {
                let df = IpcReader::new(File::open(&path)?).finish()?;

                (index.height() - df.height()) as i64
            } else {
                index.height() as i64
            };

            if !self.dry_run {
                let mut writer = IpcWriter::new(File::create(path)?)
                    .with_compression(Some(IpcCompression::ZSTD(
                        Default::default(),
                    )));
                writer.finish(&mut index)?;
            }

            pbar.finish_and_clear();

            if !self.common.quiet {
                println!(
                    "{name}: {} documents ({}, delta {}), done.",
                    HumanCount(count),
                    HumanBytes(bytes),
                    if delta < 0 {
                        format!("-{}", HumanCount(-delta as u64))
                    } else {
                        format!("{}", HumanCount(delta as u64))
                    }
                );
            }
        }

        Ok(SUCCESS)
    }
}
