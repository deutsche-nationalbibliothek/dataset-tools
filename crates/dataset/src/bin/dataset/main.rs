use std::process::ExitCode;

use clap::Parser;
use cli::{Args, Command};

pub(crate) mod cli;
pub(crate) mod commands;
pub(crate) mod prelude;

fn main() -> ExitCode {
    let args = Args::parse();

    let result = match *args.cmd {
        Command::Config(cmd) => cmd.execute(),
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
