use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    Path, Token,
};

/// Helper type for feeding $crate to a function-like proc macro
///
/// Understands the syntax `crate = <path>, args = (...)`
#[derive(Debug)]
pub struct CrateAnd<T: Parse> {
    #[allow(missing_docs)]
    pub crate_: Path,
    #[allow(missing_docs)]
    pub args: T,
}

mod kw {
    use syn::custom_keyword;

    custom_keyword!(args);
}

impl<T: Parse> Parse for CrateAnd<T> {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let _: Token![crate] = input.parse()?;
        let _: Token![=] = input.parse()?;
        let crate_ = input.parse()?;
        let _: Token![,] = input.parse()?;
        let _: kw::args = input.parse()?;
        let _: Token![=] = input.parse()?;
        let content;
        let _ = parenthesized!(content in input);
        let args = content.parse()?;
        Ok(Self { crate_, args })
    }
}
