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
    quote::{format_ident, quote, ToTokens},
    syn::{self, Attribute, Ident, ItemFn, LitStr, Path, Result},
};

use self::{arg_parser::ArgumentParser, arg_spec::ArgSpec, invoke::Invoke};

mod arg_parser;
mod arg_spec;
mod invoke;

/// Wrapper macro for defining attribute macros
#[proc_macro_attribute]
pub fn attribute(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let arg_spec = (meta::Optional::new("crate"), meta::Optional::new("host"));
    let (crate_, host) = match arg_spec.parse_bare(args.into()) {
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
    crate_: Path,
    args: Ident,
    item: Ident,
    arg_spec: ArgSpec,
    logic: ItemFn,
}

impl AttributeMacro {
    fn new(
        crate_: Option<Path>,
        host: Option<LitStr>,
        logic: ItemFn,
    ) -> Result<Self> {
        let crate_ = if let Some(crate_) = crate_ {
            crate_
        } else {
            get_crate("proc")?
        };
        let arg_spec = ArgSpec::new(&logic.sig, crate_.clone(), host)?;
        Ok(Self {
            crate_,
            args: format_ident!("__proc_internal_args"),
            item: format_ident!("__proc_internal_item"),
            arg_spec,
            logic,
        })
    }

    fn attrs(&self) -> &[Attribute] {
        &self.logic.attrs
    }

    fn name(&self) -> &Ident {
        &self.logic.sig.ident
    }

    fn arg_parser(&self) -> ArgumentParser<'_> {
        ArgumentParser::new(&self.crate_, &self.args, &self.arg_spec)
    }

    fn invoke(&self) -> Invoke<'_> {
        Invoke::new(&self.crate_, &self.item, self.name(), &self.arg_spec)
    }
}

impl ToTokens for AttributeMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_ = &self.crate_;
        let args = &self.args;
        let item = &self.item;
        let attrs = self.attrs();
        let name = self.name();
        let logic = self.arg_spec.patch_parsable_args(self.logic.clone());
        let arg_parser = self.arg_parser();
        let invoke = self.invoke();
        tokens.extend(quote! {
            #(#attrs)*
            #[proc_macro_attribute]
            pub fn #name(#args: ::proc_macro::TokenStream, #item: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
                #[allow(unreachable_pub)]
                #[allow(unnecessary_wraps)]
                #[allow(clippy::needless_pass_by_value)]
                #logic
                #arg_parser
                #crate_::proc_attribute_function_must_return_proc_result(
                    #invoke
                ).into()
            }
        });
    }
}
