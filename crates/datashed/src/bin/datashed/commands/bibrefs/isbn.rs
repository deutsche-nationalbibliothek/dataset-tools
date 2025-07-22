use std::str::FromStr;
use std::sync::OnceLock;

use bstr::ByteSlice;
use isbn::Isbn;
use regex::bytes::Regex;

use super::{Matcher, Reference, ReferenceType};

fn isbn_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?ix)
                (?:e-)?ISBN(?:-1[03])?(?::?\s*)?
                (?:\s*\(\w+\)\s*)?\s
                ((?:97[89][-\ ]?)?
                 \d{1,5}[-\ ]?
                 (?:\d+[-\ ]?){2}
                 (?:\d|X))",
        )
        .unwrap()
    })
}

#[derive(Debug, Default)]
pub struct IsbnMatcher {
    normalize: bool,
}

impl IsbnMatcher {
    pub fn new(normalize: bool) -> Self {
        Self { normalize }
    }
}

impl Matcher for IsbnMatcher {
    fn matches(&self, data: &[u8]) -> Vec<Reference> {
        isbn_re()
            .captures_iter(data)
            .filter_map(|caps| {
                let m = caps.get(0).unwrap();
                let (_, [value]) = caps.extract();
                let v = value.to_str().unwrap();

                if let Ok(isbn) = Isbn::from_str(v) {
                    let value = if self.normalize {
                        isbn.hyphenate().ok()?.to_string()
                    } else {
                        v.to_string()
                    };

                    Some(Reference {
                        reftype: ReferenceType::Isbn,
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
    fn test_isbn13() {
        let isbn = IsbnMatcher::default();

        let matches = isbn.matches(b"ISBN 9780596528126");
        assert_eq!(matches.len(), 1);

        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Isbn,
                value: "9780596528126".into(),
                start: 0,
                end: 18,
            }
        );

        let matches = isbn.matches(b"ISBN 978-0-596-52812-6");
        assert_eq!(matches.len(), 1);

        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Isbn,
                value: "978-0-596-52812-6".into(),
                start: 0,
                end: 22,
            }
        );
    }

    #[test]
    fn test_isbn13_normalize() {
        let isbn = IsbnMatcher { normalize: true };
        let matches = isbn.matches(b"ISBN 978-0-596-52812-6");
        assert_eq!(matches.len(), 1);

        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Isbn,
                value: "978-0-596-52812-6".into(),
                start: 0,
                end: 22,
            }
        )
    }

    #[test]
    fn test_isbn10() {
        let isbn = IsbnMatcher::default();
        let matches = isbn.matches(b"ISBN 3518293036");
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Isbn,
                value: "3518293036".into(),
                start: 0,
                end: 15,
            }
        );

        let matches = isbn.matches(b"ISBN 3-518-29303-6");
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Isbn,
                value: "3-518-29303-6".into(),
                start: 0,
                end: 18,
            }
        );
    }

    #[test]
    fn test_isbn10_normalize() {
        let isbn = IsbnMatcher { normalize: true };
        let matches = isbn.matches(b"  ISBN 3518293036");
        assert_eq!(matches.len(), 1);

        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Isbn,
                value: "3-518-29303-6".into(),
                start: 2,
                end: 17,
            }
        )
    }
}
