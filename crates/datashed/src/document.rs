use std::ffi::OsStr;
use std::fmt::Write;
use std::fs::{self};
use std::os::linux::fs::MetadataExt;
use std::path::Path;
use std::sync::OnceLock;
use std::time::UNIX_EPOCH;

use bstr::ByteSlice;
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use sha2::{Digest, Sha256};

use crate::DatashedResult;

pub struct Document {
    pub path: String,
    pub hash: String,
    pub name: String,
    pub lang_code: Option<String>,
    pub lang_score: Option<f64>,
    pub size: u64,
    pub alpha: f64,
    pub mtime: u64,
}

fn language_detector() -> &'static LanguageDetector {
    static DETECTOR: OnceLock<LanguageDetector> = OnceLock::new();
    DETECTOR.get_or_init(|| {
        if cfg!(test) {
            LanguageDetectorBuilder::from_languages(&[
                Language::German,
                Language::English,
            ])
            .with_low_accuracy_mode()
            .build()
        } else {
            LanguageDetectorBuilder::from_all_languages().build()
        }
    })
}

pub fn sha256<T: AsRef<[u8]>>(data: T) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);

    let hash = hasher.finalize();
    hash.iter().take(6).fold(String::new(), |mut out, b| {
        let _ = write!(out, "{b:02x}");
        out
    })
}

#[inline]
fn alpha<T: AsRef<[u8]>>(data: T) -> f64 {
    let data = data.as_ref();

    let total = data.chars().count() as f64;
    if total <= 0.0 {
        return 0.0;
    }

    let alpha =
        data.chars().filter(|c: &char| c.is_alphabetic()).count()
            as f64;

    alpha / total
}

fn lang<T: Into<String>>(text: T) -> Option<(String, f64)> {
    let lang = language_detector()
        .compute_language_confidence_values(text)
        .into_iter()
        .next();

    if let Some((code, score)) = lang {
        let code = match code {
            Language::Albanian => "alb".to_string(),
            Language::Armenian => "arm".to_string(),
            Language::Basque => "baq".to_string(),
            Language::Chinese => "chi".to_string(),
            Language::Czech => "cze".to_string(),
            Language::Dutch => "dut".to_string(),
            Language::French => "fre".to_string(),
            Language::Georgian => "geo".to_string(),
            Language::German => "ger".to_string(),
            Language::Greek => "gre".to_string(),
            Language::Macedonian => "mac".to_string(),
            Language::Malay => "may".to_string(),
            Language::Maori => "mao".to_string(),
            Language::Persian => "per".to_string(),
            Language::Romanian => "rum".to_string(),
            Language::Slovak => "slo".to_string(),
            Language::Welsh => "wel".to_string(),
            lang => lang.iso_code_639_3().to_string(),
        };

        Some((code, score))
    } else {
        None
    }
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
        let data = fs::read(&path)?;
        let content = data.to_str().unwrap();

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

        let (lang_code, lang_score) =
            if let Some((code, score)) = lang(content) {
                (Some(code), Some(score))
            } else {
                (None, None)
            };

        Ok((
            Self {
                path: relpath,
                hash: sha256(&data),
                name,
                lang_code,
                lang_score,
                size: metadata.st_size(),
                alpha: alpha(&data),
                mtime,
            },
            data,
        ))
    }
}
