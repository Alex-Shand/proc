//! common
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_results)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::let_underscore_untyped)]
#![allow(clippy::similar_names)]

pub use proc_macro2;
use proc_macro2::TokenStream;
use proc_macro_crate::FoundCrate;
pub use quote;
use quote::ToTokens;
pub use syn;
use syn::{parse::Parser, Result};

#[doc(hidden)]
pub fn get_crate(name: &'static str) -> Result<syn::Path> {
    match proc_macro_crate::crate_name(name) {
        Ok(FoundCrate::Itself) => Ok(syn::parse_quote!(crate)),
        Ok(FoundCrate::Name(name)) => {
            let ident = quote::format_ident!("{name}");
            Ok(syn::parse_quote!(::#ident))
        }
        Err(e) => Err(syn::Error::new(proc_macro2::Span::call_site(), e)),
    }
}

#[doc(hidden)]
pub fn parse_attribute_args(tokens: TokenStream) -> Result<Option<syn::Path>> {
    let mut crate_ = None;
    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("crate") {
            crate_ = Some(meta.value()?.parse()?);
            return Ok(());
        }
        Err(meta.error("unrecognised argument"))
    });
    let () = parser.parse2(tokens)?;
    Ok(crate_)
}

#[doc(hidden)]
#[expect(clippy::inline_always)]
#[inline(always)]
pub fn proc_attribute_function_must_return_proc_result<T: ToTokens>(
    result: Result<T>,
) -> TokenStream {
    match result {
        Ok(result) => quote::quote!(#result),
        Err(e) => e.into_compile_error(),
    }
}
