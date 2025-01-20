use syn::{parse::Parse, Result};

use super::Meta;

/// Optional argument
#[derive(Debug)]
pub struct Optional<T: Parse> {
    name: &'static str,
    value: Option<T>,
}

impl<T: Parse> Optional<T> {
    /// New
    #[must_use]
    pub fn new(name: &'static str) -> Self {
        Self { name, value: None }
    }
}

impl<T: Parse> Meta for Optional<T> {
    type Item = Option<T>;

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
        Ok(self.value)
    }
}
