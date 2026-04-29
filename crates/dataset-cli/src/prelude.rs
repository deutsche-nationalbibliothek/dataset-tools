use std::process::ExitCode;

pub(crate) use actix_web::rt::task::spawn_blocking;
pub(crate) use dataset_core::{CommandResult, bail};

pub(crate) use crate::cli::CommonOpts;
pub(crate) use crate::config::Config;
pub(crate) use crate::dataset::Dataset;

pub(crate) const SUCCESS: ExitCode = ExitCode::SUCCESS;
// pub(crate) const FAILURE: ExitCode = ExitCode::FAILURE;
