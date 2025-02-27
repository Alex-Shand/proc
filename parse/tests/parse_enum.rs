use std::str::FromStr;

use common::{proc_macro2::TokenStream, syn};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(parse::Parse, Debug, PartialEq)]
#[parse(crate = common, __internal_proc_hack = parse)]
enum Enum {
    Named {
        str: syn::LitStr,
    },
    NeverConstructed {
        duplicate_first_field_type: syn::LitStr,
    },
    Tuple(syn::LitInt),
}

#[test]
fn parse_enum1() -> Result<()> {
    let expected = Enum::Named {
        str: syn::parse_quote!("str"),
    };
    let tokens = TokenStream::from_str("\"str\"")?;
    let actual = syn::parse2(tokens)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn parse_enum2() -> Result<()> {
    let expected = Enum::Tuple(syn::parse_quote!(0));
    let tokens = TokenStream::from_str("0")?;
    let actual = syn::parse2(tokens)?;
    assert_eq!(expected, actual);
    Ok(())
}
