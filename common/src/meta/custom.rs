use syn::{
    parse::{Parse, ParseStream},
    Result,
};

/// Custom meta arg format
///
/// Syntax is `key <something>` where `<something>` is defined by the [`Parse`]
/// implementation of `T`. The value exposed to the macro is `T`, if the key is
/// not present `T`'s [`Default`] implementation is used.
///
/// ```rust,ignore
/// // E.g using Expr's Parse implementation
/// #[my_macro(key 1 + 2)]
/// fn item() {}
/// ```
#[derive(Debug)]
pub struct Custom<T: Meta> {
    name: &'static str,
    value: Option<T>,
}

impl<T: Meta> Custom<T> {
    #[must_use]
    #[doc(hidden)]
    pub fn new(name: &'static str) -> Self {
        Self { name, value: None }
    }
}

#[sealed::sealed]
impl<T: Meta> super::MetaParser for Custom<T> {
    type Item = T;

    fn parse(&mut self, meta: &syn::meta::ParseNestedMeta<'_>) -> Result<bool> {
        if meta.path.is_ident(self.name) {
            self.value = Some(Meta::parse(meta.input)?);
            return Ok(true);
        }
        Ok(false)
    }

    fn validate(self) -> Result<Self::Item> {
        Ok(self.value.unwrap_or_default())
    }
}

/// Custom meta argument parser
pub trait Meta: Default {
    #[allow(missing_docs)]
    fn parse(stream: ParseStream<'_>) -> Result<Self>;
}

impl<T: Parse> Meta for Option<T> {
    fn parse(stream: ParseStream<'_>) -> Result<Self> {
        Ok(Some(stream.parse()?))
    }
}

#[cfg(test)]
mod tests {
    use pa::assert_eq;
    use proc_macro2::Span;
    use quote::ToTokens;

    use super::{super::MetaParser as _, Custom};

    #[test]
    fn parse_present() -> syn::Result<()> {
        let attr: syn::Attribute =
            syn::parse_quote!(#[attribute(custom 1 + 2)]);
        let mut parser: Custom<Option<syn::Expr>> = Custom::new("custom");
        attr.parse_nested_meta(|meta| {
            assert!(parser.parse(&meta)?);
            Ok(())
        })?;
        let result = parser.validate()?;
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!("1 + 2", result.into_token_stream().to_string());
        Ok(())
    }

    #[test]
    fn parse_absent() -> syn::Result<()> {
        let attr: syn::Attribute = syn::parse_quote!(#[attribute()]);
        let mut parser: Custom<Option<syn::Expr>> = Custom::new("custom");
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
        let mut parser: Custom<Option<syn::Expr>> = Custom::new("custom");
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
        let mut parser: Custom<Option<syn::Expr>> = Custom::new("custom");
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
