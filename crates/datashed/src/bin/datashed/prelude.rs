use std::process::ExitCode;

pub(crate) use anyhow::bail;
pub(crate) use datashed::{Config, Datashed, DatashedResult};

pub(crate) use crate::cli::CommonArgs;

pub type CommandResult = DatashedResult<ExitCode>;

pub(crate) const SUCCESS: ExitCode = ExitCode::SUCCESS;
// pub(crate) const FAILURE: ExitCode = ExitCode::FAILURE;
