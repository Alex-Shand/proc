use proc_common::{
    proc_macro2::TokenStream,
    quote::{quote, ToTokens},
    syn::Ident,
};

use crate::arg_spec::ArgSpec;

pub(crate) struct Invoke<'a> {
    item: &'a Ident,
    name: &'a Ident,
    extra_args: Vec<Ident>,
}

impl<'a> Invoke<'a> {
    pub(crate) fn new(
        item: &'a Ident,
        name: &'a Ident,
        arg_spec: &'a ArgSpec,
    ) -> Self {
        Self {
            item,
            name,
            extra_args: arg_spec.idents().collect(),
        }
    }
}

impl ToTokens for Invoke<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Invoke {
            item,
            name,
            extra_args,
        } = self;
        tokens.extend(quote! {
            #name(#(#extra_args,)* #item)
        });
    }
}
