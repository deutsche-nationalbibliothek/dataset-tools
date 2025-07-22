use std::sync::OnceLock;

use bstr::ByteSlice;
use regex::bytes::Regex;

use super::{Matcher, Reference, ReferenceType};

fn lcc_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?x)\bLCC:?\s+([A-HJ-NP-VZ][A-Z]{1,2}\d{2,}(?:\.\d+)?(?:-\d{2,}(?:\.\d+))?)(?:\s*\.?[A-Z]\d+)?(?:\s+\d+)?")
            .unwrap()
    })
}

#[derive(Debug, Default)]
pub struct LccMatcher {
    _normalize: bool,
}

impl LccMatcher {
    pub fn new(normalize: bool) -> Self {
        Self {
            _normalize: normalize,
        }
    }
}

impl Matcher for LccMatcher {
    fn matches(&self, data: &[u8]) -> Vec<Reference> {
        lcc_re()
            .captures_iter(data.as_ref())
            .flat_map(|caps| {
                caps.iter()
                    .skip(1)
                    .filter_map(|group| {
                        if let Some(m) = group {
                            let value = m.as_bytes().to_str().unwrap();

                            Some(Reference {
                                reftype: ReferenceType::Lcc,
                                value: value.to_string(),
                                start: m.start(),
                                end: m.end(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcc_default() {
        let matcher = LccMatcher::default();

        let matches = matcher.matches(b"LCC BR145.3 .W54 2016");
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Lcc,
                value: "BR145.3".into(),
                start: 4,
                end: 11,
            }
        );

        let matches = matcher.matches(b"LCC BR145.3 .W54");
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Lcc,
                value: "BR145.3".into(),
                start: 4,
                end: 11,
            }
        );

        let matches = matcher.matches(b"LCC: BR145.3");
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Lcc,
                value: "BR145.3".into(),
                start: 5,
                end: 12,
            }
        );

        let matches = matcher.matches(b"LCC HD9696.2-9696.82");
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Lcc,
                value: "HD9696.2-9696.82".into(),
                start: 4,
                end: 20,
            }
        );
    }
}
