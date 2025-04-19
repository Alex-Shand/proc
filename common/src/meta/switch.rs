/// Boolean switch meta argument
///
/// Argument format is a single identifier which can be present or absent. The
/// macro implementation recieves [`true`] if the identifier is present and
/// [`false`] if it is absent.
///
/// ```rust,ignore
/// // my_macro recieves true
/// #[my_macro(switch)]
/// fn item() {}
///
/// // my_macro recieves false
/// #[my_macro]
/// fn item() {}
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Switch {
    name: &'static str,
    taken: bool,
}

impl Switch {
    #[must_use]
    #[doc(hidden)]
    pub fn new(name: &'static str) -> Self {
        Self { name, taken: false }
    }
}

#[sealed::sealed]
impl super::RawMeta for Switch {
    type Item = bool;

    fn parse(
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

#[cfg(test)]
mod tests {
    use pa::assert_eq;
    use proc_macro2::Span;

    use super::{super::RawMeta as _, Switch};

    #[test]
    fn parse_present() -> syn::Result<()> {
        let attr: syn::Attribute = syn::parse_quote!(#[attribute(arg)]);
        let mut parser = Switch::new("arg");
        attr.parse_nested_meta(|meta| {
            assert!(parser.parse(&meta)?);
            Ok(())
        })?;
        assert!(parser.validate()?);
        Ok(())
    }

    #[test]
    fn parse_absent() -> syn::Result<()> {
        let attr: syn::Attribute = syn::parse_quote!(#[attribute()]);
        let mut parser = Switch::new("arg");
        attr.parse_nested_meta(|meta| {
            assert!(parser.parse(&meta)?);
            Ok(())
        })?;
        assert!(!parser.validate()?);
        Ok(())
    }

    #[test]
    fn parse_incorrect() {
        let attr: syn::Attribute = syn::parse_quote!(#[attribute(foo = false)]);
        let mut parser = Switch::new("arg");
        let result = attr.parse_nested_meta(|meta| {
            assert!(!parser.parse(&meta)?);
            Err(syn::Error::new(Span::call_site(), "AHHHH!"))
        });
        assert!(result.is_err());
        let Err(error) = result else { unreachable!() };
        assert_eq!(
            r#":: core :: compile_error ! { "AHHHH!" }"#,
            error.into_compile_error().to_string()
        );
    }

    #[test]
    fn invalid_meta_syntax() {
        let attr: syn::Attribute = syn::parse_quote!(#[attribute(...)]);
        let mut parser = Switch::new("arg");
        let result = attr.parse_nested_meta(|meta| {
            assert!(!parser.parse(&meta)?);
            Err(syn::Error::new(Span::call_site(), "AHHHH!"))
        });
        assert!(result.is_err());
        let Err(error) = result else { unreachable!() };
        assert_eq!(
            r#":: core :: compile_error ! { "unexpected token in nested attribute, expected ident" }"#,
            error.into_compile_error().to_string()
        );
    }
}
