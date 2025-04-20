#[macro_use]
mod horrible_hack;

macro_rules! reverse {
    ([][$($reversed:ident),*]) => {
        ($($reversed),*)
    };
    ([$first:ident $(, $idents:ident)*][$($reversed:ident),*]) => {
        reverse!([$($idents),*][$first $(, $reversed)*])
    };
}

macro_rules! reverse_and_concat {
    ([][$($reversed:ident),*]) => {
        $($reversed)||*
    };
    ([$first:ident $(, $idents:ident)*][$($reversed:ident),*]) => {
        reverse_and_concat!([$($idents),*][$first $(, $reversed)*])
    };
}

macro_rules! expand_validate {
    ([$self:ident][][$($tt:tt)*][$($idents:ident),*]) => {
        $($tt)*
        return Ok(reverse!([$($idents),*][]));
    };
    ([$self:ident][$t:ident$(, $rest:ident)*][$($tt:tt)*][$($idents:ident),*]) => {
        expand_validate!([$self][$($rest),*][#[allow(non_snake_case)] let $t = index_tuple!($self, $t$(, $rest)*).validate()?;$($tt)*][$($idents),*])
    };
}

macro_rules! expand_parse {
    ([$self:ident, $meta:ident][][$($tt:tt)*][$($idents:ident),*]) => {
        $($tt)*
        return Ok(reverse_and_concat!([$($idents),*][]));
    };
    ([$self:ident, $meta:ident][$t:ident$(, $rest:ident)*][$($tt:tt)*][$($idents:ident),*]) => {
        expand_parse!([$self, $meta][$($rest),*][#[allow(non_snake_case)] let $t = index_tuple!($self, $t$(, $rest)*).parse($meta)?;$($tt)*][$($idents),*])
    }
}

macro_rules! tuple_impl {
    ($t:ident) => {};
    ($t:ident, $($rest:ident),+) => {
        #[sealed::sealed]
        impl<$t: super::MetaParser, $($rest: super::MetaParser),*> super::MetaParser for ($t, $($rest),*) {
            type Item = ($t::Item, $($rest::Item),*);

            fn parse(&mut self, meta: &syn::meta::ParseNestedMeta<'_>) -> syn::Result<bool> {
                expand_parse!([self, meta][$t, $($rest),*][][$t, $($rest),*]);
            }

            fn validate(self) -> syn::Result<Self::Item> {
                expand_validate!([self][$t, $($rest),*][][$t, $($rest),*]);
            }
        }
        tuple_impl!($($rest),*);
    }
}

tuple_impl!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y,
    Z
);

#[cfg(test)]
mod tests {
    use pa::assert_eq;
    use proc_macro2::Span;
    use quote::ToTokens as _;
    use rstest::rstest;

    use super::super::{MetaParser as _, Optional, Required, Switch};

    #[rstest]
    #[case(
        syn::parse_quote!(#[attribute(required = true, optional = false, switch)]),
        ("true", Some("false"), true),
    )]
    #[case(
        syn::parse_quote!(#[attribute(optional = false, switch, required = true)]),
        ("true", Some("false"), true),
    )]
    #[case(
        syn::parse_quote!(#[attribute(switch, required = true, optional = false)]),
        ("true", Some("false"), true),
    )]
    #[case(
        syn::parse_quote!(#[attribute(required = true)]),
        ("true", None, false),
    )]
    fn parse_success(
        #[case] attr: syn::Attribute,
        #[case] expected: (&str, Option<&str>, bool),
    ) -> syn::Result<()> {
        let mut parser: (
            Required<syn::LitBool>,
            Optional<syn::LitBool>,
            Switch,
        ) = (
            Required::new("required"),
            Optional::new("optional"),
            Switch::new("switch"),
        );
        attr.parse_nested_meta(|meta| {
            assert!(parser.parse(&meta)?);
            Ok(())
        })?;
        let actual = parser.validate()?;
        assert_eq!(expected.0, actual.0.into_token_stream().to_string());
        assert_eq!(
            expected.1.map(ToOwned::to_owned),
            actual.1.map(|l| l.into_token_stream().to_string())
        );
        assert_eq!(expected.2, actual.2);
        Ok(())
    }

    #[test]
    fn parse_fail() -> syn::Result<()> {
        let attr: syn::Attribute = syn::parse_quote!(#[attribute()]);
        let mut parser: (
            Required<syn::LitBool>,
            Optional<syn::LitBool>,
            Switch,
        ) = (
            Required::new("required"),
            Optional::new("optional"),
            Switch::new("switch"),
        );
        attr.parse_nested_meta(|meta| {
            assert!(parser.parse(&meta)?);
            Ok(())
        })?;
        let result = parser.validate();
        assert!(result.is_err());
        let Err(error) = result else { unreachable!() };
        assert_eq!(
            r#":: core :: compile_error ! { "missing required argument: required" }"#,
            error.into_compile_error().to_string()
        );
        Ok(())
    }

    #[test]
    fn parse_incorrect() {
        let attr: syn::Attribute = syn::parse_quote!(#[attribute(foo = false)]);
        let mut parser: (
            Required<syn::LitBool>,
            Optional<syn::LitBool>,
            Switch,
        ) = (
            Required::new("required"),
            Optional::new("optional"),
            Switch::new("switch"),
        );
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
        let mut parser: (
            Required<syn::LitBool>,
            Optional<syn::LitBool>,
            Switch,
        ) = (
            Required::new("required"),
            Optional::new("optional"),
            Switch::new("switch"),
        );
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
