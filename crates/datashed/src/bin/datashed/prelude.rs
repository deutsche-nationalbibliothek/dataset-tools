use std::process::ExitCode;

pub(crate) use anyhow::bail;
pub(crate) use datashed::{Config, Datashed, DatashedResult};
pub(crate) use indicatif::{
    ParallelProgressIterator, ProgressIterator,
};
pub(crate) use polars::prelude::*;
pub(crate) use rayon::prelude::*;

pub(crate) use crate::cli::CommonOpts;
pub(crate) use crate::progress::ProgressBarBuilder;
pub(crate) use crate::utils::{
    apply_allow_list, apply_deny_list, write_df,
};

pub type CommandResult = DatashedResult<ExitCode>;

pub(crate) const SUCCESS: ExitCode = ExitCode::SUCCESS;
// pub(crate) const FAILURE: ExitCode = ExitCode::FAILURE;
