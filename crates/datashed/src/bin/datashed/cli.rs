use std::path::PathBuf;

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
    Archive(Archive),
    Bibrefs(Bibrefs),
    Completions(Completions),
    Config(Config),
    Grep(Grep),
    Index(Index),
    Init(Init),
    Lfreq(Lfreq),
    Restore(Restore),
    Summary(Summary),
    Verify(Verify),
    Version(Version),
    Vocab(Vocab),
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

#[derive(Debug, clap::Args)]
pub(crate) struct FilterOpts {
    /// Path to an allow list file
    #[arg(long = "allow-list", short = 'A', value_name = "filename")]
    pub(crate) allow: Option<PathBuf>,

    /// Path to a deny list file
    #[arg(long = "deny-list", short = 'D', value_name = "filename")]
    pub(crate) deny: Option<PathBuf>,

    /// Use an alternative index
    #[arg(long, short = 'I', value_name = "filename")]
    pub(crate) index: Option<PathBuf>,

    /// A predicate to filter the index set
    #[arg(long = "where", value_name = "predicate")]
    pub(crate) predicate: Option<String>,
}
