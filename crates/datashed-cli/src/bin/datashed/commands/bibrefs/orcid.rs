use std::sync::OnceLock;

use bstr::ByteSlice;
use regex::bytes::Regex;

use super::{Matcher, Reference, ReferenceType};

fn orcid_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?ix)(?:https?:\/\/orcid\.org\/)(\d{4}-\d{4}-\d{4}-\d{3}(?:\d|X))").unwrap()
    })
}

#[derive(Debug, Default)]
pub struct OrcidMatcher {
    normalize: bool,
}

impl OrcidMatcher {
    pub fn new(normalize: bool) -> Self {
        Self { normalize }
    }
}

const DIGITS: [char; 11] =
    ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'X'];

fn is_valid(orcid: &str) -> bool {
    let n = orcid.chars().count();

    let total: u64 = orcid
        .chars()
        .take(n - 1)
        .filter_map(|c| {
            if c.is_ascii_digit() {
                Some(((c as u8) - 48) as u64)
            } else {
                None
            }
        })
        .fold(0u64, |acc, digit| (acc + digit) * 2);

    let reminder = total % 11;
    let result = (12 - reminder) % 11;
    let digit = DIGITS[result as usize];

    match orcid.chars().last() {
        // be tolerant if the check digit is in lower case
        Some('x') => digit == 'X',
        Some(c) => c == digit,
        _ => false,
    }
}

impl Matcher for OrcidMatcher {
    fn matches(&self, data: &[u8]) -> Vec<Reference> {
        orcid_re()
            .captures_iter(data.as_ref())
            .filter_map(|caps| {
                let m = caps.get(0).unwrap();
                let (_, [value]) = caps.extract();
                let v = value.to_str().unwrap();

                if is_valid(v) {
                    let value = if self.normalize {
                        v.chars()
                            .map(|c| match c {
                                ch if ch.is_ascii_digit() => ch,
                                'x' | 'X' => 'X',
                                _ => '-',
                            })
                            .collect::<String>()
                    } else {
                        v.to_string()
                    };

                    Some(Reference {
                        reftype: ReferenceType::Orcid,
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

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
