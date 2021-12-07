use quote::quote;
use syn::ItemFn;

use proc_common::procutils::parse_macro_processor;

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn attribute(input: ItemFn) -> proc_macro2::TokenStream {
    let (name, test_name, docs, proc) = match parse_macro_processor(&input, "attribute") {
        Ok(ok) => ok,
        Err(tokens) => return tokens,
    };

    quote! {
        #(#docs)*
        #[proc_macro_attribute]
        pub fn #name(
            _attr: ::proc_macro::TokenStream,
            input: ::proc_macro::TokenStream
        ) -> ::proc_macro::TokenStream {
            #input
            ::std::convert::Into::into(#name(#proc::syn::parse_macro_input!(input)))
        }

        #[cfg(test)]
        fn #test_name(
            input: #proc::TokenStream
        ) -> ::std::result::Result<#proc::testutils::TokenStream, ::std::string::String> {
            #input
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

    fn attribute(input: TokenStream) -> TokenStream {
        TokenStream(super::attribute(syn::parse2(input.0).unwrap()))
    }

    testcase! {
        success,
        attribute(q! {
            /// Docs
            fn test(_input: Input) -> proc::TokenStream {
                todo!()
            }
        }),
        q! {
            /// Docs
            #[proc_macro_attribute]
            pub fn test(
                _attr: ::proc_macro::TokenStream,
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
