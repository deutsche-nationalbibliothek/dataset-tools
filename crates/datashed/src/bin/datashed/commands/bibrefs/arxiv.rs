use std::sync::OnceLock;

use bstr::ByteSlice;
use regex::bytes::Regex;

use super::{Matcher, Reference, ReferenceType};

fn arxiv_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?ix)
                (?:(?:arXiv:)|(?:https://doi\.org/10\.48550/arXiv\.))
                (\d{2}(?:0[1-9]|1[0-2])\.\d+(?:v[1-9]\d*)?)
            ",
        )
        .unwrap()
    })
}

#[derive(Debug, Default)]
pub struct ArxivMatcher {
    _normalize: bool,
}

impl ArxivMatcher {
    pub fn new(normalize: bool) -> Self {
        Self {
            _normalize: normalize,
        }
    }
}

impl Matcher for ArxivMatcher {
    fn matches(&self, data: &[u8]) -> Vec<Reference> {
        arxiv_re()
            .captures_iter(data.as_ref())
            .filter_map(|caps| {
                let m = caps.get(0).unwrap();
                let (_, [value]) = caps.extract();
                let v = value.to_str().unwrap();

                Some(Reference {
                    reftype: ReferenceType::Arxiv,
                    start: m.start(),
                    end: m.end(),
                    value: v.to_string(),
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arxiv() {
        let arxiv = ArxivMatcher::default();
        let matches = arxiv.matches(b"arXiv:0706.1234v1 [math.FA]");
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Arxiv,
                value: "0706.1234v1".into(),
                start: 0,
                end: 17,
            }
        );

        let arxiv = ArxivMatcher::default();
        let matches = arxiv.matches(b"arXiv:1501.00001");
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Arxiv,
                value: "1501.00001".into(),
                start: 0,
                end: 16,
            }
        );

        let arxiv = ArxivMatcher::default();
        let matches = arxiv.matches(b"arXiv:0706.0001");
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Arxiv,
                value: "0706.0001".into(),
                start: 0,
                end: 15,
            }
        );

        let arxiv = ArxivMatcher::default();
        let matches =
            arxiv.matches(b"https://doi.org/10.48550/arXiv.2202.01037");
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0],
            Reference {
                reftype: ReferenceType::Arxiv,
                value: "2202.01037".into(),
                start: 0,
                end: 41,
            }
        );
    }
}
