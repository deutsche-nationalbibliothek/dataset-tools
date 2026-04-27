use std::process::ExitCode;

pub(crate) use dataset_core::{
    CommandResult, DatasetError, DatasetResult, bail,
};
pub(crate) use datashed::{Config, Datashed};
pub(crate) use indicatif::{
    ParallelProgressIterator, ProgressIterator,
};
pub(crate) use polars::prelude::*;
pub(crate) use rayon::prelude::*;

pub(crate) use crate::cli::{CommonOpts, FilterOpts};
pub(crate) use crate::progress::ProgressBarBuilder;
pub(crate) use crate::utils::*;

pub(crate) const SUCCESS: ExitCode = ExitCode::SUCCESS;
pub(crate) const FAILURE: ExitCode = ExitCode::FAILURE;
