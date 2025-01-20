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
    meta::{self, Meta as _},
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
    let arg_spec = (meta::Optional::new("crate"), meta::Optional::new("host"));
    let (crate_, host) = match arg_spec.parse(args.into()) {
        Ok(result) => result,
        Err(e) => return e.into_compile_error().into(),
    };
    let result =
        AttributeMacro::new(crate_, host, syn::parse_macro_input!(item));
    match result {
        Ok(result) => quote!(#result).into(),
        Err(e) => e.into_compile_error().into(),
    }
}

struct AttributeMacro {
    crate_: syn::Path,
    host: Option<syn::LitStr>,
    logic: syn::ItemFn,
}

impl AttributeMacro {
    fn new(
        crate_: Option<syn::Path>,
        host: Option<syn::LitStr>,
        logic: syn::ItemFn,
    ) -> syn::Result<Self> {
        let crate_ = if let Some(crate_) = crate_ {
            crate_
        } else {
            get_crate("proc")?
        };
        Ok(Self {
            crate_,
            host,
            logic,
        })
    }

    fn name(&self) -> &syn::Ident {
        &self.logic.sig.ident
    }

    fn attrs(&self) -> &[syn::Attribute] {
        &self.logic.attrs
    }

    fn args(&self) -> ArgumentParser<'_> {
        ArgumentParser {
            crate_: &self.crate_,
            host: &self.host,
        }
    }

    fn invoke(&self) -> Invoke<'_> {
        Invoke {
            crate_: &self.crate_,
            name: self.name(),
            extra_args: self.args().idents(),
        }
    }
}

impl ToTokens for AttributeMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_ = &self.crate_;
        let name = self.name();
        let attrs = self.attrs();
        let logic = &self.logic;
        let args = self.args();
        let invoke = self.invoke();
        tokens.extend(quote! {
            #(#attrs)*
            #[proc_macro_attribute]
            pub fn #name(args: ::proc_macro::TokenStream, item: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
                #[allow(unreachable_pub)]
                #[allow(unnecessary_wraps)]
                #logic
                #args
                #crate_::proc_attribute_function_must_return_proc_result(
                    #invoke
                ).into()
            }
        });
    }
}

struct ArgumentParser<'a> {
    crate_: &'a syn::Path,
    host: &'a Option<syn::LitStr>,
}

impl ArgumentParser<'_> {
    fn idents(&self) -> Vec<syn::Ident> {
        let Self { host, .. } = self;
        if host.is_some() {
            return vec![syn::parse_quote!(crate_)];
        }
        Vec::new()
    }
}

impl ToTokens for ArgumentParser<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ArgumentParser { crate_, host } = self;
        let Some(host) = host else {
            return;
        };
        tokens.extend(quote! {
            let arg_spec = #crate_::meta::Optional::new("crate");
            let crate_ = match #crate_::meta::Meta::parse(arg_spec, args.into()) {
                Ok(result) => result,
                Err(e) => return e.into_compile_error().into()
            };
            let crate_ = if let Some(c) = crate_ {
                c
            } else {
                match #crate_::get_crate(#host) {
                    Ok(c) => c,
                    Err(e) => return e.into_compile_error().into()
                }
            };
        });
    }
}

struct Invoke<'a> {
    crate_: &'a syn::Path,
    name: &'a syn::Ident,
    extra_args: Vec<syn::Ident>,
}

impl ToTokens for Invoke<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Invoke {
            crate_,
            name,
            extra_args,
        } = self;
        tokens.extend(quote! {
            #name(#(#extra_args,)* #crate_::syn::parse_macro_input!(item))
        });
    }
}
