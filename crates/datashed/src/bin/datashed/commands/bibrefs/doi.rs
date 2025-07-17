use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::OnceLock;

use bstr::ByteSlice;
use flate2::read::GzDecoder;
use hashbrown::HashSet;
use regex::bytes::Regex;
use walkdir::WalkDir;

use super::{Matcher, Reference, ReferenceType};
use crate::prelude::*;

fn doi_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?ix)\b(?:doi[:\ ]|https://(?:dx\.)?doi.org/)?(10\.[0-9]{3,}(?:\.[0-9]+)*/[[:graph:]]+)\b",
        )
        .unwrap()
    })
}

#[derive(Debug, Default)]
pub struct DoiMatcher {
    crossrefs: HashSet<String>,
    normalize: bool,
}

const PBAR_CROSSREF: &str = "Process Crossref data: {human_pos} ({percent}%) | \
        elapsed: {elapsed_precise}{msg}";

impl DoiMatcher {
    pub fn new(normalize: bool, crossref: Option<PathBuf>) -> Self {
        let xrefs = if let Some(path) = crossref {
            let files = WalkDir::new(path)
                .into_iter()
                .filter_map(Result::ok)
                .map(|dirent| dirent.into_path())
                .filter(|path| {
                    path.to_str()
                        .map(|s| s.ends_with(".jsonl.gz"))
                        .unwrap_or(false)
                })
                .collect::<Vec<_>>();

            let pbar = ProgressBarBuilder::new(PBAR_CROSSREF, false)
                .len(files.len() as u64)
                .build();

            files
                .par_iter()
                .progress_with(pbar)
                .map(|path| -> HashSet<String> {
                    let mut dois = HashSet::new();

                    let mut rdr =
                        GzDecoder::new(File::open(path).unwrap());
                    let mut buf = String::new();
                    let _ = rdr.read_to_string(&mut buf).unwrap();

                    for line in buf.lines() {
                        let v: serde_json::Value =
                            serde_json::from_str(line).unwrap();

                        let value = if normalize {
                            v["DOI"].as_str().unwrap().to_lowercase()
                        } else {
                            v["DOI"].to_string()
                        };

                        dois.insert(value);
                    }

                    dois
                })
                .reduce(HashSet::new, |mut acc, e| {
                    acc.extend(e);
                    acc
                })
        } else {
            HashSet::new()
        };

        Self {
            normalize,
            crossrefs: xrefs,
        }
    }
}

impl Matcher for DoiMatcher {
    fn matches(&self, data: &[u8]) -> Vec<Reference> {
        doi_re()
            .captures_iter(data)
            .filter_map(|caps| {
                let m = caps.get(0).unwrap();
                let (_, [value]) = caps.extract();
                let v = value.to_str().unwrap();

                let value = if self.normalize {
                    v.to_lowercase()
                } else {
                    v.to_string()
                };

                if self.crossrefs.is_empty()
                    || self.crossrefs.contains(&value)
                {
                    Some(Reference {
                        reftype: ReferenceType::Doi,
                        start: m.start(),
                        end: m.end(),
                        value,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {}
