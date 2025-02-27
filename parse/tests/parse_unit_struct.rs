use common::{proc_macro2::TokenStream, syn};

#[derive(parse::Parse)]
#[parse(crate = common)]
struct Test;

#[test]
fn parse_unit_struct() -> syn::Result<()> {
    let tokens = TokenStream::new();
    let _: Test = syn::parse2(tokens)?;
    Ok(())
}
