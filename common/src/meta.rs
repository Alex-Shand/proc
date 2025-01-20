use proc_macro2::TokenStream;
use syn::{parse::Parser as _, Result};

pub use self::{optional::Optional, required::Required};

mod optional;
mod required;
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
    fn parse(mut self, tokens: TokenStream) -> Result<Self::Item> {
        let parser = syn::meta::parser(|meta| {
            if self.parse_impl(&meta)? {
                return Ok(());
            }
            Err(meta.error("unrecognised argument"))
        });
        let () = parser.parse2(tokens)?;
        self.validate()
    }
}
