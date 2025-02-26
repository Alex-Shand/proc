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
    proc_macro2::TokenStream,
    quote::ToTokens,
    syn::{Data, DeriveInput, Error, Path, Result},
};

use self::{enum_impl::EnumImpl, struct_impl::StructImpl};

mod enum_impl;
mod struct_impl;

/// .
#[derive::derive(crate = common, host = "proc", name = Parse)]
pub fn derive(crate_: Path, item: DeriveInput) -> Result<ParseImpl> {
    if item.generics.params.iter().next().is_some() {
        return Err(Error::new_spanned(
            item.generics,
            "#[derive(Parse)] doesn't support generics",
        ));
    }
    Ok(match item.data {
        Data::Struct(data) => {
            ParseImpl::Struct(StructImpl::new(crate_, item.ident, data))
        }
        Data::Enum(data) => {
            ParseImpl::Enum(EnumImpl::new(crate_, item.ident, data)?)
        }
        Data::Union(data) => {
            return Err(Error::new_spanned(
                data.union_token,
                "#[derive(Parse)] cannot be used on unions",
            ));
        }
    })
}

enum ParseImpl {
    Struct(StructImpl),
    Enum(EnumImpl),
}

impl ToTokens for ParseImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ParseImpl::Struct(s) => s.to_tokens(tokens),
            ParseImpl::Enum(e) => e.to_tokens(tokens),
        }
    }
}
