use clap::builder::Styles;
use clap::builder::styling::{AnsiColor, Effects};
use clap::{Parser, Subcommand};

use crate::commands::*;

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());

#[derive(Debug, Parser)]
#[command(name = "dataset", version, about, long_about = None)]
#[command(max_term_width = 72, styles = STYLES)]
#[command(disable_help_subcommand = true)]
pub(crate) struct Args {
    /// Number of threads to use
    #[clap(
        short = 'j',
        long,
        value_name = "n",
        hide_env_values = true
    )]
    pub(crate) num_jobs: Option<usize>,

    #[command(subcommand)]
    pub(crate) cmd: Box<Command>,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    Init(Init),
}

#[derive(Debug, clap::Args)]
pub(crate) struct CommonOpts {
    /// Operate quietly; do not show progress
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub(crate) quiet: bool,

    /// Run verbosely; print additional infos to stderr
    #[arg(short, long, global = true, conflicts_with = "quiet")]
    pub(crate) verbose: bool,
}
