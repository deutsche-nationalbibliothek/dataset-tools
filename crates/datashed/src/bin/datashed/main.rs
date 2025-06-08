use std::process::ExitCode;

use clap::Parser;

use crate::cli::{Args, Command};
use crate::prelude::CommandResult;

pub(crate) mod cli;
pub(crate) mod commands;
pub(crate) mod prelude;

fn run(args: Args) -> CommandResult {
    match *args.cmd {
        Command::Init(cmd) => cmd.execute(),
        Command::Version(cmd) => cmd.execute(),
    }
}

fn main() -> ExitCode {
    let args = Args::parse();

    match run(args) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::FAILURE
        }
    }
}
