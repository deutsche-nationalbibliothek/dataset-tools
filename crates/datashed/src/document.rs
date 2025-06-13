use std::os::linux::fs::MetadataExt;
use std::path::Path;

use crate::DatashedResult;

pub struct Document {
    pub path: String,
    pub size: u64,
}

impl Document {
    pub fn from_path<P: AsRef<Path>>(
        path: P,
        data_dir: P,
    ) -> DatashedResult<Self> {
        let path = path.as_ref().to_path_buf();
        let metadata = path.metadata()?;

        let relpath = path
            .strip_prefix(data_dir)
            .expect("strip prefix")
            .to_str()
            .expect("valid path")
            .into();

        Ok(Self {
            path: relpath,
            size: metadata.st_size(),
        })
    }
}
