use std::fmt::Write;
use std::fs::{self};
use std::os::linux::fs::MetadataExt;
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::DatashedResult;

pub struct Document {
    pub path: String,
    pub hash: String,
    pub size: u64,
}

#[inline]
fn sha256<T: AsRef<[u8]>>(data: T) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);

    let hash = hasher.finalize();
    hash.iter().take(4).fold(String::new(), |mut out, b| {
        let _ = write!(out, "{b:02x}");
        out
    })
}

impl Document {
    pub fn from_path<P: AsRef<Path>>(
        path: P,
        data_dir: P,
    ) -> DatashedResult<Self> {
        let path = path.as_ref().to_path_buf();
        let metadata = path.metadata()?;
        let content = fs::read(&path)?;

        let relpath = path
            .strip_prefix(data_dir)
            .expect("strip prefix")
            .to_str()
            .expect("valid path")
            .into();

        Ok(Self {
            path: relpath,
            hash: sha256(content),
            size: metadata.st_size(),
        })
    }
}
