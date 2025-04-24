//! Types with useful [Parse] impls

use syn::{
    braced, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    Result,
};

/// Type based version of [braced]
#[derive(Debug)]
pub struct Braced<T: Parse>(pub T);

impl<T: Parse> Parse for Braced<T> {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        let _ = braced!(content in input);
        Ok(Braced(content.parse()?))
    }
}

/// Type based version of [bracketed]
#[derive(Debug)]
pub struct Bracketed<T: Parse>(pub T);

impl<T: Parse> Parse for Bracketed<T> {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        let _ = bracketed!(content in input);
        Ok(Bracketed(content.parse()?))
    }
}

/// Type based version of [parenthesized]
#[derive(Debug)]
pub struct Parenthesized<T: Parse>(pub T);

impl<T: Parse> Parse for Parenthesized<T> {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        let _ = parenthesized!(content in input);
        Ok(Parenthesized(content.parse()?))
    }
}

/// Parse the rest of the input as a sequence of items
#[derive(Debug)]
pub struct GreedySequence<T: Parse>(pub Vec<T>);

impl<T: Parse> Parse for GreedySequence<T> {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut result = Vec::new();
        while !input.is_empty() {
            result.push(input.parse()?);
        }
        Ok(Self(result))
    }
}
