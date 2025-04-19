use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{self, Attribute, Error, Ident, Pat, PatType, Path, Result, Type};

/// .
#[derive(Debug, Clone)]
pub struct Argument {
    attrs: Vec<Attribute>,
    name: String,
    meta_name: String,
    pat: Pat,
    typ: Type,
    arg_spec: Type,
}

impl Argument {
    ///
    /// # Errors
    ///
    pub fn new(
        name: &str,
        PatType { attrs, pat, ty, .. }: &PatType,
        crate_: &Path,
    ) -> Result<Self> {
        let Pat::Ident(ident) = &**pat else {
            return Err(Error::new_spanned(
                pat,
                format!(
                    "proc::{name} can only parse variable binding arguments"
                ),
            ));
        };
        let attrs = attrs.clone();
        let pat = (**pat).clone();
        let arg_spec = (**ty).clone();
        let typ =
            syn::parse_quote!(<#arg_spec as #crate_::meta::RawMeta>::Item);
        Ok(Self {
            attrs,
            name: ident.ident.to_string(),
            meta_name: ident.ident.to_string(),
            pat,
            typ,
            arg_spec,
        })
    }

    ///.
    #[must_use]
    pub fn crate_ident() -> Ident {
        format_ident!("crate_")
    }

    /// .
    #[must_use]
    pub fn crate_(
        PatType { attrs, pat, ty, .. }: &PatType,
        crate_: &Path,
    ) -> Self {
        Self {
            attrs: attrs.clone(),
            name: String::from("crate_"),
            meta_name: String::from("crate"),
            pat: (**pat).clone(),
            typ: (**ty).clone(),
            arg_spec: syn::parse_quote!(#crate_::meta::Optional<#crate_::Path>),
        }
    }

    /// .
    #[must_use]
    pub fn as_logic_argument(&self) -> TokenStream {
        let Argument {
            attrs,
            name: _,
            meta_name: _,
            pat,
            typ,
            arg_spec: _,
        } = self;
        quote!(#(#attrs)* #pat: #typ)
    }

    /// .
    #[must_use]
    pub fn as_parser_object(&self) -> TokenStream {
        let Argument {
            meta_name,
            arg_spec,
            ..
        } = self;
        quote!(<#arg_spec>::new(#meta_name))
    }

    /// .
    #[must_use]
    pub fn ident(&self) -> Ident {
        format_ident!("{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use pa::assert_eq;
    use rstest::rstest;

    use crate::argument::Argument;

    #[test]
    fn constructor_success() {
        assert!(Argument::new(
            "test",
            &syn::parse_quote!(x: Type),
            &syn::parse_quote!(crate)
        )
        .is_ok());
    }

    #[test]
    fn constructor_fail() {
        assert!(Argument::new(
            "test",
            &syn::parse_quote!(Type(_): Type),
            &syn::parse_quote!(crate)
        )
        .is_err());
    }

    #[rstest]
    #[case(
        Argument::new(
            "test",
            &syn::parse_quote!(x: Type),
            &syn::parse_quote!(crate),
        ),
        "x : < Type as crate :: meta :: RawMeta > :: Item",
    )]
    #[case(
        Ok(Argument::crate_(
            &syn::parse_quote!(x: Type),
            &syn::parse_quote!(crate),
        )),
        "x : Type",
    )]
    fn as_logic_argument(
        #[case] arg: syn::Result<Argument>,
        #[case] expected: &str,
    ) -> syn::Result<()> {
        assert_eq!(expected, arg?.as_logic_argument().to_string());
        Ok(())
    }

    #[rstest]
    #[case(
        Argument::new(
            "test",
            &syn::parse_quote!(x: Type),
            &syn::parse_quote!(crate),
        ),
        r#"< Type > :: new ("x")"#,
    )]
    #[case(
        Ok(Argument::crate_(
            &syn::parse_quote!(x: Type),
            &syn::parse_quote!(crate),
        )),
        r#"< crate :: meta :: Optional < crate :: Path > > :: new ("crate")"#,
    )]
    fn as_parser_object(
        #[case] arg: syn::Result<Argument>,
        #[case] expected: &str,
    ) -> syn::Result<()> {
        assert_eq!(expected, arg?.as_parser_object().to_string());
        Ok(())
    }

    #[rstest]
    #[case(
        Argument::new(
            "test",
            &syn::parse_quote!(x: Type),
            &syn::parse_quote!(crate),
        ),
        "x",
    )]
    #[case(
        Ok(Argument::crate_(
            &syn::parse_quote!(x: Type),
            &syn::parse_quote!(crate),
        )),
        "crate_",
    )]
    fn ident(
        #[case] arg: syn::Result<Argument>,
        #[case] expected: &str,
    ) -> syn::Result<()> {
        assert_eq!(expected, arg?.ident().to_string());
        Ok(())
    }
}
