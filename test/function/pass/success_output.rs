extern crate proc_macro;
use proc::{syn, testutils, TokenStream};

struct Input;

impl syn::parse::Parse for Input {
    fn parse(_: syn::parse::ParseStream<'_>) -> syn::Result<Self> { todo!() }
}

/// Docs
//#[proc_macro]
pub fn test(
    input: ::proc_macro::TokenStream
) -> ::proc_macro::TokenStream {
    /// Docs
    fn test(_input: Input) -> proc::TokenStream {
        todo!()
    }
    ::std::convert::Into::into(
        test(self::syn::parse_macro_input!(input))
    )
}

//#[cfg(test)]
fn __test(
    input: self::TokenStream
) -> ::std::result::Result<self::testutils::TokenStream, ::std::string::String> {
    /// Docs
    fn test(_input: Input) -> proc::TokenStream {
        todo!()
    }
    ::std::result::Result::Ok(
        self::testutils::TokenStream(
            test(self::syn::parse2(input).map_err(|e| e.to_string())?)
        )
    )
}

fn main() {}
