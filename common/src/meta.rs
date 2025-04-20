use proc_macro2::TokenStream;
use syn::{parse::Parser as _, Attribute, Result};

pub use self::{
    custom::{Custom, Meta},
    list::List,
    optional::Optional,
    required::{required_argument_error, Required},
    switch::Switch,
};

mod custom;
mod list;
mod optional;
mod required;
mod switch;
mod tuple;

/// Parser for attribute meta-argument syntax
#[sealed::sealed]
pub trait MetaParser: Sized {
    #[doc(hidden)]
    type Item;
    #[doc(hidden)]
    fn parse(&mut self, meta: &syn::meta::ParseNestedMeta<'_>) -> Result<bool>;
    #[doc(hidden)]
    fn validate(self) -> Result<Self::Item>;
}

#[doc(hidden)]
pub fn parse_bare<M: MetaParser>(
    mut parser: M,
    tokens: TokenStream,
) -> Result<M::Item> {
    let parse_fn = syn::meta::parser(|meta| {
        if parser.parse(&meta)? {
            return Ok(());
        }
        Err(meta.error("unrecognised argument"))
    });
    let () = parse_fn.parse2(tokens)?;
    parser.validate()
}

/// Parse meta attributes
///
/// The provided attribute list is filtered for only attributes matching the
/// provided attribute name and parsed according to the provided parser
pub fn parse_attrs<M: MetaParser>(
    mut parser: M,
    attribute: &str,
    attrs: &[Attribute],
) -> Result<M::Item> {
    for attr in attrs.iter().filter(|a| a.path().is_ident(attribute)) {
        attr.parse_nested_meta(|meta| {
            if parser.parse(&meta)? {
                return Ok(());
            }
            Err(meta.error("unrecognised argument"))
        })?;
    }
    parser.validate()
}

/// Trait implemented by valid inputs to a derive macro
#[sealed::sealed]
pub trait DeriveInput: Sized {
    #[doc(hidden)]
    fn skim_attributes(&self) -> &[Attribute];
}

#[sealed::sealed]
impl DeriveInput for syn::DeriveInput {
    fn skim_attributes(&self) -> &[Attribute] {
        &self.attrs
    }
}

#[sealed::sealed]
impl DeriveInput for syn::ItemStruct {
    fn skim_attributes(&self) -> &[Attribute] {
        &self.attrs
    }
}

#[sealed::sealed]
impl DeriveInput for syn::ItemEnum {
    fn skim_attributes(&self) -> &[Attribute] {
        &self.attrs
    }
}

#[sealed::sealed]
impl DeriveInput for syn::ItemUnion {
    fn skim_attributes(&self) -> &[Attribute] {
        &self.attrs
    }
}
