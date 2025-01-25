use super::Meta;

/// Switch
#[derive(Debug, Copy, Clone)]
pub struct Switch {
    name: &'static str,
    taken: bool,
}

impl Switch {
    /// New
    #[must_use]
    pub fn new(name: &'static str) -> Self {
        Self { name, taken: false }
    }
}

impl Meta for Switch {
    type Item = bool;

    fn parse_impl(
        &mut self,
        meta: &syn::meta::ParseNestedMeta<'_>,
    ) -> syn::Result<bool> {
        if meta.path.is_ident(self.name) {
            self.taken = true;
            return Ok(true);
        }
        Ok(false)
    }

    fn validate(self) -> syn::Result<Self::Item> {
        Ok(self.taken)
    }
}
