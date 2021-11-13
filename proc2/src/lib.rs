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
//#![deny(dead_code)]
#![warn(clippy::pedantic)]

use std::path::Path;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

proc_common::proc_internal_hack! {}
use proc_common::error_at;

mod tests;

/// Generate test cases for a `proc` generated proc macros
#[proc1::function]
fn tests(input: tests::Input) -> TokenStream {
    let pass = input.pass;
    let fail = input.fail;
    let macro_name = format_ident!("__{}", input.name);

    let tests = match tests::get_expansion_tests(Path::new(&pass.value())) {
        Ok(tests) => tests,
        Err(err) => {
            return error_at!(
                pass,
                format!("Failed to read expansion test cases: {}", err)
            )
        }
    };
    let tests = tests
        .into_iter()
        .map(|(name, input, output)| {
            quote! {
                #[test]
                fn #name() {
                    ::proc::testutils::expand_and_compare(#macro_name, #input, #output)
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        #[cfg(test)]
        mod test {
            use super::#macro_name;

            #[test]
            fn __trybuild() {
                let t = ::proc::trybuild::TestCases::new();
                t.pass(::std::concat!(#pass, "/*.rs"));
                t.compile_fail(::std::concat!(#fail, "/*.rs"))
            }

            #(#tests)*
        }
    }
}
