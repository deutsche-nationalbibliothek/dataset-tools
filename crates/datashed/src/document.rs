use std::ffi::OsStr;
use std::fmt::Write;
use std::fs::{self};
use std::os::linux::fs::MetadataExt;
use std::path::Path;
use std::sync::{LazyLock, OnceLock};
use std::time::UNIX_EPOCH;

use bstr::{BStr, ByteSlice};
use hashbrown::HashMap;
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use ndarray::{Array1, Zip};
use sha2::{Digest, Sha256};
use unicode_normalization::UnicodeNormalization;

use crate::{DatashedResult, Doctype};

pub struct Document {
    pub path: String,
    pub hash: String,
    pub name: String,
    pub doctype: Option<Doctype>,
    pub lang_code: Option<String>,
    pub lang_score: Option<f64>,
    pub chars: u64,
    pub size: u64,
    pub alpha: f64,
    pub lfreq: Option<f64>,
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

fn lfreq(code: &str, data: &BStr) -> Option<f64> {
    const ALPHABET_GER: [char; 30] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
        'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
        'y', 'z', 'ß', 'ä', 'ö', 'ü',
    ];

    const ALPHABET_ENG: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
        'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
        'y', 'z',
    ];

    static LFREQ_GER: LazyLock<Array1<f64>> = LazyLock::new(|| {
        Array1::from_vec(vec![
            0.06006, 0.02148, 0.02690, 0.04718, 0.16006, 0.01832,
            0.03064, 0.04249, 0.07752, 0.00297, 0.01536, 0.03787,
            0.02798, 0.09660, 0.02684, 0.01049, 0.00028, 0.07737,
            0.06343, 0.06369, 0.03820, 0.00918, 0.01427, 0.00051,
            0.00107, 0.01237, 0.00170, 0.00548, 0.00269, 0.00683,
        ])
    });

    static LFREQ_ENG: LazyLock<Array1<f64>> = LazyLock::new(|| {
        Array1::from_vec(vec![
            0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228,
            0.02015, 0.06094, 0.06966, 0.00253, 0.01772, 0.04025,
            0.02406, 0.06749, 0.07507, 0.01929, 0.00950, 0.05987,
            0.06327, 0.09056, 0.02758, 0.00978, 0.02360, 0.00250,
            0.01974, 0.00074,
        ])
    });

    fn lfreq_inner(
        data: &BStr,
        alphabet: &[char],
        frequencies: &Array1<f64>,
    ) -> Option<f64> {
        let freqs = data
            .chars()
            .nfc()
            .to_string()
            .to_lowercase()
            .chars()
            .filter(|c| alphabet.contains(c))
            .fold(HashMap::new(), |mut freqs, value| {
                freqs
                    .entry(value)
                    .and_modify(|entry| *entry += 1)
                    .or_insert(1);
                freqs
            });

        let n = freqs.values().sum::<u64>();
        let x = if n > 0 {
            Array1::from_iter(
                alphabet
                    .iter()
                    .map(|c| *freqs.get(c).unwrap_or(&0) as f64),
            ) / n as f64
        } else {
            Array1::zeros(alphabet.len())
        };

        let mut diff: Array1<f64> = Array1::zeros(alphabet.len());
        Zip::from(&mut diff).and(&x).and(frequencies).for_each(
            |diff, &x, &y| {
                *diff = (x - y).powi(2);
            },
        );

        Some(diff.sum().sqrt())
    }

    match code {
        "ger" => lfreq_inner(data, &ALPHABET_GER, &LFREQ_GER),
        "eng" => lfreq_inner(data, &ALPHABET_ENG, &LFREQ_ENG),
        _ => None,
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
            .strip_prefix(&data_dir)
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

        let lfreq = lang_code
            .as_ref()
            .and_then(|code| lfreq(code, data.as_bstr()));

        let doctype =
            Doctype::try_from(path.strip_prefix(&data_dir).unwrap())
                .ok();

        Ok((
            Self {
                path: relpath,
                hash: sha256(&data),
                name,
                doctype,
                lang_code,
                lang_score,
                chars: data.chars().count() as u64,
                size: metadata.st_size(),
                alpha: alpha(&data),
                lfreq,
                mtime,
            },
            data,
        ))
    }
}
