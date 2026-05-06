use std::path::PathBuf;
use std::{env, fs};

use dataset_core::{DatasetError, DatasetResult, Parameters};

use crate::config::Config;

pub struct Dataset {
    /// The root directory of the dataset.
    root_dir: PathBuf,
}

impl Dataset {
    pub const DOT_DIR: &'static str = ".dataset";
    pub const REMOTES_DIR: &'static str = "remotes";
    pub const CONFIG: &'static str = "config.toml";
    pub const PARAMS: &'static str = "params.toml";

    /// Discovers the root of the dataset.
    ///
    /// This function fails, if neither the current directory nor any
    /// parent directory contains a dataset [Config].
    pub fn discover() -> DatasetResult<Self> {
        let mut root_dir = env::current_dir()?;

        loop {
            if let Ok(metadata) = fs::metadata(
                root_dir.join(Self::DOT_DIR).join(Self::CONFIG),
            ) && metadata.is_file()
            {
                break;
            }

            if !root_dir.pop() {
                return Err(DatasetError::Other(
                    "not a dataset (or any parent directory)".into(),
                ));
            }
        }

        Ok(Self { root_dir })
    }

    /// Returns the config associated with the dataset.
    pub fn config(&self) -> DatasetResult<Config> {
        Config::from_path(
            self.root_dir.join(Self::DOT_DIR).join(Self::CONFIG),
        )
    }

    /// Returns the parameters associated with the dataset.
    pub fn params(&self) -> DatasetResult<Parameters> {
        Parameters::from_path(self.base_dir().join(Self::PARAMS))
    }

    /// Returns the base directory of the dataset.
    pub fn base_dir(&self) -> &PathBuf {
        &self.root_dir
    }

    /// Returns the remotes directory of the dataset.
    #[inline]
    pub fn dot_dir(&self) -> PathBuf {
        self.root_dir.join(Self::DOT_DIR)
    }

    /// Returns the remotes directory of the dataset.
    #[inline]
    pub fn remotes_dir(&self) -> PathBuf {
        self.dot_dir().join(Self::REMOTES_DIR)
    }
}
