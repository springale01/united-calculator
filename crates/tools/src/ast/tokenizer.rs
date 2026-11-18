use std::any::Any;

pub trait Tokenizer: Any {
    fn tokenize(&self, input: &str) -> Result<Box<[Box<str>]>, TokenizerError>;
}
#[derive(Debug, thiserror::Error)]
pub enum TokenizerError {
    #[error("Empty Input")]
    EmptyInput,
}

pub(crate) mod generic_tokenizer {

    use std::marker::PhantomData;

    use itertools::Itertools;

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
        const DEFAULT_LIST: &[&str] = &[";", ",", ":", "-"];
        pub const fn new(delimiters: &'static [char]) -> Self {
            Self {
                delimiters,
                behavior: PhantomData,
            }
        }
    }

    impl<T> Tokenizer for GenericTokenizer<T>
    where
        T: GenericCalculatorPolicy,
    {
        fn tokenize(&self, input: &str) -> Result<Box<[Box<str>]>, TokenizerError> {
            if input.is_empty() {
                return Err(TokenizerError::EmptyInput);
            }

            todo!()
        }
    }
    // ----------- Policy Stuff ------------------------
    mod sealed {
        pub trait Sealed: Send {}
    }
    pub trait GenericCalculatorPolicy: self::sealed::Sealed + 'static {}
    /// the default visibility is pub crate, add a vis field for custom
    /// there should be a self::sealed::Sealed thing and it should
    /// be mod with pub trait Sealed
    macro_rules! seagull_structs {
        ($name:ident, $trait:ident) => {
            pub(crate) struct $name;
            impl self::sealed::Sealed for $name {}
            impl $trait for $name {
            }
        };
        ($name:ident, $visibility:vis, $trait:ident) => {
            pub($visibility) struct $name;
            impl self::sealed::Sealed for $name {}
            impl $trait for $name {

            }
        }
    }
    seagull_structs!(Preserve, GenericCalculatorPolicy);
    seagull_structs!(Discard, GenericCalculatorPolicy);
}
