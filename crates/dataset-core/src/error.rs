use std::process::ExitCode;

use tokio::task::JoinError;

#[derive(Debug, thiserror::Error)]
pub enum DatasetError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    TomlSer(#[from] toml_edit::ser::Error),

    #[error(transparent)]
    TomlDe(#[from] toml_edit::de::Error),

    #[error(transparent)]
    Polars(#[from] polars::error::PolarsError),

    #[error(transparent)]
    Utf8(#[from] bstr::Utf8Error),

    #[error(transparent)]
    Regex(#[from] regex::Error),

    #[error("{0}")]
    Other(String),
}

impl DatasetError {
    pub fn other<T: Into<String>>(message: T) -> Self {
        Self::Other(message.into())
    }
}

impl From<JoinError> for DatasetError {
    fn from(err: JoinError) -> Self {
        Self::Other(format!("{err}"))
    }
}

impl From<reqwest::Error> for DatasetError {
    fn from(err: reqwest::Error) -> Self {
        Self::Other(format!("{err}"))
    }
}

pub type DatasetResult<T> = Result<T, DatasetError>;

pub type CommandResult = DatasetResult<ExitCode>;
