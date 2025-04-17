use proc_common::{
    proc_macro2::TokenStream,
    quote::{format_ident, quote, ToTokens},
    syn::{Error, Fields, Ident, Path, Result, Variant},
    util::{standard_ident, ForEachField},
};

pub(crate) struct EnumImpl<'a> {
    crate_: &'a Path,
    parse: &'a Path,
    ident: &'a Ident,
    variants: &'a [Variant],
}

impl<'a> EnumImpl<'a> {
    pub(crate) fn new(
        crate_: &'a Path,
        parse: &'a Path,
        ident: &'a Ident,
        variants: &'a [Variant],
    ) -> Result<Self> {
        if variants.is_empty() {
            return Err(Error::new_spanned(
                ident,
                "cannot #[derive(Parse)] on an empty enum",
            ));
        }
        Ok(Self {
            crate_,
            parse,
            ident,
            variants,
        })
    }

    fn parse_body(&self) -> ParseBody<'_> {
        ParseBody::new(self.crate_, self.parse, self.ident, self.variants)
    }
}

impl ToTokens for EnumImpl<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_ = self.crate_;
        let name = self.ident;
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
        let crate_ = self.crate_;
        let variant_structs = self
            .variants
            .iter()
            .map(|v| VariantStruct::new(crate_, self.parse, self.self_type, v));
        let parsers = self.variants.iter().map(Parser::new);
        tokens.extend(quote! {
            #(#variant_structs)*
            fn __maybe_parse<P: #crate_::syn::parse::Parse>(input: #crate_::syn::parse::ParseStream<'_>) -> #crate_::syn::Result<P> {
                let fork = input.fork();
                let result = fork.parse();
                if result.is_ok() {
                    use #crate_::syn::parse::discouraged::Speculative as _;
                    input.advance_to(&fork);
                }
                result
            }
            let mut __errors = Vec::new();
            #(#parsers)*
            Err(__errors.into_iter().reduce(|mut acc, e| {
                acc.combine(e);
                acc
            }).expect("__errors list is unexpectedly empty"))
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
        let ident = Self::struct_name(variant);
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
        ForEachField(self.fields, |idx, field| {
            standard_ident(&idx, field).into_token_stream()
        })
        .to_tokens(tokens);
    }
}

struct Parser {
    target_type: Ident,
}

impl Parser {
    fn new(variant: &'_ Variant) -> Self {
        Self {
            target_type: VariantStruct::struct_name(&variant.ident),
        }
    }
}

impl ToTokens for Parser {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let target_type = &self.target_type;
        tokens.extend(quote! {
            match __maybe_parse::<#target_type>(input) {
                Ok(result) => return Ok(result.into()),
                Err(e) => __errors.push(e)
            }
        });
    }
}
