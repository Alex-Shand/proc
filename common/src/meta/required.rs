use proc_macro2::Span;
use syn::{parse::Parse, spanned::Spanned as _, Error, Result};

use super::Optional;

/// Required meta argument
///
/// Argument format is a key value pair. It is an error to omit the argument.
///
/// ```rust,ignore
/// // Success
/// #[my_macro(required = <something>)]
/// fn item() {}
///
/// // error: missing required argument: required
/// #[my_macro]
/// fn item() {}
/// ```
#[derive(Debug)]
pub struct Required<T: Parse> {
    name: &'static str,
    anchor: Span,
    opt: Optional<T>,
}

impl<T: Parse> Required<T> {
    #[must_use]
    #[doc(hidden)]
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            anchor: Span::call_site(),
            opt: Optional::new(name),
        }
    }
}

#[sealed::sealed]
impl<T: Parse> super::MetaParser for Required<T> {
    type Item = T;

    fn parse(&mut self, meta: &syn::meta::ParseNestedMeta<'_>) -> Result<bool> {
        self.anchor = meta.path.span();
        self.opt.parse(meta)
    }

    fn validate(self) -> Result<Self::Item> {
        if let Some(value) = self.opt.validate()? {
            Ok(value)
        } else {
            Err(required_argument_error(self.name, self.anchor))
        }
    }
}

/// Provides standardized formatting for a missing required meta argument
#[must_use]
pub fn required_argument_error(name: &'static str, span: Span) -> Error {
    Error::new(span, format_args!("missing required argument: {name}"))
}

#[cfg(test)]
mod tests {
    use pa::assert_eq;
    use proc_macro2::Span;
    use quote::ToTokens;

    use super::{super::MetaParser as _, Required};

    #[test]
    fn parse_present() -> syn::Result<()> {
        let attr: syn::Attribute = syn::parse_quote!(#[attribute(arg = true)]);
        let mut parser: Required<syn::LitBool> = Required::new("arg");
        attr.parse_nested_meta(|meta| {
            assert!(parser.parse(&meta)?);
            Ok(())
        })?;
        let result = parser.validate()?;
        assert_eq!("true", result.into_token_stream().to_string());
        Ok(())
    }

    #[test]
    fn parse_absent() -> syn::Result<()> {
        let attr: syn::Attribute = syn::parse_quote!(#[attribute()]);
        let mut parser: Required<syn::LitBool> = Required::new("arg");
        attr.parse_nested_meta(|meta| {
            assert!(parser.parse(&meta)?);
            Ok(())
        })?;
        let result = parser.validate();
        assert!(result.is_err());
        let Err(error) = result else { unreachable!() };
        assert_eq!(
            r#":: core :: compile_error ! { "missing required argument: arg" }"#,
            error.into_compile_error().to_string()
        );
        Ok(())
    }

    #[test]
    fn parse_incorrect() {
        let attr: syn::Attribute = syn::parse_quote!(#[attribute(foo = false)]);
        let mut parser: Required<syn::LitBool> = Required::new("arg");
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
        let mut parser: Required<syn::LitBool> = Required::new("arg");
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
