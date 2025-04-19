use syn::{parenthesized, parse::Parse, Result, Token};

/// List meta argument
///
/// Syntax is `key(value1, value2, ...)`. The value exposed to the macro is [`Vec<T>`]. If
/// the key is not present the result is an empty `Vec`
///
/// ```rust,ignore
/// // Ok
/// #[my_macro(list(<something>, <something else>))]
/// fn item() {}
///
/// // Results in an empty Vec passed to the macro
/// #[my_macro(list())]
/// fn item() {}
///
/// // Also results in an empty Vec passed to the macro
/// #[my_macro]
/// fn item() {}
/// ```
#[derive(Debug)]
pub struct List<T: Parse> {
    name: &'static str,
    value: Vec<T>,
}

impl<T: Parse> List<T> {
    #[must_use]
    #[doc(hidden)]
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            value: Vec::new(),
        }
    }
}

#[sealed::sealed]
impl<T: Parse> super::RawMeta for List<T> {
    type Item = Vec<T>;

    fn parse(&mut self, meta: &syn::meta::ParseNestedMeta<'_>) -> Result<bool> {
        if meta.path.is_ident(self.name) {
            let content;
            let _ = parenthesized!(content in meta.input);
            self.value = content
                .parse_terminated(T::parse, Token![,])?
                .into_iter()
                .collect();
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

    use super::{super::RawMeta as _, List};

    #[test]
    fn parse_full_list() -> syn::Result<()> {
        let attr: syn::Attribute =
            syn::parse_quote!(#[attribute(list(true, false, true))]);
        let mut parser: List<syn::LitBool> = List::new("list");
        attr.parse_nested_meta(|meta| {
            assert!(parser.parse(&meta)?);
            Ok(())
        })?;
        let result = parser.validate()?;
        let result = result
            .into_iter()
            .map(|r| r.into_token_stream().to_string())
            .collect::<Vec<_>>();
        assert_eq!(["true", "false", "true"], result[..]);
        Ok(())
    }

    #[test]
    fn parse_empty_list() -> syn::Result<()> {
        let attr: syn::Attribute = syn::parse_quote!(#[attribute(list())]);
        let mut parser: List<syn::LitBool> = List::new("list");
        attr.parse_nested_meta(|meta| {
            assert!(parser.parse(&meta)?);
            Ok(())
        })?;
        let result = parser.validate()?;
        assert!(result.is_empty());
        Ok(())
    }

    #[test]
    fn parse_absent() -> syn::Result<()> {
        let attr: syn::Attribute = syn::parse_quote!(#[attribute()]);
        let mut parser: List<syn::LitBool> = List::new("list");
        attr.parse_nested_meta(|meta| {
            assert!(parser.parse(&meta)?);
            Ok(())
        })?;
        let result = parser.validate()?;
        assert!(result.is_empty());
        Ok(())
    }

    #[test]
    fn parse_incorrect() {
        let attr: syn::Attribute = syn::parse_quote!(#[attribute(foo = false)]);
        let mut parser: List<syn::LitBool> = List::new("list");
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
        let mut parser: List<syn::LitBool> = List::new("list");
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
