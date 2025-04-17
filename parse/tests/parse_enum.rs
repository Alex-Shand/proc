use std::str::FromStr;

use proc_common::{proc_macro2::TokenStream, syn};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(proc_parse::Parse, Debug, PartialEq)]
#[parse(crate = proc_common, __internal_proc_hack = proc_parse)]
enum Enum {
    Named {
        str: syn::LitStr,
        int: syn::LitInt,
    },
    Named2 {
        str: syn::LitStr,
        bool: syn::LitBool,
    },
    Tuple(syn::LitInt),
    Unit,
}

#[test]
fn parse_enum1() -> Result<()> {
    let expected = Enum::Named {
        str: syn::parse_quote!("str"),
        int: syn::parse_quote!(5),
    };
    let tokens = TokenStream::from_str("\"str\" 5")?;
    let actual = syn::parse2(tokens)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn parse_enum2() -> Result<()> {
    let expected = Enum::Named2 {
        str: syn::parse_quote!("str"),
        bool: syn::parse_quote!(true),
    };
    let tokens = TokenStream::from_str("\"str\" true")?;
    let actual = syn::parse2(tokens)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn parse_enum3() -> Result<()> {
    let expected = Enum::Tuple(syn::parse_quote!(0));
    let tokens = TokenStream::from_str("0")?;
    let actual = syn::parse2(tokens)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn parse_enum4() -> Result<()> {
    let expected = Enum::Unit;
    let tokens = TokenStream::from_str("")?;
    let actual = syn::parse2(tokens)?;
    assert_eq!(expected, actual);
    Ok(())
}
