use quote::quote;
use syn::ItemFn;

use proc_common::procutils::parse_macro_processor;

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn attribute(input: ItemFn) -> proc_macro2::TokenStream {
    let (name, docs, proc) = match parse_macro_processor(&input) {
        Ok(ok) => ok,
        Err(tokens) => return tokens,
    };
    quote! {
        #(#docs)*
        #[cfg(not(test))]
        #[proc_macro_attribute]
        pub fn #name(
            _attr: ::proc_macro::TokenStream,
            input: ::proc_macro::TokenStream
        ) -> ::proc_macro::TokenStream {
            #input
            ::std::convert::Into::into(#name(#proc::syn::parse_macro_input!(input)))
        }

        #[cfg(test)]
        fn #name(
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
    use proc_common::{q, compile_error, testutils::TokenStream};

    fn attribute(input: TokenStream) -> TokenStream {
        TokenStream(super::attribute(syn::parse2(input.0).unwrap()))
    }

    testcase! {
        success,
        attribute(q! {
            fn test(input: Type) -> TokenStream {
                do_something_with(input)
            }
        }),
        q! {
            #[cfg(not(test))]
            #[proc_macro_attribute]
            pub fn test(
                _attr: ::proc_macro::TokenStream,
                input: ::proc_macro::TokenStream
            ) -> ::proc_macro::TokenStream {
                fn test(input: Type) -> TokenStream {
                    do_something_with(input)
                }
                ::std::convert::Into::into(
                    test(::proc::syn::parse_macro_input!(input))
                )
            }

            #[cfg(test)]
            fn test(
                input: ::proc::TokenStream
            ) -> ::std::result::Result<::proc::testutils::TokenStream, ::std::string::String> {
                fn test(input: Type) -> TokenStream {
                    do_something_with(input)
                }
                ::std::result::Result::Ok(
                    ::proc::testutils::TokenStream(
                        test(::proc::syn::parse2(input).map_err(|e| e.to_string())?)
                    )
                )
            }
        }
    }

    testcase! {
        empty_argument_list,
        attribute(q! {
            fn test() -> TokenStream {
            }
        }),
        compile_error!("The attribute function should have one argument")
    }

    testcase! {
        more_than_one_argument,
        attribute(q! {
            fn test(first: Type, second: Type) -> TokenStream {
            }
        }),
        compile_error!("The attribute function should have one argument")
    }

    testcase! {
        self_argument,
        attribute(q! {
            fn test(self) -> TokenStream {
            }
        }),
        compile_error!("The attribute function should be free standing")
    }

    testcase! {
        ref_self_argument,
        attribute(q! {
            fn test(&self) -> TokenStream {
            }
        }),
        compile_error!("The attribute function should be free standing")
    }

    testcase! {
        mut_ref_self_argument,
        attribute(q! {
            fn test(&mut self) -> TokenStream {
            }
        }),
        compile_error!("The attribute function should be free standing")
    }

    testcase! {
        docs,
        attribute(q! {
            /// Docs
            fn test(input: Input) -> TokenStream {
            }
        }),
        q! {
            /// Docs
            #[cfg(not(test))]
            #[proc_macro_attribute]
            pub fn test(
                _attr: ::proc_macro::TokenStream,
                input: ::proc_macro::TokenStream
            ) -> ::proc_macro::TokenStream {
                /// Docs
                fn test(input: Input) -> TokenStream {
                }
                ::std::convert::Into::into(
                    test(::proc::syn::parse_macro_input!(input))
                )
            }

            #[cfg(test)]
            fn test(
                input: ::proc::TokenStream
            ) -> ::std::result::Result<::proc::testutils::TokenStream, ::std::string::String> {
                /// Docs
                fn test(input: Input) -> TokenStream {
                }
                ::std::result::Result::Ok(
                    ::proc::testutils::TokenStream(
                        test(::proc::syn::parse2(input).map_err(|e| e.to_string())?)
                    )
                )
            }
        }
    }
}
