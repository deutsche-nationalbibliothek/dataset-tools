use std::sync::OnceLock;

use bstr::ByteSlice;
use regex::bytes::Regex;

use super::{Matcher, Reference, ReferenceType};

fn jel_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?x)JEL(?::?\s*)\s
                (?:(?:[,;]?\s*)?([A-Z]\d{1,2}))
                (?:(?:[,;]?\s*)?([A-Z]\d{1,2}))?
                (?:(?:[,;]?\s*)?([A-Z]\d{1,2}))?",
        )
        .unwrap()
    })
}

#[derive(Debug, Default)]
pub struct JelMatcher {}

impl JelMatcher {
    pub fn new(_normalize: bool) -> Self {
        Self {}
    }
}

impl Matcher for JelMatcher {
    fn matches(&self, data: &[u8]) -> Vec<Reference> {
        jel_re()
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
                                reftype: ReferenceType::Jel,
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
