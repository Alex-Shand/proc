use proc_macro2::TokenStream;
use syn::{parse::Parser as _, Attribute, Result};

pub use self::{optional::Optional, required::Required, switch::Switch};

mod optional;
mod required;
mod switch;
mod tuple;

/// For meta argument parsing
pub trait Meta: Sized {
    /// .
    type Item;

    ///
    /// # Errors
    ///
    fn parse_impl(
        &mut self,
        meta: &syn::meta::ParseNestedMeta<'_>,
    ) -> Result<bool>;

    ///
    /// # Errors
    ///
    fn validate(self) -> Result<Self::Item>;

    ///
    /// # Errors
    ///
    fn parse_bare(mut self, tokens: TokenStream) -> Result<Self::Item> {
        let parser = syn::meta::parser(|meta| {
            if self.parse_impl(&meta)? {
                return Ok(());
            }
            Err(meta.error("unrecognised argument"))
        });
        let () = parser.parse2(tokens)?;
        self.validate()
    }

    ///
    /// # Errors
    ///
    fn parse_attrs(mut self, attrs: &[Attribute]) -> Result<Self::Item> {
        for attr in attrs {
            attr.parse_nested_meta(|meta| {
                if self.parse_impl(&meta)? {
                    return Ok(());
                }
                Err(meta.error("unrecognised argument"))
            })?;
        }
        self.validate()
    }
}

/// .
pub trait DeriveInput: Sized {
    /// .
    fn skim_attributes(self, guard: &'static str) -> (Self, Vec<Attribute>);
}

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
