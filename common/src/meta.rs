use proc_macro2::TokenStream;
use syn::{parse::Parser as _, Attribute, Result};

pub use self::{optional::Optional, required::Required, switch::Switch};

mod optional;
mod required;
mod switch;
mod tuple;

#[doc(hidden)]
#[sealed::sealed]
pub trait Meta: Sized {
    type Item;
    fn parse(&mut self, meta: &syn::meta::ParseNestedMeta<'_>) -> Result<bool>;
    fn validate(self) -> Result<Self::Item>;
}

#[doc(hidden)]
pub fn parse_bare<M: Meta>(
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

#[doc(hidden)]
pub fn parse_attrs<M: Meta>(
    mut parser: M,
    attrs: &[Attribute],
) -> Result<M::Item> {
    for attr in attrs {
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
    fn skim_attributes(self, guard: &'static str) -> (Self, Vec<Attribute>);
}

#[sealed::sealed]
impl DeriveInput for syn::DeriveInput {
    fn skim_attributes(
        mut self,
        guard: &'static str,
    ) -> (Self, Vec<Attribute>) {
        let (our_attrs, rest) = self
            .attrs
            .into_iter()
            .partition(|a| a.path().is_ident(guard));
        self.attrs = rest;
        (self, our_attrs)
    }
}

#[sealed::sealed]
impl DeriveInput for syn::ItemStruct {
    fn skim_attributes(
        mut self,
        guard: &'static str,
    ) -> (Self, Vec<Attribute>) {
        let (our_attrs, rest) = self
            .attrs
            .into_iter()
            .partition(|a| a.path().is_ident(guard));
        self.attrs = rest;
        (self, our_attrs)
    }
}

#[sealed::sealed]
impl DeriveInput for syn::ItemEnum {
    fn skim_attributes(
        mut self,
        guard: &'static str,
    ) -> (Self, Vec<Attribute>) {
        let (our_attrs, rest) = self
            .attrs
            .into_iter()
            .partition(|a| a.path().is_ident(guard));
        self.attrs = rest;
        (self, our_attrs)
    }
}

#[sealed::sealed]
impl DeriveInput for syn::ItemUnion {
    fn skim_attributes(
        mut self,
        guard: &'static str,
    ) -> (Self, Vec<Attribute>) {
        let (our_attrs, rest) = self
            .attrs
            .into_iter()
            .partition(|a| a.path().is_ident(guard));
        self.attrs = rest;
        (self, our_attrs)
    }
}
