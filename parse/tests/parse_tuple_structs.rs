use std::str::FromStr;

use common::{proc_macro2::TokenStream, syn};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(parse::Parse, Debug, PartialEq)]
#[parse(crate = common)]
struct Unit1(syn::LitBool);

#[derive(parse::Parse, Debug, PartialEq)]
#[parse(crate = common)]
struct Unit2(syn::LitBool, syn::LitBool);

#[derive(parse::Parse, Debug, PartialEq)]
#[parse(crate = common)]
struct Unit3(syn::LitInt, syn::LitInt, syn::LitInt);

#[test]
fn parse_unit_1() -> Result<()> {
    let expected = Unit1(syn::parse_quote!(false));
    let tokens = TokenStream::from_str("false")?;
    let actual = syn::parse2(tokens)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn parse_unit_2() -> Result<()> {
    let expected = Unit2(syn::parse_quote!(false), syn::parse_quote!(true));
    let tokens = TokenStream::from_str("false true")?;
    let actual = syn::parse2(tokens)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn parse_unit_3() -> Result<()> {
    let expected = Unit3(
        syn::parse_quote!(0),
        syn::parse_quote!(1),
        syn::parse_quote!(2),
    );
    let tokens = TokenStream::from_str("0 1 2")?;
    let actual = syn::parse2(tokens)?;
    assert_eq!(expected, actual);
    Ok(())
}
