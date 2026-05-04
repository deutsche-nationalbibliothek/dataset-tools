use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::DatasetResult;
use crate::vocab::Vocabulary;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Parameters {
    /// The path of the config.
    #[serde(skip)]
    path: PathBuf,

    vocabulary: Option<Vocabulary>,

    /// This structure should always be constructed using a public
    /// constructor or using the update syntax:
    #[doc(hidden)]
    #[serde(skip)]
    __non_exhaustive: (),
}

impl Parameters {
    pub fn create<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Ok(Self {
            path: path.as_ref().into(),
            ..Default::default()
        })
    }

    /// Loads parameters from a path.
    pub fn from_path<P>(path: P) -> DatasetResult<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().into();
        let content = fs::read_to_string(&path)?;
        let mut params: Self = toml_edit::de::from_str(&content)?;
        params.path = path;

        Ok(params)
    }

    pub fn save(&self) -> DatasetResult<()> {
        let content = toml_edit::ser::to_string_pretty(self)?;
        let mut out = File::create(&self.path)?;
        out.write_all(content.as_bytes())?;

        Ok(())
    }

    pub fn vocabulary(&self) -> Option<&Vocabulary> {
        self.vocabulary.as_ref()
    }
}
