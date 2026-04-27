use std::process::ExitCode;

pub(crate) use dataset::{Config, Dataset};
pub(crate) use dataset_core::{CommandResult, bail};

pub(crate) use crate::cli::CommonOpts;

pub(crate) const SUCCESS: ExitCode = ExitCode::SUCCESS;
// pub(crate) const FAILURE: ExitCode = ExitCode::FAILURE;
