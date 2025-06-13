use std::process::ExitCode;

use clap::Parser;
use rayon::ThreadPoolBuilder;

use crate::cli::{Args, Command};
use crate::prelude::CommandResult;

pub(crate) mod cli;
pub(crate) mod commands;
pub(crate) mod prelude;
pub(crate) mod progress;

fn run(args: Args) -> CommandResult {
    match *args.cmd {
        Command::Index(cmd) => cmd.execute(),
        Command::Init(cmd) => cmd.execute(),
        Command::Version(cmd) => cmd.execute(),
    }
}

fn main() -> ExitCode {
    let args = Args::parse();

    ThreadPoolBuilder::new()
        .num_threads(args.num_jobs.unwrap_or(0))
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
