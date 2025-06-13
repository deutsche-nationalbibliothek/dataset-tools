use std::path::PathBuf;
use std::{env, fs};

use anyhow::bail;

use crate::{Config, DatashedResult};

pub struct Datashed {
    /// The root directory of the datashed.
    root_dir: PathBuf,
}

impl Datashed {
    pub const DATA_DIR: &'static str = "data";
    pub const TMP_DIR: &'static str = "tmp";

    pub const CONFIG: &'static str = "config.toml";
    pub const INDEX: &'static str = "index.ipc";

    /// Discovers the root of the datashed.
    ///
    /// This function fails, if neither the current directory nor any
    /// parent directory contains a datashed [Config].
    pub fn discover() -> DatashedResult<Self> {
        let mut root_dir = env::current_dir()?;

        loop {
            if let Ok(metadata) =
                fs::metadata(root_dir.join(Self::CONFIG))
            {
                if metadata.is_file() {
                    break;
                }
            }

            if !root_dir.pop() {
                bail!("not a datashed (or any parent directory)");
            }
        }

        Ok(Self { root_dir })
    }

    /// Returns the config associated with the datashed.
    pub fn config(&self) -> DatashedResult<Config> {
        Config::from_path(self.root_dir.join(Self::CONFIG))
    }

    /// Returns the base directory of the datashed.
    pub fn base_dir(&self) -> &PathBuf {
        &self.root_dir
    }

    /// Returns the data directory of the datashed.
    pub fn data_dir(&self) -> PathBuf {
        self.root_dir.join(Self::DATA_DIR)
    }
}
