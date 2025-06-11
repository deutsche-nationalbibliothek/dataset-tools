use std::fs::Metadata;
use std::os::linux::fs::MetadataExt;
use std::path::{Path, PathBuf};

use crate::DatashedResult;

pub struct Document {
    _path: PathBuf,
    metadata: Metadata,
}

impl Document {
    pub fn from_path<P: AsRef<Path>>(path: P) -> DatashedResult<Self> {
        let path = path.as_ref().to_path_buf();
        let metadata = path.metadata()?;
        Ok(Self {
            _path: path,
            metadata,
        })
    }

    #[inline(always)]
    pub fn size(&self) -> u64 {
        self.metadata.st_size()
    }
}
