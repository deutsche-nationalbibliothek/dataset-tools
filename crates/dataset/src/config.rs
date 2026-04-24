use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use semver::Version;
use serde::{Deserialize, Serialize};

use crate::Remote;
use crate::error::DatasetResult;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// The path of the config.
    #[serde(skip)]
    path: PathBuf,

    /// Datashed metadata.
    pub metadata: Metadata,

    /// Runtime options.
    pub runtime: Option<Runtime>,

    #[serde(
        rename = "remote",
        skip_serializing_if = "BTreeMap::is_empty",
        default
    )]
    pub remotes: BTreeMap<String, Remote>,

    /// This structure should always be constructed using a public
    /// constructor or using the update syntax:
    ///
    /// ```ignore
    /// use crate::config::Config;
    ///
    /// let config = Config {
    ///     ..Default::default()
    /// };
    /// ```
    #[doc(hidden)]
    #[serde(skip)]
    __non_exhaustive: (),
}

impl Config {
    pub fn create<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Ok(Self {
            path: path.as_ref().into(),
            ..Default::default()
        })
    }

    /// Loads an existing config from a path.
    pub fn from_path<P>(path: P) -> DatasetResult<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().into();
        let content = fs::read_to_string(&path)?;
        let mut config: Self = toml_edit::de::from_str(&content)?;
        config.path = path;

        Ok(config)
    }

    /// Saves the config.
    pub fn save(&self) -> DatasetResult<()> {
        let content = toml_edit::ser::to_string_pretty(self)?;
        let mut out = File::create(&self.path)?;
        out.write_all(content.as_bytes())?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// The name of the dataset.
    pub name: String,

    /// The version of the dataset.
    pub version: Version,

    /// A short blurb about the dataset.
    pub description: Option<String>,

    /// A list of people or organizations, which are considered as the
    /// authors of the dataset.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub authors: Vec<String>,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            name: "".into(),
            description: None,
            version: Version::new(0, 1, 0),
            authors: Vec::new(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Runtime {
    /// Number of threads to use. If this options isn't set or a value
    /// of "0" is chosen, the maximum number of available threads
    /// is used.
    pub num_jobs: Option<usize>,
}
