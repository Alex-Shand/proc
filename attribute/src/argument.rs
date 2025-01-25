use common::{
    proc_macro2::TokenStream,
    quote::{format_ident, quote},
    syn::{self, Attribute, Error, Ident, Pat, PatType, Path, Result, Type},
};

#[derive(Debug, Clone)]
pub(crate) struct Argument {
    attrs: Vec<Attribute>,
    name: String,
    meta_name: String,
    pat: Pat,
    typ: Type,
    arg_spec: Type,
}

impl Argument {
    pub(crate) fn new(
        PatType { attrs, pat, ty, .. }: &PatType,
        crate_: &Path,
    ) -> Result<Self> {
        let Pat::Ident(ident) = &**pat else {
            return Err(Error::new_spanned(
                pat,
                "proc::attribute can only parse variable binding arguments",
            ));
        };
        let attrs = attrs.clone();
        let pat = (**pat).clone();
        let arg_spec = (**ty).clone();
        let typ = syn::parse_quote!(<#arg_spec as #crate_::meta::Meta>::Item);
        Ok(Self {
            attrs,
            name: ident.ident.to_string(),
            meta_name: ident.ident.to_string(),
            pat,
            typ,
            arg_spec,
        })
    }

    pub(crate) fn crate_(
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

    pub(crate) fn as_logic_argument(&self) -> TokenStream {
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

    pub(crate) fn as_parser_object(&self) -> TokenStream {
        let Argument {
            meta_name,
            arg_spec,
            ..
        } = self;
        quote!(<#arg_spec>::new(#meta_name))
    }

    pub(crate) fn ident(&self) -> Ident {
        format_ident!("{}", self.name)
    }
}
