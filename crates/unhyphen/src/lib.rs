use bstr::ByteSlice;
use hashbrown::HashMap;

pub fn unhyphen<B: AsRef<[u8]>>(bytes: B, threshold: u64) -> String {
    let bytes = bytes.as_ref();

    let tokens = tokenize(bytes);
    let freqs = frequencies(&tokens, threshold);

    let mut result = String::with_capacity(bytes.len());
    let mut state = State::Start;
    let mut buf = vec![];

    for token in tokens.iter() {
        match state {
            State::Start => {
                buf.push(token);
                state = match token {
                    Token::Word(_) => State::Word,
                    Token::Whitespace(_) => State::Start,
                    Token::Hyphen(_) => State::Subword,
                }
            }
            State::Subword => {
                buf.push(token);
                state = match token {
                    Token::Word(_) => State::Word,
                    Token::Whitespace(_) => State::Subword,
                    Token::Hyphen(_) => State::Subword,
                };
            }
            State::Word => {
                match token {
                    Token::Word(_) => {
                        if buf.iter().any(|token| token.is_hyphen()) {
                            let temp = buf
                                .iter()
                                .filter(|w| matches!(w, Token::Word(_)))
                                .fold(String::new(), |mut acc, v| {
                                    acc.push_str(v.as_str());
                                    acc
                                });

                            if freqs.contains_key(&temp) {
                                let mut replacement = String::new();
                                replacement.push_str(&prefix(&buf));
                                replacement.push_str(&temp);
                                replacement.push_str(&suffix(&buf));
                                result.push_str(&replacement);
                                buf.clear();
                            }
                        }

                        buf.iter().for_each(|word| {
                            result.push_str(word.as_str())
                        });

                        buf.clear();
                    }
                    Token::Whitespace(_) => {}
                    Token::Hyphen(_) => {
                        state = State::Subword;
                    }
                };

                buf.push(token)
            }
        }
    }

    // TODO: get rid of duplicate code
    if buf.iter().any(|token| token.is_hyphen()) {
        let temp = buf
            .iter()
            .filter(|w| matches!(w, Token::Word(_)))
            .fold(String::new(), |mut acc, v| {
                acc.push_str(v.as_str());
                acc
            });

        if freqs.contains_key(&temp) {
            let mut replacement = String::new();
            replacement.push_str(&prefix(&buf));
            replacement.push_str(&temp);
            replacement.push_str(&suffix(&buf));
            result.push_str(&replacement);
            buf.clear();
        }
    }

    for word in buf.iter() {
        result.push_str(word.as_str());
    }

    result
}

#[derive(Debug, PartialEq)]
enum Token<'a> {
    Word(&'a str),
    Whitespace(&'a str),
    Hyphen(&'a str),
}

impl<'a> Token<'a> {
    #[inline(always)]
    fn is_alphabetic(&self) -> bool {
        match self {
            Self::Word(word) => word.chars().any(char::is_alphabetic),
            _ => false,
        }
    }

    #[inline(always)]
    fn is_hyphen(&self) -> bool {
        matches!(self, Token::Hyphen(_))
    }

    #[inline]
    fn as_str(&self) -> &str {
        match self {
            Self::Word(s) => s,
            Self::Whitespace(s) => s,
            Self::Hyphen(s) => s,
        }
    }
}

impl<'a> From<&'a str> for Token<'a> {
    fn from(bytes: &'a str) -> Self {
        if bytes.chars().all(char::is_whitespace) {
            return Token::Whitespace(bytes);
        }

        match bytes {
            "\u{002d}" | "\u{00ad}" | "\u{2010}" | "\u{2011}" => {
                Token::Hyphen(bytes)
            }
            _ => Token::Word(bytes),
        }
    }
}

#[inline(always)]
fn tokenize<'a>(bytes: &'a [u8]) -> Vec<Token<'a>> {
    bytes.words_with_breaks().map(Token::from).collect()
}

#[inline]
fn prefix(tokens: &Vec<&Token>) -> String {
    let mut prefix = String::new();
    for token in tokens {
        match token {
            Token::Whitespace(s) => prefix.push_str(s),
            Token::Hyphen(s) => prefix.push_str(s),
            Token::Word(_) => break,
        }
    }

    prefix
}

#[inline]
fn suffix(tokens: &Vec<&Token>) -> String {
    let mut suffixes = vec![];
    for token in tokens.iter().rev() {
        match token {
            Token::Whitespace(s) => suffixes.push(s),
            Token::Hyphen(s) => suffixes.push(s),
            Token::Word(_) => break,
        }
    }

    suffixes.iter().rev().fold(String::new(), |mut acc, s| {
        acc.push_str(s);
        acc
    })
}

#[derive(Debug)]
enum State {
    Start,
    Subword,
    Word,
}

fn frequencies(
    tokens: &[Token<'_>],
    threshold: u64,
) -> HashMap<String, u64> {
    let mut state = State::Start;
    let mut words = vec![];
    let mut buf = vec![];

    for token in tokens.iter() {
        match state {
            State::Start => match token {
                Token::Whitespace(_) => continue,
                Token::Word(_) => {
                    state = State::Word;
                    buf.push(token);
                }
                Token::Hyphen(_) => state = State::Subword,
            },

            State::Word => match token {
                Token::Whitespace(_) => continue,
                Token::Word(_) => {
                    words.extend_from_slice(&buf);
                    buf.clear();
                    buf.push(token);
                }
                Token::Hyphen(_) => {
                    state = State::Subword;
                    buf.clear();
                }
            },

            State::Subword => match token {
                Token::Whitespace(_) => continue,
                Token::Word(_) => {
                    debug_assert!(buf.is_empty());
                    state = State::Word;
                }
                Token::Hyphen(_) => {
                    debug_assert!(buf.is_empty());
                }
            },
        }
    }

    words.extend_from_slice(&buf);

    let mut freqs = words
        .iter()
        .filter(|word| word.is_alphabetic())
        .fold(HashMap::new(), |mut acc, value| {
            let value = value.as_str().to_string();
            acc.entry(value)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            acc
        });

    if threshold > 1 {
        freqs.retain(|_, &mut value| value >= threshold);
    }

    freqs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unhypehen() {
        assert_eq!(
            unhyphen(
                "Dies ist ein Test des Programms 'un-hyphen'. Das Binary heißt unhyphen.",
                1,
            ),
            "Dies ist ein Test des Programms 'unhyphen'. Das Binary heißt unhyphen."
        );

        assert_eq!(
            unhyphen(
                "Dies ist ein Test des Programms 'un-hyphen'. Das Binary heißt unhyphen.",
                2,
            ),
            "Dies ist ein Test des Programms 'un-hyphen'. Das Binary heißt unhyphen."
        );
    }

    #[test]
    fn test_token_is_alphabetic() {
        assert!(Token::from("abc").is_alphabetic());
        assert!(Token::from(".b.").is_alphabetic());
        assert!(!Token::from("...").is_alphabetic());
        assert!(!Token::from("-").is_alphabetic());
    }

    #[test]
    fn test_token_is_hyphen() {
        assert!(!Token::from("abc").is_hyphen());
        assert!(!Token::from(".b.").is_hyphen());
        assert!(!Token::from("...").is_hyphen());
        assert!(Token::from("-").is_hyphen());
    }

    #[test]
    fn test_token_as_str() {
        assert_eq!(Token::from("abc").as_str(), "abc");
        assert_eq!(Token::from(".b.").as_str(), ".b.");
        assert_eq!(Token::from("...").as_str(), "...");
        assert_eq!(Token::from("-").as_str(), "-");
    }

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize(b"abc - def  hij."),
            vec![
                Token::Word("abc"),
                Token::Whitespace(" "),
                Token::Hyphen("-"),
                Token::Whitespace(" "),
                Token::Word("def"),
                Token::Whitespace("  "),
                Token::Word("hij"),
                Token::Word(".")
            ]
        );
    }

    #[test]
    fn test_frequencies() {
        let tokens = tokenize(b"abc def abc hij - klm");
        let freqs = frequencies(&tokens, 1);
        assert_eq!(freqs.get("abc"), Some(&2));
        assert_eq!(freqs.get("def"), Some(&1));
        assert_eq!(freqs.len(), 2);

        let tokens = tokenize(b"abc def abc hij - klm");
        let freqs = frequencies(&tokens, 2);
        assert_eq!(freqs.get("abc"), Some(&2));
        assert_eq!(freqs.len(), 1);
    }
}
