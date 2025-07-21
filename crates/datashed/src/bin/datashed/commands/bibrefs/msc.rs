use std::sync::OnceLock;

use bstr::ByteSlice;
use regex::bytes::Regex;

use super::{Matcher, Reference, ReferenceType};

fn msc_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?ix)
                (?:Mathematical\sSubject\sClassification|MSC)(?:\s*20[012]0\s*)?(?:\s*:?\s*)\s
                (?:(?:Primary\s*)?(\d{2}[A-Z]\d{2})(?:\s*[.·,;]?\s*)?)
                (?:(?:Secondary\s*)?(\d{2}[A-Z]\d{2})(?:\s*[.·,;]?\s*)?)?
                (?:(\d{2}[A-Z]\d{2})(?:\s*[.·,;]?\s*)?)?
                (?:(\d{2}[A-Z]\d{2})(?:\s*[.·,;]?\s*)?)?
                (?:(\d{2}[A-Z]\d{2})(?:\s*[.·,;]?\s*)?)?
                (?:(\d{2}[A-Z]\d{2})(?:\s*[.·,;]?\s*)?)?
                (?:(\d{2}[A-Z]\d{2})(?:\s*[.·,;]?\s*)?)?
                (?:(\d{2}[A-Z]\d{2})(?:\s*[.·,;]?\s*)?)?
                (?:(\d{2}[A-Z]\d{2})(?:\s*[.·,;]?\s*)?)?
                (?:(\d{2}[A-Z]\d{2})(?:\s*[.·,;]?\s*)?)?
            ",
        )
        .unwrap()
    })
}

#[derive(Debug, Default)]
pub struct MscMatcher {}

impl MscMatcher {
    pub fn new(_normalize: bool) -> Self {
        Self {}
    }
}

impl Matcher for MscMatcher {
    fn matches(&self, data: &[u8]) -> Vec<Reference> {
        msc_re()
            .captures_iter(data.as_ref())
            .flat_map(|caps| {
                caps.iter()
                    .skip(1)
                    .filter_map(|group| {
                        if let Some(m) = group {
                            let value = m
                                .as_bytes()
                                .to_str()
                                .unwrap()
                                .to_string();

                            Some(Reference {
                                reftype: ReferenceType::Msc,
                                start: m.start(),
                                end: m.end(),
                                value,
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
