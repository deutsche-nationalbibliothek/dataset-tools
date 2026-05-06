use std::fs::{File, read_to_string};
use std::path::PathBuf;
use std::time::Instant;

use actix_web::rt::task::spawn_blocking;
use indexmap::IndexMap;
use owo_colors::OwoColorize;
use polars::sql::SQLContext;
use serde::Deserialize;

use crate::cli::FilterOpts;
use crate::prelude::*;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
struct Config {
    #[serde(
        rename = "check",
        skip_serializing_if = "IndexMap::is_empty",
        default
    )]
    checks: IndexMap<String, CheckSpec>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
struct CheckSpec {
    description: Option<String>,
    query: String,
    #[serde(default)]
    skip: bool,
}

/// Executes checks to ensure the integrity of the index
#[derive(Debug, clap::Parser)]
pub(crate) struct Check {
    /// Specify a file containing bibliographic identifiers so that
    /// these can also be checked.
    #[arg(long, short = 'B', value_name = "filename")]
    bibrefs: Option<PathBuf>,

    /// The configuration file that contains the checks to be
    /// performed.
    #[arg(default_value = "checks.toml")]
    config: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Check {
    pub(crate) async fn execute(self) -> CommandResult {
        let content = read_to_string(self.config.unwrap())?;
        let config: Config = toml_edit::de::from_str(&content)?;

        let datashed = Datashed::discover()?;
        let index = read_index(&datashed, &self.filter).await?;

        let ctx = SQLContext::new();
        ctx.register("index", index.lazy());

        if let Some(path) = self.bibrefs {
            let bibrefs = IpcReader::new(File::open(path)?).finish()?;
            ctx.register("bibrefs", bibrefs.lazy());
        } else {
            let path = datashed.base_dir().join("bibrefs.ipc");
            if path.is_file() {
                let bibrefs =
                    IpcReader::new(File::open(path)?).finish()?;
                ctx.register("bibrefs", bibrefs.lazy());
            }
        }

        let mut passed = 0;
        let mut skipped = 0;
        let mut failed = 0;

        let start = Instant::now();

        for (id, check) in config.checks.iter() {
            if check.skip {
                skipped += 1;
                continue;
            }

            let ctx = ctx.clone();
            let query = check.query.clone();

            let result =
                spawn_blocking(move || ctx.clone().execute(&query))
                    .await?;

            let Ok(lf) = result else {
                eprintln!("invalid query: {}", check.query);
                return Ok(FAILURE);
            };

            let result = spawn_blocking(move || lf.collect()).await??;
            let column = result.columns().first().unwrap();

            let info = if let Some(ref desc) = check.description {
                desc.to_string()
            } else {
                check.query.to_owned()
            };

            let elapsed = start.elapsed().as_secs_f32();

            match column.get(0).unwrap_or_default() {
                AnyValue::Boolean(true) => {
                    passed += 1;
                    if !self.common.quiet {
                        println!(
                            "        {} [{elapsed:>9.3}s] {} ⊢ {}",
                            "PASS".green(),
                            id.cyan(),
                            info.cyan(),
                        )
                    }
                }
                _ => {
                    failed += 1;
                    if !self.common.quiet {
                        println!(
                            "        {} [{elapsed:>9.3}s] {} ⊢ {}",
                            "FAIL".red(),
                            id.cyan(),
                            info.cyan(),
                        )
                    }
                }
            }
        }

        if !self.common.quiet {
            println!("────────────");
            println!(
                "     {} {} checks executed: {passed} {}, {skipped} {}, {failed} {}",
                "Summary".green(),
                passed + failed,
                "passed".green(),
                "skipped".yellow(),
                "failed".red()
            );
        }

        Ok(if failed > 0 { FAILURE } else { SUCCESS })
    }
}
