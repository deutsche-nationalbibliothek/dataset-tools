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
#[command(name = "datashed", version, about, long_about = None)]
#[command(max_term_width = 72, styles = STYLES)]
#[command(disable_help_subcommand = true)]
pub(crate) struct Args {
    /// Number of threads to use. If this options isn't set or a value
    /// of "0" is chosen, the maximum number of available threads
    /// is used.
    #[clap(short = 'j', long, hide_env_values = true)]
    pub(crate) num_jobs: Option<usize>,

    #[command(subcommand)]
    pub(crate) cmd: Box<Command>,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    Index(Index),
    Init(Init),
    Version(Version),
}

#[derive(Debug, clap::Args)]
pub(crate) struct CommonArgs {
    /// Operate quietly; do not show progress
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub(crate) quiet: bool,

    /// Run verbosely; print additional information to the standard
    /// error stream
    #[arg(short, long, global = true, conflicts_with = "quiet")]
    pub(crate) verbose: bool,
}
