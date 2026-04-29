use std::process::ExitCode;

use clap::Parser;
use cli::{Args, Command};
use dataset::Dataset;
use rayon::ThreadPoolBuilder;

pub(crate) mod cli;
pub(crate) mod commands;
pub(crate) mod config;
pub(crate) mod dataset;
pub(crate) mod prelude;
pub(crate) mod remote;

fn num_jobs(args: &Args) -> usize {
    if let Some(value) = args.num_jobs {
        return value;
    }

    if let Ok(dataset) = Dataset::discover()
        && let Ok(config) = dataset.config()
        && let Some(runtime) = config.runtime
        && let Some(num_jobs) = runtime.num_jobs
    {
        return num_jobs;
    }

    0
}

#[actix_web::main]
async fn main() -> ExitCode {
    let args = Args::parse();

    ThreadPoolBuilder::new()
        .num_threads(num_jobs(&args))
        .build_global()
        .unwrap();

    let result = match *args.cmd {
        Command::Config(cmd) => cmd.execute(),
        Command::Fetch(cmd) => cmd.execute().await,
        Command::Init(cmd) => cmd.execute(),
        Command::Remote(cmd) => cmd.execute(),
        Command::Version(cmd) => cmd.execute(),
    };

    match result {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::FAILURE
        }
    }
}
