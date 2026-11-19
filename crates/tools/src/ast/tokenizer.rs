pub trait Tokenizer {
    fn tokenize<'a>(&self, input: &'a str) -> Result<TokenizedDocument<'a>, TokenizerError>;
}
#[derive(Debug, thiserror::Error)]
pub enum TokenizerError {
    #[error("Empty Input")]
    EmptyInput,
}

type TokenLine<'a> = Box<[Token<'a>]>;
type TokenizedDocument<'a> = Box<[TokenLine<'a>]>;
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, strum::EnumIs, strum::VariantNames, strum::AsRefStr,
)]
pub enum Token<'a> {
    Delimiter(char),
    Text(&'a str),
}
pub(crate) mod generic_tokenizer {

    use std::marker::PhantomData;

    use crate::ast::tokenizer::{Token, TokenizedDocument};

    use super::{Tokenizer, TokenizerError};

    #[derive(Debug, Clone)]
    pub struct GenericTokenizer<T>
    where
        T: GenericCalculatorPolicy,
    {
        // I was thinking, that GenericTokenizer would be a component to other tokenizers
        // And the other tokenizers do their own const lists
        delimiters: &'static [char],
        behavior: PhantomData<fn() -> T>,
    }

    impl<T> GenericTokenizer<T>
    where
        T: GenericCalculatorPolicy,
    {
        pub const fn new(delimiters: &'static [char]) -> Self {
            Self {
                delimiters,
                behavior: PhantomData,
            }
        }
    }
    impl<T: GenericCalculatorPolicy> Tokenizer for GenericTokenizer<T> {
        /// Splits each line on configured delimiters, retaining whitespace in text.
        /// Empty lines are retained; line endings follow `str::lines` semantics.
        fn tokenize<'a>(&self, input: &'a str) -> Result<TokenizedDocument<'a>, TokenizerError> {
            if input.is_empty() {
                return Err(TokenizerError::EmptyInput);
            }

            Ok(input
                .lines()
                .map(|line| {
                    let mut tokens = Vec::new();
                    let mut start = 0;
                    for (index, character) in line.char_indices() {
                        if self.delimiters.contains(&character) {
                            if start < index {
                                tokens.push(Token::Text(&line[start..index]));
                            }
                            if T::PRESERVE_DELIMITERS {
                                tokens.push(Token::Delimiter(character));
                            }
                            start = index + character.len_utf8();
                        }
                    }
                    if start < line.len() {
                        tokens.push(Token::Text(&line[start..]));
                    }
                    tokens.into_boxed_slice()
                })
                .collect::<TokenizedDocument>())
        }
    }
    // ----------- Policy Stuff ------------------------
    mod sealed {
        pub trait Sealed {}
    }
    pub trait GenericCalculatorPolicy: self::sealed::Sealed {
        const PRESERVE_DELIMITERS: bool;
    }
    /// Defines a sealed policy, with crate visibility by default.
    /// Seagull because they're always "mine".
    macro_rules! seagull_structs {
        ($name:ident, $trait:ident, $preserve:expr) => {
            seagull_structs!($name, pub, $trait, $preserve);
        };
        ($name:ident, $visibility:vis, $trait:ident, $preserve:expr) => {
            #[derive(Debug, Clone, Copy)]
            $visibility struct $name;
            impl self::sealed::Sealed for $name {}
            impl $trait for $name {
                const PRESERVE_DELIMITERS: bool = $preserve;
            }
        }
    }
    seagull_structs!(Preserve, GenericCalculatorPolicy, true);
    seagull_structs!(Discard, GenericCalculatorPolicy, false);
}

#[cfg(test)]
mod tests {
    use super::generic_tokenizer::{Discard, GenericTokenizer, Preserve};
    use super::{Token, Tokenizer, TokenizerError};

    #[test]
    fn preserves_attached_and_adjacent_delimiters() {
        let document = GenericTokenizer::<Preserve>::new(&[';', ','])
            .tokenize(";a;,b;")
            .unwrap();
        assert_eq!(
            document[0].as_ref(),
            &[
                Token::Delimiter(';'),
                Token::Text("a"),
                Token::Delimiter(';'),
                Token::Delimiter(','),
                Token::Text("b"),
                Token::Delimiter(';'),
            ]
        );
    }

    #[test]
    fn policies_have_identical_text_boundaries_and_whitespace() {
        let input = " a ; β,, c ;";
        let preserved = GenericTokenizer::<Preserve>::new(&[';', ','])
            .tokenize(input)
            .unwrap();
        let discarded = GenericTokenizer::<Discard>::new(&[';', ','])
            .tokenize(input)
            .unwrap();
        let text: Vec<_> = preserved[0]
            .iter()
            .copied()
            .filter(|token| matches!(token, Token::Text(_)))
            .collect();
        assert_eq!(discarded[0].as_ref(), text.as_slice());
        assert_eq!(
            text,
            vec![Token::Text(" a "), Token::Text(" β"), Token::Text(" c ")]
        );
    }

    #[test]
    fn handles_unicode_delimiters_and_text() {
        let document = GenericTokenizer::<Preserve>::new(&['🍍'])
            .tokenize("é🍍猫🍍")
            .unwrap();
        assert_eq!(
            document[0].as_ref(),
            &[
                Token::Text("é"),
                Token::Delimiter('🍍'),
                Token::Text("猫"),
                Token::Delimiter('🍍'),
            ]
        );
    }

    #[test]
    fn retains_empty_lines_and_accepts_crlf() {
        let document = GenericTokenizer::<Discard>::new(&[';'])
            .tokenize("a\r\n\r\n;;\nb\n")
            .unwrap();
        assert_eq!(document.len(), 4);
        assert_eq!(document[0].as_ref(), &[Token::Text("a")]);
        assert!(document[1].is_empty());
        assert!(document[2].is_empty());
        assert_eq!(document[3].as_ref(), &[Token::Text("b")]);
    }

    #[test]
    fn no_delimiters_keeps_the_whole_line() {
        let document = GenericTokenizer::<Preserve>::new(&[])
            .tokenize(" a b ")
            .unwrap();
        assert_eq!(document[0].as_ref(), &[Token::Text(" a b ")]);
    }

    #[test]
    fn rejects_empty_input_for_both_policies() {
        assert!(matches!(
            GenericTokenizer::<Preserve>::new(&[]).tokenize(""),
            Err(TokenizerError::EmptyInput)
        ));
        assert!(matches!(
            GenericTokenizer::<Discard>::new(&[]).tokenize(""),
            Err(TokenizerError::EmptyInput)
        ));
    }
}
