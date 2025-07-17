use std::sync::OnceLock;

use bstr::ByteSlice;
use regex::bytes::Regex;

use super::{Matcher, Reference, ReferenceType};

fn issn_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?ix)ISSN(?::?\s*)?\s(\d{4}[-\u00AD\u2010\u2011\u2014]\d{3}[\dXx])")
            .unwrap()
    })
}

#[derive(Debug, Default)]
pub struct IssnMatcher {
    normalize: bool,
}

impl IssnMatcher {
    pub fn new(normalize: bool) -> Self {
        Self { normalize }
    }
}

const DIGITS: [char; 10] =
    ['_', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn is_valid(issn: &str) -> bool {
    let n = issn.chars().count();
    debug_assert!(n == 9);

    let sum: u64 = issn
        .chars()
        .take(n - 1)
        .filter_map(|c| {
            if c.is_ascii_digit() {
                Some(((c as u8) - 48) as u64)
            } else {
                None
            }
        })
        .zip((2..n).rev())
        .map(|(digit, i)| digit * (i as u64))
        .sum();

    let rem = sum % 11;

    let digit = if rem > 0 {
        let digit = 11 - rem;
        if digit < 10 {
            DIGITS[digit as usize]
        } else {
            'X'
        }
    } else {
        '0'
    };

    match issn.chars().last() {
        // be tolerant if the check digit is in lower case
        Some('x') => digit == 'X',
        Some(c) => c == digit,
        _ => false,
    }
}

impl Matcher for IssnMatcher {
    fn matches(&self, data: &[u8]) -> Vec<Reference> {
        issn_re()
            .captures_iter(data.as_ref())
            .filter_map(|caps| {
                let m = caps.get(0).unwrap();
                let (_, [value]) = caps.extract();
                let v = value.to_str().unwrap();

                if is_valid(v) {
                    let value = if self.normalize {
                        let offset = v
                            .char_indices()
                            .map(|(i, _)| i)
                            .nth_back(3)
                            .unwrap();

                        format!("{}-{}", &v[0..4], &v[offset..])
                    } else {
                        v.to_string()
                    };

                    Some(Reference {
                        reftype: ReferenceType::Issn,
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
    fn test_issn_default() {
        let matcher = IssnMatcher::default();

        let matches = matcher.matches(b"ISSN 0378-5955");
        assert_eq!(matches.len(), 1);

        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Issn,
                value: "0378-5955".into(),
                start: 0,
                end: 14,
            }
        );

        let matches =
            matcher.matches("ISSN 0378\u{2010}5955".as_bytes());
        assert_eq!(matches.len(), 1);

        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Issn,
                value: "0378\u{2010}5955".into(),
                start: 0,
                end: 16,
            }
        );

        let matches =
            matcher.matches("ISSN 0378\u{2011}5955".as_bytes());
        assert_eq!(matches.len(), 1);

        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Issn,
                value: "0378\u{2011}5955".into(),
                start: 0,
                end: 16,
            }
        );

        let matches =
            matcher.matches("ISSN 0378\u{2014}5955".as_bytes());
        assert_eq!(matches.len(), 1);

        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Issn,
                value: "0378\u{2014}5955".into(),
                start: 0,
                end: 16,
            }
        );

        let matches = matcher.matches("ISSN 0378 5955".as_bytes());
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_issn_normalize() {
        let matcher = IssnMatcher { normalize: true };

        let matches = matcher.matches(b"ISSN 0378-5955");
        assert_eq!(matches.len(), 1);

        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Issn,
                value: "0378-5955".into(),
                start: 0,
                end: 14,
            }
        );

        let matches =
            matcher.matches("ISSN 0378\u{2010}5955".as_bytes());
        assert_eq!(matches.len(), 1);

        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Issn,
                value: "0378-5955".into(),
                start: 0,
                end: 16,
            }
        );

        let matches =
            matcher.matches("ISSN 0378\u{2011}5955".as_bytes());
        assert_eq!(matches.len(), 1);

        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Issn,
                value: "0378-5955".into(),
                start: 0,
                end: 16,
            }
        );

        let matches =
            matcher.matches("ISSN 0378\u{2014}5955".as_bytes());
        assert_eq!(matches.len(), 1);

        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Issn,
                value: "0378-5955".into(),
                start: 0,
                end: 16,
            }
        );
    }

    #[test]
    fn test_issn_invalid() {
        let matcher = IssnMatcher::default();

        let matches = matcher.matches("ISSN 0378 5955".as_bytes());
        assert_eq!(matches.len(), 0);

        let matches = matcher.matches("ISSN 0378-5956".as_bytes());
        assert_eq!(matches.len(), 0);
    }
}
