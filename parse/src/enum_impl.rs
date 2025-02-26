use common::{
    proc_macro2::TokenStream,
    quote::{format_ident, quote, ToTokens},
    syn::{DataEnum, Error, Fields, Ident, Path, Result, Variant},
};

pub(crate) struct EnumImpl {
    crate_: Path,
    name: Ident,
    variants: Vec<Variant>,
}

impl EnumImpl {
    pub(crate) fn new(
        crate_: Path,
        name: Ident,
        data: DataEnum,
    ) -> Result<Self> {
        let variants = data.variants.into_iter().collect::<Vec<_>>();
        if variants.is_empty() {
            return Err(Error::new_spanned(
                name,
                "cannot #[derive(Parse)] on an empty enum",
            ));
        }
        Ok(Self {
            crate_,
            name,
            variants,
        })
    }

    fn parse_body(&self) -> ParseBody<'_> {
        ParseBody::new(&self.crate_, &self.name, &self.variants)
    }
}

impl ToTokens for EnumImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_ = &self.crate_;
        let name = &self.name;
        let parse_body = self.parse_body();
        tokens.extend(quote! {
            impl #crate_::syn::parse::Parse for #name {
                fn parse(input: #crate_::syn::parse::ParseStream<'_>) -> #crate_::syn::Result<Self> {
                    #parse_body
                }
            }
        });
    }
}

struct ParseBody<'a> {
    crate_: &'a Path,
    self_type: &'a Ident,
    variants: &'a [Variant],
}

impl<'a> ParseBody<'a> {
    fn new(
        crate_: &'a Path,
        self_type: &'a Ident,
        variants: &'a [Variant],
    ) -> Self {
        Self {
            crate_,
            self_type,
            variants,
        }
    }
}

impl ToTokens for ParseBody<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let variant_structs = self
            .variants
            .iter()
            .map(|v| VariantStruct::new(self.crate_, self.self_type, v));
        let parsers = self.variants.iter().map(Parser);
        tokens.extend(quote! {
            #(#variant_structs)*
            #(#parsers)*
            return Err(_err);
        });
    }
}

struct VariantStruct<'a> {
    crate_: &'a Path,
    self_type: &'a Ident,
    variant: &'a Variant,
}

impl<'a> VariantStruct<'a> {
    fn new(
        crate_: &'a Path,
        self_type: &'a Ident,
        variant: &'a Variant,
    ) -> Self {
        Self {
            crate_,
            self_type,
            variant,
        }
    }

    fn matcher(&self) -> Matcher<'_> {
        Matcher::new(&self.variant.fields)
    }
}

impl ToTokens for VariantStruct<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_ = self.crate_;
        let self_type = self.self_type;
        let ident = &self.variant.ident;
        let fields = &self.variant.fields;
        let matcher = self.matcher();
        tokens.extend(quote! {
            #[derive(#crate_::Parse)] struct #ident #fields;

            impl From<#ident> for #self_type {
                fn from(#ident #matcher: #ident) -> #self_type {
                    #self_type::#ident #matcher
                }
            }
        });
    }
}

struct Matcher<'a> {
    fields: &'a Fields,
}

impl<'a> Matcher<'a> {
    fn new(fields: &'a Fields) -> Self {
        Self { fields }
    }
}

impl ToTokens for Matcher<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.fields {
            Fields::Unit => (),
            Fields::Unnamed(fields) => {
                let mut counter = 0;
                let idents = fields.unnamed.iter().map(|_| {
                    let ident = format_ident!("_{counter}");
                    counter += 1;
                    ident
                });
                tokens.extend(quote!((#(#idents),*)));
            }
            Fields::Named(fields) => {
                let idents = fields.named.iter().map(|f| &f.ident);
                tokens.extend(quote!({#(#idents),*}));
            }
        }
    }
}

struct Parser<'a>(&'a Variant);

impl ToTokens for Parser<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.0.ident;
        tokens.extend(quote! {
            let _err = match input.parse::<#ident>() {
                Ok(result) => return Ok(result.into()),
                Err(err) => err,
            };
        });
    }
}
