#[cfg(feature = "dnb")]
use std::ffi::OsStr;
use std::fmt::Write;
use std::fs::{self};
use std::os::linux::fs::MetadataExt;
use std::path::Path;
use std::time::UNIX_EPOCH;

use sha2::{Digest, Sha256};

use crate::DatashedResult;

pub struct Document {
    pub path: String,
    pub hash: String,
    pub name: String,
    pub size: u64,
    pub mtime: u64,
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
    pub fn from_path<P, Q>(
        path: P,
        data_dir: Q,
    ) -> DatashedResult<(Self, Vec<u8>)>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        let path = path.as_ref().to_path_buf();
        let metadata = path.metadata()?;
        let content = fs::read(&path)?;

        let relpath = path
            .strip_prefix(data_dir)
            .expect("strip prefix")
            .to_str()
            .expect("valid path")
            .into();

        let mtime = metadata
            .modified()
            .ok()
            .and_then(|x| x.duration_since(UNIX_EPOCH).ok())
            .map(|x| x.as_secs())
            .expect("valid mtime");

        let name = path
            .file_stem()
            .and_then(OsStr::to_str)
            .map(ToString::to_string)
            .unwrap_or_default();

        Ok((
            Self {
                path: relpath,
                hash: sha256(&content),
                name,
                size: metadata.st_size(),
                mtime,
            },
            content,
        ))
    }
}
