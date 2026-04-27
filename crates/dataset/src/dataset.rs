use std::fs::File;
use std::path::PathBuf;
use std::{env, fs};

use dataset_core::{DatasetError, DatasetResult};
use polars::prelude::*;

use crate::Config;

pub struct Dataset {
    /// The root directory of the dataset.
    root_dir: PathBuf,
}

impl Dataset {
    pub const DOT_DIR: &'static str = ".dataset";
    pub const REMOTES_DIR: &'static str = "remotes";
    pub const DATA_DIR: &'static str = "data";
    pub const TMP_DIR: &'static str = "tmp";

    pub const CONFIG: &'static str = "config.toml";
    pub const INDEX: &'static str = "index.ipc";

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

    /// Returns the base directory of the dataset.
    pub fn base_dir(&self) -> &PathBuf {
        &self.root_dir
    }

    /// Returns the data directory of the dataset.
    pub fn data_dir(&self) -> PathBuf {
        self.root_dir.join(Self::DATA_DIR)
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

    /// Returns the index associated with the dataset.
    pub fn index(&self) -> DatasetResult<DataFrame> {
        Ok(IpcReader::new(File::open(
            self.base_dir().join(Self::INDEX),
        )?)
        .memory_mapped(None)
        .finish()?)
    }
}
