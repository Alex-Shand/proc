use syn::{parse::Parse, Result};

/// Optional meta argument
///
/// The syntax is identical to [`Required`](super::Required) but it is not an
/// error if the argument is not present. The value exposed to the macro
/// implementation is [`Option<T>`]
///
/// ```rust,ignore
/// // Ok
/// #[my_macro(optional = <something>)]
/// fn item() {}
///
/// // Also ok
/// #[my_macro]
/// fn item() {}
/// ```
#[derive(Debug)]
pub struct Optional<T: Parse> {
    name: &'static str,
    value: Option<T>,
}

impl<T: Parse> Optional<T> {
    #[must_use]
    #[doc(hidden)]
    pub fn new(name: &'static str) -> Self {
        Self { name, value: None }
    }
}

#[sealed::sealed]
impl<T: Parse> super::Meta for Optional<T> {
    type Item = Option<T>;

    fn parse(&mut self, meta: &syn::meta::ParseNestedMeta<'_>) -> Result<bool> {
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

#[cfg(test)]
mod tests {
    use pa::assert_eq;
    use proc_macro2::Span;
    use quote::ToTokens;

    use super::{super::Meta as _, Optional};

    #[test]
    fn parse_present() -> syn::Result<()> {
        let attr: syn::Attribute =
            syn::parse_quote!(#[attribute(optional = true)]);
        let mut parser: Optional<syn::LitBool> = Optional::new("optional");
        attr.parse_nested_meta(|meta| {
            assert!(parser.parse(&meta)?);
            Ok(())
        })?;
        let result = parser.validate()?;
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!("true", result.into_token_stream().to_string());
        Ok(())
    }

    #[test]
    fn parse_absent() -> syn::Result<()> {
        let attr: syn::Attribute = syn::parse_quote!(#[attribute()]);
        let mut parser: Optional<syn::LitBool> = Optional::new("optional");
        attr.parse_nested_meta(|meta| {
            assert!(parser.parse(&meta)?);
            Ok(())
        })?;
        let result = parser.validate()?;
        assert!(result.is_none());
        Ok(())
    }

    #[test]
    fn parse_incorrect() {
        let attr: syn::Attribute = syn::parse_quote!(#[attribute(foo = false)]);
        let mut parser: Optional<syn::LitBool> = Optional::new("optional");
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
        let mut parser: Optional<syn::LitBool> = Optional::new("optional");
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
