use common::{
    proc_macro2::TokenStream,
    quote::{format_ident, quote, ToTokens},
    syn::{self, DataEnum, Error, Expr, Fields, Ident, Path, Result, Variant},
    ResultFormatter,
};

pub(crate) struct EnumImpl {
    crate_: Path,
    parse: Path,
    name: Ident,
    variants: Vec<Variant>,
}

impl EnumImpl {
    pub(crate) fn new(
        crate_: Path,
        parse: Path,
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
            parse,
            name,
            variants,
        })
    }

    fn parse_body(&self) -> ParseBody<'_> {
        ParseBody::new(&self.crate_, &self.parse, &self.name, &self.variants)
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
    parse: &'a Path,
    self_type: &'a Ident,
    variants: &'a [Variant],
}

impl<'a> ParseBody<'a> {
    fn new(
        crate_: &'a Path,
        parse: &'a Path,
        self_type: &'a Ident,
        variants: &'a [Variant],
    ) -> Self {
        Self {
            crate_,
            parse,
            self_type,
            variants,
        }
    }
}

impl ToTokens for ParseBody<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let variant_structs = self.variants.iter().map(|v| {
            VariantStruct::new(self.crate_, self.parse, self.self_type, v)
        });
        let parsers =
            self.variants.iter().map(Parser::new).map(ResultFormatter);
        tokens.extend(quote! {
            #(#variant_structs)*
            let lookahead = input.lookahead1();
            #(#parsers)*
            return Err(lookahead.error());
        });
    }
}

struct VariantStruct<'a> {
    crate_: &'a Path,
    parse: &'a Path,
    self_type: &'a Ident,
    variant: &'a Variant,
}

impl<'a> VariantStruct<'a> {
    fn new(
        crate_: &'a Path,
        parse: &'a Path,
        self_type: &'a Ident,
        variant: &'a Variant,
    ) -> Self {
        Self {
            crate_,
            parse,
            self_type,
            variant,
        }
    }

    fn struct_name(ident: &Ident) -> Ident {
        format_ident!("__Proc_Internal_{ident}")
    }

    fn matcher(&self) -> Matcher<'_> {
        Matcher::new(&self.variant.fields)
    }
}

impl ToTokens for VariantStruct<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_ = self.crate_;
        let parse = self.parse;
        let self_type = self.self_type;
        let variant = &self.variant.ident;
        let ident = Self::struct_name(&self.variant.ident);
        let fields = &self.variant.fields;
        let matcher = self.matcher();
        tokens.extend(quote! {
            #[derive(#parse::Parse)]
            #[parse(crate = #crate_)]
            #[allow(non_camel_case_types)]
            struct #ident #fields;

            impl From<#ident> for #self_type {
                fn from(#ident #matcher: #ident) -> #self_type {
                    #self_type::#variant #matcher
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

struct Parser<'a> {
    matcher: Expr,
    ident: &'a Ident,
}

impl<'a> Parser<'a> {
    fn new(variant: &'a Variant) -> Result<Self> {
        let err = || {
            Error::new_spanned(
                &variant.ident,
                "unit enum variants cannot be parsed",
            )
        };
        let matcher = match &variant.fields {
            Fields::Named(fields) => &fields.named.first().ok_or_else(err)?.ty,
            Fields::Unnamed(fields) => {
                &fields.unnamed.first().ok_or_else(err)?.ty
            }
            Fields::Unit => return Err(err()),
        };
        Ok(Self {
            matcher: syn::parse2(matcher.into_token_stream())?,
            ident: &variant.ident,
        })
    }
}

impl ToTokens for Parser<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let matcher = &self.matcher;
        let ident = VariantStruct::struct_name(self.ident);
        tokens.extend(quote! {
            if lookahead.peek(#matcher) {
                return Ok(input.parse::<#ident>()?.into())
            }
        });
    }
}
