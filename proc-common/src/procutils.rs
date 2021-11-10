use super::{error_at, import_crate};

use proc_macro2::TokenStream;
use syn::{Attribute, FnArg, Ident, ItemFn};

pub fn parse_macro_processor(
    input: &ItemFn,
) -> Result<(&Ident, Vec<&Attribute>, TokenStream), TokenStream> {
    let name = &input.sig.ident;
    let args = &input.sig.inputs.iter().collect::<Vec<_>>();
    if args.len() != 1 {
        return Err(error_at!(
            input.sig,
            "The attribute function should have one argument"
        ));
    }

    if let FnArg::Receiver(_) = args[0] {
        return Err(error_at!(
            input.sig,
            "The attribute function should be free standing"
        ));
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
