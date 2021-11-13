use super::{error_at, import_crate};

use proc_macro2::TokenStream;
use quote::format_ident;
use syn::{Attribute, FnArg, Ident, ItemFn};

#[doc(hidden)]
pub fn parse_macro_processor<'a, 'b>(
    input: &'a ItemFn,
    kind: &'b str,
) -> Result<(&'a Ident, Ident, Vec<&'a Attribute>, TokenStream), TokenStream> {
    let name = &input.sig.ident;
    let args = &input.sig.inputs.iter().collect::<Vec<_>>();
    if args.len() != 1 {
        return Err(error_at!(
            input.sig,
            format!("The {} function should have one argument", kind)
        ));
    }

    if let FnArg::Receiver(_) = args[0] {
        return Err(error_at!(
            input.sig,
            format!("The {} function should be free standing", kind)
        ));
    };

    let docs = input.attrs.iter().filter(is_doc).collect::<Vec<_>>();
    let proc = import_crate("proc");

    Ok((name, format_ident!("__{}", name), docs, proc))
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_doc(attr: &&Attribute) -> bool {
    let segments = &attr.path.segments;
    if segments.len() != 1 {
        return false;
    }
    &segments[0].ident.to_string() == "doc"
}
