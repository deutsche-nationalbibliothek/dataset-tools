use std::process::ExitCode;

pub(crate) use anyhow::bail;
pub(crate) use datashed::{Config, Datashed, DatashedResult};
pub(crate) use indicatif::ProgressIterator as _;
pub(crate) use polars::prelude::*;
pub(crate) use rayon::prelude::*;

pub(crate) use crate::cli::CommonArgs;
pub(crate) use crate::progress::ProgressBarBuilder;

pub type CommandResult = DatashedResult<ExitCode>;

pub(crate) const SUCCESS: ExitCode = ExitCode::SUCCESS;
// pub(crate) const FAILURE: ExitCode = ExitCode::FAILURE;
