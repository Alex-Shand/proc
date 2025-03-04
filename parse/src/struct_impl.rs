use proc_common::{
    proc_macro2::TokenStream,
    quote::{quote, ToTokens},
    syn::{DataStruct, Fields, Ident, Path},
};

pub(crate) struct StructImpl {
    crate_: Path,
    name: Ident,
    fields: Fields,
}

impl StructImpl {
    pub(crate) fn new(crate_: Path, name: Ident, data: DataStruct) -> Self {
        Self {
            crate_,
            name,
            fields: data.fields,
        }
    }

    fn parse_body(&self) -> ParseBody<'_> {
        ParseBody::new(&self.fields)
    }
}

impl ToTokens for StructImpl {
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
    fields: &'a Fields,
}

impl<'a> ParseBody<'a> {
    fn new(fields: &'a Fields) -> Self {
        Self { fields }
    }
}

impl ToTokens for ParseBody<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.fields {
            Fields::Unit => tokens.extend(quote!(Ok(Self))),
            Fields::Unnamed(fields) => {
                let fields =
                    fields.unnamed.iter().map(|_| quote!(input.parse()?));
                tokens.extend(quote! {
                    Ok(Self(#(#fields),*))
                });
            }
            Fields::Named(fields) => {
                let fields = fields.named.iter().map(|f| {
                    let name = f
                        .ident
                        .as_ref()
                        .expect("Named fields should always have idents");
                    quote!(#name: input.parse()?)
                });
                tokens.extend(quote! {
                    Ok(Self { #(#fields),*})
                });
            }
        }
    }
}
