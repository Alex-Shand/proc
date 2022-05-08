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

use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Visibility};

proc_common::proc_internal_hack! {}
use proc_common::procutils::parse_macro_processor;

/// Define a function-like proc macro
#[proc0::attribute]
#[allow(clippy::needless_pass_by_value)]
fn function(input: ItemFn) -> TokenStream {
    let (name, test_name, docs, proc) = match parse_macro_processor(&input, "macro") {
        Ok(ok) => ok,
        Err(tokens) => return tokens,
    };

    let mut internal = input.clone();
    internal.vis = Visibility::Inherited;

    quote! {
        #(#docs)*
        #[proc_macro]
        pub fn #name(input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
            #internal
            ::std::convert::Into::into(#name(#proc::syn::parse_macro_input!(input)))
        }

        #[cfg(test)]
        fn #test_name(
            input: #proc::TokenStream
        ) -> ::std::result::Result<#proc::testutils::TokenStream, ::std::string::String> {
            #internal
            ::std::result::Result::Ok(
                #proc::testutils::TokenStream(
                    #name(#proc::syn::parse2(input).map_err(|e| e.to_string())?)
                )
            )
        }
    }
}

syntax_abuse::tests! {
    use proc_common::{q, testutils::TokenStream};

    fn function(input: TokenStream) -> TokenStream {
        __function(input.0).unwrap()
    }

    testcase! {
        success,
        function(q! {
            /// Docs
            fn test(_input: Input) -> proc::TokenStream {
                todo!()
            }
        }),
        q! {
            /// Docs
            #[proc_macro]
            pub fn test(
                input: ::proc_macro::TokenStream
            ) -> ::proc_macro::TokenStream {
                /// Docs
                fn test(_input: Input) -> proc::TokenStream {
                    todo!()
                }
                ::std::convert::Into::into(
                    test(self::syn::parse_macro_input!(input))
                )
            }

            #[cfg(test)]
            fn __test(
                input: self::TokenStream
            ) -> ::std::result::Result<self::testutils::TokenStream, ::std::string::String> {
                /// Docs
                fn test(_input: Input) -> proc::TokenStream {
                    todo!()
                }
                ::std::result::Result::Ok(
                    self::testutils::TokenStream(
                        test(self::syn::parse2(input).map_err(|e| e.to_string())?)
                    )
                )
            }
        }
    }
}
