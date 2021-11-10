use proc_macro_crate::{crate_name, FoundCrate};
use quote::format_ident;
use syn::{Ident, Attribute, FnArg, ItemFn};

pub use quote;
pub use syn;
pub use trybuild;

mod macros;

pub mod testutils;

fn get_crate_path<S: AsRef<str>>(name: S) -> proc_macro2::TokenStream {
    let ident = format_ident!("{}", name.as_ref());
    quote::quote!(::#ident)
}

pub fn import_crate(name: &'static str) -> proc_macro2::TokenStream {
    match crate_name(name) {
        Ok(FoundCrate::Itself) => quote::quote!(crate),
        Ok(FoundCrate::Name(name)) => get_crate_path(name),
        Err(_) => get_crate_path(name),
    }
}

pub fn parse_macro_processor(input: &ItemFn) -> Result<(&Ident, Vec<&Attribute>, proc_macro2::TokenStream), proc_macro2::TokenStream> {
    let name = &input.sig.ident;
    let args = &input.sig.inputs.iter().collect::<Vec<_>>();
    if args.len() != 1 {
        return Err(error_at!(input.sig, "The attribute function should have one argument"));
    }

    if let FnArg::Receiver(_) = args[0] {
        return Err(error_at!(input.sig, "The attribute function should be free standing"));
    };

    let docs = input.attrs.iter().filter(is_doc).collect::<Vec<_>>();
    let proc = import_crate("proc");

    Ok((name, docs, proc))
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_doc(attr: &&Attribute) -> bool {
    let segments = &attr.path.segments;
    if segments.len() != 1 {
        return false;
    }
    &segments[0].ident.to_string() == "doc"
}
