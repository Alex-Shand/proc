//! derive
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
    meta::{Optional, Required},
    proc_macro2::TokenStream,
    quote::{format_ident, quote, ToTokens},
    syn::{self, Ident, ItemFn, LitStr, Path, Result},
};
use convert_case::{Case, Casing as _};

use self::{arg_parser::ArgumentParser, arg_spec::ArgSpec, invoke::Invoke};

mod arg_parser;
mod arg_spec;
mod invoke;

/// .
#[attribute::attribute(crate = common, host = "proc")]
pub fn derive(
    crate_: Path,
    name: Required<Ident>,
    attribute: Optional<Ident>,
    host: Optional<LitStr>,
    item: ItemFn,
) -> Result<DeriveMacro> {
    let attribute = if let Some(attribute) = attribute {
        attribute
    } else {
        format_ident!("{}", name.to_string().to_case(Case::Snake))
    };
    DeriveMacro::new(crate_, name, attribute, host, item)
}

struct DeriveMacro {
    crate_: Path,
    args: Ident,
    item: Ident,
    name: Ident,
    attribute: Ident,
    arg_spec: ArgSpec,
    logic: ItemFn,
}

impl DeriveMacro {
    fn new(
        crate_: Path,
        name: Ident,
        attribute: Ident,
        host: Option<LitStr>,
        logic: ItemFn,
    ) -> Result<Self> {
        let arg_spec = ArgSpec::new(&logic.sig, crate_.clone(), host)?;
        Ok(Self {
            crate_,
            args: format_ident!("__proc_internal_args"),
            item: format_ident!("__proc_internal_item"),
            name,
            attribute,
            arg_spec,
            logic,
        })
    }

    fn attrs(&self) -> &[syn::Attribute] {
        &self.logic.attrs
    }

    fn impl_name(&self) -> &Ident {
        &self.logic.sig.ident
    }

    fn args(&self) -> ArgumentParser<'_> {
        ArgumentParser::new(
            &self.crate_,
            &self.item,
            &self.args,
            &self.attribute,
            &self.arg_spec,
        )
    }

    fn invoke(&self) -> Invoke<'_> {
        Invoke::new(&self.item, self.impl_name(), &self.arg_spec)
    }
}

impl ToTokens for DeriveMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_ = &self.crate_;
        let item = &self.item;
        let attrs = self.attrs();
        let name = &self.name;
        let attribute = &self.attribute;
        let impl_name = self.impl_name();
        let logic = self.arg_spec.patch_parsable_args(self.logic.clone());
        let args = self.args();
        let invoke = self.invoke();
        tokens.extend(quote! {
            #(#attrs)*
            #[proc_macro_derive(#name, attributes(#attribute))]
            pub fn #impl_name(#item: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
                #[allow(unreachable_pub)]
                #[allow(unnecessary_wraps)]
                #[allow(clippy::needless_pass_by_value)]
                #logic
                #args
                #crate_::proc_attribute_function_must_return_proc_result(
                    #invoke
                ).into()
            }
        });
    }
}
