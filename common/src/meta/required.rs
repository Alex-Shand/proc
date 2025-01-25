use proc_macro2::Span;
use syn::{parse::Parse, Error, Result};

use super::{Meta, Optional};

/// Required argument
#[derive(Debug)]
pub struct Required<T: Parse> {
    name: &'static str,
    opt: Optional<T>,
}

impl<T: Parse> Required<T> {
    /// New
    #[must_use]
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            opt: Optional::new(name),
        }
    }
}

impl<T: Parse> Meta for Required<T> {
    type Item = T;

    fn parse_impl(
        &mut self,
        meta: &syn::meta::ParseNestedMeta<'_>,
    ) -> Result<bool> {
        self.opt.parse_impl(meta)
    }

    fn validate(self) -> Result<Self::Item> {
        if let Some(value) = self.opt.validate()? {
            Ok(value)
        } else {
            Err(Error::new(
                Span::call_site(),
                format_args!("missing required argument: {}", self.name),
            ))
        }
    }
}
