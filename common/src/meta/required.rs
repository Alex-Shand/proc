use proc_macro2::Span;
use syn::{parse::Parse, Error, Result};

use super::Meta;

/// Required argument
#[derive(Debug)]
pub struct Required<T: Parse> {
    name: &'static str,
    value: Option<T>,
}

impl<T: Parse> Required<T> {
    /// New
    #[must_use]
    pub fn new(name: &'static str) -> Self {
        Self { name, value: None }
    }
}

impl<T: Parse> Meta for Required<T> {
    type Item = T;

    fn parse_impl(
        &mut self,
        meta: &syn::meta::ParseNestedMeta<'_>,
    ) -> Result<bool> {
        if meta.path.is_ident(self.name) {
            self.value = Some(meta.value()?.parse()?);
            return Ok(true);
        }
        Ok(false)
    }

    fn validate(self) -> Result<Self::Item> {
        if let Some(value) = self.value {
            Ok(value)
        } else {
            Err(Error::new(
                Span::call_site(),
                format_args!("missing required argument: {}", self.name),
            ))
        }
    }
}
