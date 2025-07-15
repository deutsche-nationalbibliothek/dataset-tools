use std::sync::OnceLock;

use bstr::ByteSlice;
use regex::bytes::Regex;

use super::{Matcher, Reference, ReferenceType};

fn ismn_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?ix)ISMN(?::?\s*)\s(
                (?:M|979[-\ \u00AD\u2010\u2011\u2014]0)
                [-\ \u00AD\u2010\u2011\u2014]\d{3,7}
                [-\ \u00AD\u2010\u2011\u2014]\d{1,5}
                [-\ \u00AD\u2010\u2011\u2014][\dX])",
        )
        .unwrap()
    })
}

#[derive(Debug, Default)]
pub struct IsmnMatcher {
    pub normalize: bool,
}

fn is_valid(ismn: &str) -> bool {
    let n = ismn.chars().count();
    if n != 17 && n != 13 {
        return false;
    }

    let mut sum: u64 = ismn
        .chars()
        .filter_map(|c| {
            if c.is_ascii_digit() {
                Some(((c as u8) - 48) as u64)
            } else {
                None
            }
        })
        .zip([1u64, 3].iter().cycle())
        .map(|(digit, factor)| digit * factor)
        .sum();

    if n == 13 {
        sum += 39;
    }

    sum % 10 == 0
}

impl Matcher for IsmnMatcher {
    fn matches(&self, data: &[u8]) -> Vec<Reference> {
        ismn_re()
            .captures_iter(data)
            .filter_map(|caps| {
                let m = caps.get(0).unwrap();
                let (_, [value]) = caps.extract();
                let v = value.to_str().unwrap();

                if is_valid(v) {
                    let value = if self.normalize {
                        v.chars()
                            .map(|c| {
                                if !c.is_ascii_digit() {
                                    '-'
                                } else {
                                    c
                                }
                            })
                            .collect()
                    } else {
                        v.to_string()
                    };

                    Some(Reference {
                        reftype: ReferenceType::Ismn,
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
mod tests {
    use super::*;

    #[test]
    fn test_ismn_default() {
        let matcher = IsmnMatcher::default();

        let matches = matcher.matches(b"ISMN 979-0-2306-7118-7");
        assert_eq!(matches.len(), 1);

        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Ismn,
                value: "979-0-2306-7118-7".into(),
                start: 0,
                end: 22,
            }
        );

        let matches = matcher.matches(b"ISMN: 979-0-2306-7118-7");
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Ismn,
                value: "979-0-2306-7118-7".into(),
                start: 0,
                end: 23,
            }
        );

        let matches = matcher.matches(
            "ISMN 979\u{00AD}0\u{00AD}2306\u{00AD}7118\u{00AD}7"
                .as_bytes(),
        );
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Ismn,
                value: "979\u{00AD}0\u{00AD}2306\u{00AD}7118\u{00AD}7"
                    .into(),
                start: 0,
                end: 26,
            }
        );

        let matches = matcher.matches(
            "ISMN 979\u{2010}0\u{2010}2306\u{2010}7118\u{2010}7"
                .as_bytes(),
        );
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Ismn,
                value: "979\u{2010}0\u{2010}2306\u{2010}7118\u{2010}7"
                    .into(),
                start: 0,
                end: 30,
            }
        );

        let matches = matcher.matches(
            "ISMN 979\u{2011}0\u{2011}2306\u{2011}7118\u{2011}7"
                .as_bytes(),
        );
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Ismn,
                value: "979\u{2011}0\u{2011}2306\u{2011}7118\u{2011}7"
                    .into(),
                start: 0,
                end: 30,
            }
        );

        let matches = matcher.matches(
            "ISMN 979\u{2014}0\u{2014}2306\u{2014}7118\u{2014}7"
                .as_bytes(),
        );
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Ismn,
                value: "979\u{2014}0\u{2014}2306\u{2014}7118\u{2014}7"
                    .into(),
                start: 0,
                end: 30,
            }
        );
    }

    #[test]
    fn test_ismn_normalize() {
        let matcher = IsmnMatcher { normalize: true };

        let matches = matcher.matches(b"ISMN 979-0-2306-7118-7");
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Ismn,
                value: "979-0-2306-7118-7".into(),
                start: 0,
                end: 22,
            }
        );

        let matches = matcher.matches(
            "ISMN 979\u{00AD}0\u{00AD}2306\u{00AD}7118\u{00AD}7"
                .as_bytes(),
        );
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Ismn,
                value: "979-0-2306-7118-7".into(),
                start: 0,
                end: 26,
            }
        );

        let matches = matcher.matches(
            "ISMN 979\u{2010}0\u{2010}2306\u{2010}7118\u{2010}7"
                .as_bytes(),
        );
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Ismn,
                value: "979-0-2306-7118-7".into(),
                start: 0,
                end: 30,
            }
        );

        let matches = matcher.matches(
            "ISMN 979\u{2011}0\u{2011}2306\u{2011}7118\u{2011}7"
                .as_bytes(),
        );
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Ismn,
                value: "979-0-2306-7118-7".into(),
                start: 0,
                end: 30,
            }
        );

        let matches = matcher.matches(
            "ISMN 979\u{2014}0\u{2014}2306\u{2014}7118\u{2014}7"
                .as_bytes(),
        );
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Ismn,
                value: "979-0-2306-7118-7".into(),
                start: 0,
                end: 30,
            }
        );
    }

    #[test]
    fn test_ismn_invalid() {
        let matcher = IsmnMatcher::default();

        let matches = matcher.matches(b"ISMN 979-0-2306-7118-8");
        assert_eq!(matches.len(), 0);

        let matches = matcher.matches(b"ISMN M-2306-7118-8");
        assert_eq!(matches.len(), 0);

        let matcher = IsmnMatcher { normalize: true };

        let matches = matcher.matches(b"ISMN 979-0-2306-7118-8");
        assert_eq!(matches.len(), 0);

        let matches = matcher.matches(b"ISMN M-2306-7118-8");
        assert_eq!(matches.len(), 0);
    }
}
