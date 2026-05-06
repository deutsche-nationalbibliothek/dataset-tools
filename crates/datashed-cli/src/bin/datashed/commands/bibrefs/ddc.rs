use std::sync::OnceLock;

use bstr::ByteSlice;
use regex::bytes::Regex;

use super::{Matcher, Reference, ReferenceType};

fn ddc_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?x)\b(?:DDC:?\s*)\s(\d{3}(?:\[\.\d+\]|[./]?\d)+)")
            .unwrap()
    })
}

#[derive(Debug, Default)]
pub struct DdcMatcher {
    normalize: bool,
}

impl DdcMatcher {
    pub fn new(normalize: bool) -> Self {
        Self { normalize }
    }
}

impl Matcher for DdcMatcher {
    fn matches(&self, data: &[u8]) -> Vec<Reference> {
        ddc_re()
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
                                    .filter(|c| *c != '/')
                                    .collect()
                            } else {
                                value.to_string()
                            };

                            Some(Reference {
                                reftype: ReferenceType::Ddc,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ddc_default() {
        let matcher = DdcMatcher::default();

        let matches = matcher.matches(b"DDC 306.8709417--dc23");
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Ddc,
                value: "306.8709417".into(),
                start: 4,
                end: 15,
            }
        );

        let matches = matcher.matches(b"DDC 511[.22]");
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Ddc,
                value: "511[.22]".into(),
                start: 4,
                end: 12,
            }
        );

        let matches = matcher.matches("DDC 616.99/463–dc23".as_bytes());
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Ddc,
                value: "616.99/463".into(),
                start: 4,
                end: 14,
            }
        );

        let matches =
            matcher.matches("DDC 616.9/04231—dc23".as_bytes());
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Ddc,
                value: "616.9/04231".into(),
                start: 4,
                end: 15,
            }
        );
    }

    #[test]
    fn test_ddc_normalize() {
        let matcher = DdcMatcher::new(true);

        let matches = matcher.matches(b"DDC 306.8709417--dc23");
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Ddc,
                value: "306.8709417".into(),
                start: 4,
                end: 15,
            }
        );

        let matches = matcher.matches(b"DDC 511[.22]");
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Ddc,
                value: "511[.22]".into(),
                start: 4,
                end: 12,
            }
        );

        let matches = matcher.matches("DDC 616.99/463–dc23".as_bytes());
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Ddc,
                value: "616.99463".into(),
                start: 4,
                end: 14,
            }
        );

        let matches =
            matcher.matches("DDC 616.9/04231—dc23".as_bytes());
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Ddc,
                value: "616.904231".into(),
                start: 4,
                end: 15,
            }
        );
    }

    // #[test]
    // fn test_ddc_invalid() {
    //     let matcher = DdcMatcher::default();
    //     let matches = matcher.matches("DDC
    // 624.074:678.067".as_bytes());     assert_eq!(matches.len(),
    // 0); }
}
