/// TODO: 666.113'47'22:666.1.031:666.1.032:666.1.038.7
use std::sync::OnceLock;

use bstr::ByteSlice;
use regex::bytes::Regex;

use super::{Matcher, Reference, ReferenceType};

fn udc_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?x)UDC\s*
                (\d+(?:(?:\s*\.\s*)?\d+)+)
                (?:\s*\+\s*(\d+(?:(?:\s*\.\s*)?\d+)+))
                (?:\s*\+\s*(\d+(?:(?:\s*\.\s*)?\d+)+))?
                (?:\s*\+\s*(\d+(?:(?:\s*\.\s*)?\d+)+))?
                (?:\s*\+\s*(\d+(?:(?:\s*\.\s*)?\d+)+))?",
        )
        .unwrap()
    })
}

#[derive(Debug, Default)]
pub struct UdcMatcher {
    normalize: bool,
}

impl UdcMatcher {
    pub fn new(normalize: bool) -> Self {
        Self { normalize }
    }
}

impl Matcher for UdcMatcher {
    fn matches(&self, data: &[u8]) -> Vec<Reference> {
        udc_re()
            .captures_iter(data.as_ref())
            .flat_map(|caps| {
                caps.iter()
                    .skip(1)
                    .filter_map(|group| {
                        if let Some(m) = group {
                            let value = m.as_bytes().to_str().unwrap();
                            let value = if self.normalize {
                                value
                                    .chars()
                                    .filter(|c| {
                                        !c.is_ascii_whitespace()
                                    })
                                    .collect()
                            } else {
                                value.to_string()
                            };

                            Some(Reference {
                                reftype: ReferenceType::Udc,
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
