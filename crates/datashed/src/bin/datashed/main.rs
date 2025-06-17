use std::process::ExitCode;

use clap::Parser;
use datashed::Datashed;
use rayon::ThreadPoolBuilder;

use crate::cli::{Args, Command};
use crate::prelude::CommandResult;

pub(crate) mod cli;
pub(crate) mod commands;
pub(crate) mod prelude;
pub(crate) mod progress;

fn num_jobs(args: &Args) -> usize {
    if let Some(value) = args.num_jobs {
        return value;
    }

    if let Ok(datashed) = Datashed::discover() {
        if let Ok(config) = datashed.config() {
            if let Some(runtime) = config.runtime {
                if let Some(num_jobs) = runtime.num_jobs {
                    return num_jobs;
                }
            }
        }
    }

    0
}

fn run(args: Args) -> CommandResult {
    match *args.cmd {
        Command::Index(cmd) => cmd.execute(),
        Command::Init(cmd) => cmd.execute(),
        Command::Verify(cmd) => cmd.execute(),
        Command::Version(cmd) => cmd.execute(),
    }
}

fn main() -> ExitCode {
    let args = Args::parse();

    ThreadPoolBuilder::new()
        .num_threads(num_jobs(&args))
        .build_global()
        .unwrap();

    match run(args) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::FAILURE
        }
    }
}
