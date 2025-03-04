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

use proc_common::{
    meta::Optional,
    proc_macro2::TokenStream,
    quote::ToTokens,
    syn::{Data, DeriveInput, Error, Path, Result},
};

use self::{enum_impl::EnumImpl, struct_impl::StructImpl};

mod enum_impl;
mod struct_impl;

#[allow(missing_docs)]
#[proc_derive::derive(crate = proc_common, host = "proc", name = Parse)]
pub fn derive(
    crate_: Path,
    __internal_proc_hack: Optional<Path>,
    item: DeriveInput,
) -> Result<ParseImpl> {
    let parse = __internal_proc_hack.unwrap_or_else(|| crate_.clone());
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
            ParseImpl::Enum(EnumImpl::new(crate_, parse, item.ident, data)?)
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

#[cfg(test)]
mod tests {
    #[test]
    fn ui() {
        trybuild::TestCases::new().compile_fail("tests/ui/*.rs");
    }
}
