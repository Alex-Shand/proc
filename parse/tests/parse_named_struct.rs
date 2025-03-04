use std::str::FromStr;

use proc_common::{proc_macro2::TokenStream, syn};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(proc_parse::Parse, Debug, PartialEq)]
#[parse(crate = proc_common)]
struct Named {
    b: syn::LitBool,
    n: syn::LitInt,
    s: syn::LitStr,
}

#[test]
fn parse_named_struct() -> Result<()> {
    let expected = Named {
        b: syn::parse_quote!(false),
        n: syn::parse_quote!(0),
        s: syn::parse_quote!("str"),
    };
    let tokens = TokenStream::from_str("false 0 \"str\"")?;
    let actual = syn::parse2(tokens)?;
    assert_eq!(expected, actual);
    Ok(())
}
