//! Proc macros for `proc` to re-export

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
#![deny(dead_code)]
#![warn(clippy::pedantic)]

mod attribute;

/// Declare an attribute `proc_macro`
#[proc_macro_attribute]
pub fn attribute(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(
        attribute::attribute(
            syn::parse_macro_input!(input)
        )
    )
}
