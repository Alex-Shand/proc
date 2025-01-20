//! attribute
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

use common::{
    get_crate,
    proc_macro2::TokenStream,
    quote::{quote, ToTokens},
    syn,
};

/// Wrapper macro for defining attribute macros
#[proc_macro_attribute]
pub fn attribute(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let crate_ = match common::parse_attribute_args(args.into()) {
        Ok(result) => result,
        Err(e) => return e.into_compile_error().into(),
    };
    let result = AttributeMacro::new(crate_, syn::parse_macro_input!(item));
    match result {
        Ok(result) => quote!(#result).into(),
        Err(e) => e.into_compile_error().into(),
    }
}

struct AttributeMacro {
    crate_: syn::Path,
    logic: syn::ItemFn,
}

impl AttributeMacro {
    fn new(crate_: Option<syn::Path>, logic: syn::ItemFn) -> syn::Result<Self> {
        let crate_ = if let Some(crate_) = crate_ {
            crate_
        } else {
            get_crate("proc")?
        };
        Ok(Self { crate_, logic })
    }

    fn name(&self) -> &syn::Ident {
        &self.logic.sig.ident
    }

    fn attrs(&self) -> &[syn::Attribute] {
        &self.logic.attrs
    }
}

impl ToTokens for AttributeMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_ = &self.crate_;
        let name = self.name();
        let attrs = self.attrs();
        let logic = &self.logic;
        tokens.extend(quote! {
            #(#attrs)*
            #[proc_macro_attribute]
            pub fn #name(args: ::proc_macro::TokenStream, item: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
                #[allow(unreachable_pub)]
                #[allow(unnecessary_wraps)]
                #logic
                let crate_ = match #crate_::parse_attribute_args(args.into()) {
                    Ok(result) => result,
                    Err(e) => return e.into_compile_error().into()
                };
                let crate_ = if let Some(crate_) = crate_ {
                    crate_
                } else {
                    match #crate_::get_crate(::std::module_path!()) {
                        Ok(c) => c,
                        Err(e) => return e.into_compile_error().into()
                    }
                };
                #crate_::proc_attribute_function_must_return_proc_result(
                    #name(crate_, #crate_::syn::parse_macro_input!(item)
                )).into()
            }
        });
    }
}
