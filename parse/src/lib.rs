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
    quote::{quote, ToTokens},
    syn::{DeriveInput, Error, Path, Result},
    util::{
        EnumData, ForEachField, ResultFormatter, StructData, StructEnumDeriver,
    },
};

use self::enum_impl::EnumImpl;

mod enum_impl;

#[allow(missing_docs)]
#[proc_derive::derive(crate = proc_common, host = "proc", name = Parse)]
pub fn derive(
    crate_: Path,
    __internal_proc_hack: Optional<Path>,
    item: DeriveInput,
) -> Result<StructEnumDeriver<(Path, Path)>> {
    if !item.generics.params.is_empty() {
        return Err(Error::new_spanned(
            item.generics,
            "#[derive(Parse)] doesn't support generics",
        ));
    }

    let parse = __internal_proc_hack.unwrap_or_else(|| crate_.clone());
    StructEnumDeriver::new(
        "Parse",
        item,
        (crate_, parse),
        struct_logic,
        enum_logic,
    )
}

fn struct_logic(
    StructData {
        ident,
        generics: _,
        fields,
    }: &StructData,
    (crate_, _): &(Path, Path),
    tokens: &mut TokenStream,
) {
    let fields = ForEachField(fields, |_, field| {
        if let Some(ident) = &field.ident {
            quote!(#ident: input.parse()?)
        } else {
            quote!(input.parse()?)
        }
    });
    tokens.extend(quote! {
        impl #crate_::syn::parse::Parse for #ident {
            fn parse(input: #crate_::syn::parse::ParseStream<'_>) -> #crate_::syn::Result<Self> {
                Ok(Self #fields)
            }
        }
    });
}

fn enum_logic(
    EnumData {
        ident,
        generics: _,
        variants,
    }: &EnumData,
    (crate_, parse): &(Path, Path),
    tokens: &mut TokenStream,
) {
    ResultFormatter(EnumImpl::new(crate_, parse, ident, variants))
        .to_tokens(tokens);
}

#[cfg(test)]
mod tests {
    #[test]
    fn ui() {
        trybuild::TestCases::new().compile_fail("tests/ui/*.rs");
    }
}
