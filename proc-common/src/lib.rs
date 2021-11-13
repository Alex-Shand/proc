//! Code shared between the underlying proc crates

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
//#![deny(dead_code)]
#![warn(clippy::pedantic)]

use proc_macro2::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::format_ident;

pub use quote;
pub use syn;
pub use trybuild;

mod macros;

#[doc(hidden)]
pub mod procutils;
pub mod testutils;

/// Get appropriate tokens to use to refer to paths in the named crate
#[must_use]
pub fn import_crate(name: &str) -> TokenStream {
    match crate_name(name) {
        Ok(FoundCrate::Itself) => quote::quote!(crate),
        Ok(FoundCrate::Name(name)) => {
            let ident = format_ident!("{}", name);
            quote::quote!(::#ident)
        }
        Err(_) => quote::quote!(self),
    }
}
