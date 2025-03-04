use proc_common::{proc_macro2::TokenStream, syn};

#[derive(proc_parse::Parse)]
#[parse(crate = proc_common)]
struct Test;

#[test]
fn parse_unit_struct() -> syn::Result<()> {
    let tokens = TokenStream::new();
    let _: Test = syn::parse2(tokens)?;
    Ok(())
}
