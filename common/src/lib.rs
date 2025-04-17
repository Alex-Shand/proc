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
pub use syn::{self, Path};
use syn::{Attribute, Result};

#[doc(hidden)]
pub mod argument;
/// Macro meta-argument parsing
pub mod meta;
/// Utilities for writing macros
pub mod util;

#[doc(hidden)]
pub fn get_crate(name: &'static str) -> Result<Path> {
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
#[expect(clippy::inline_always)]
#[inline(always)]
pub fn proc_attribute_function_must_return_proc_result<T: ToTokens>(
    result: Result<T>,
) -> TokenStream {
    util::ResultFormatter(result).into_token_stream()
}

#[doc(hidden)]
#[expect(clippy::inline_always)]
#[inline(always)]
pub fn proc_derive_function_must_return_proc_result<T: ToTokens>(
    result: Result<T>,
) -> TokenStream {
    util::ResultFormatter(result).into_token_stream()
}

#[doc(hidden)]
#[expect(clippy::inline_always)]
#[inline(always)]
pub fn proc_macro_function_must_return_proc_result<T: ToTokens>(
    result: Result<T>,
) -> TokenStream {
    util::ResultFormatter(result).into_token_stream()
}

#[doc(hidden)]
#[expect(clippy::inline_always)]
#[inline(always)]
pub fn proc_derive_last_argument_must_implement_meta_derive_input<
    D: meta::DeriveInput,
>(
    derive_input: D,
    guard: &'static str,
) -> (D, Vec<Attribute>) {
    derive_input.skim_attributes(guard)
}

#[cfg(test)]
mod tests {
    use pa::assert_eq;
    use quote::ToTokens;

    use crate::get_crate;

    #[test]
    fn get_current_crate() -> syn::Result<()> {
        assert_eq!(
            "crate",
            get_crate("proc_common")?.into_token_stream().to_string()
        );
        Ok(())
    }

    #[test]
    fn get_dependency_crate() -> syn::Result<()> {
        assert_eq!(":: syn", get_crate("syn")?.into_token_stream().to_string());
        Ok(())
    }

    #[test]
    fn get_aliased_crate() -> syn::Result<()> {
        assert_eq!(
            ":: pa",
            get_crate("pretty_assertions")?
                .into_token_stream()
                .to_string()
        );
        Ok(())
    }

    #[test]
    fn get_crate_error() {
        let manifest_path = env!("CARGO_MANIFEST_PATH");
        let result = get_crate("doesnt_exist");
        assert!(result.is_err());
        let Err(error) = result else { unreachable!() };
        assert_eq!(
            r#":: core :: compile_error ! { "Could not find `doesnt_exist` in `dependencies` or `dev-dependencies` in `MANIFEST`!" }"#,
            error
                .into_compile_error()
                .to_string()
                .replace(manifest_path, "MANIFEST")
        );
    }
}
