use std::process::ExitCode;

pub(crate) use anyhow::bail;
pub(crate) use dataset::{Config, Dataset, DatasetResult};

pub(crate) use crate::cli::CommonOpts;

pub type CommandResult = DatasetResult<ExitCode>;

pub(crate) const SUCCESS: ExitCode = ExitCode::SUCCESS;
// pub(crate) const FAILURE: ExitCode = ExitCode::FAILURE;
