use common::{
    proc_macro2::TokenStream,
    quote::{quote, ToTokens},
    syn::Ident,
};

use crate::arg_spec::ArgSpec;

pub(crate) struct Invoke<'a> {
    name: &'a Ident,
    extra_args: Vec<Ident>,
}

impl<'a> Invoke<'a> {
    pub(crate) fn new(name: &'a Ident, arg_spec: &'a ArgSpec) -> Self {
        Self {
            name,
            extra_args: arg_spec.idents().collect(),
        }
    }
}

impl ToTokens for Invoke<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Invoke { name, extra_args } = self;
        tokens.extend(quote! {
            #name(#(#extra_args,)* item)
        });
    }
}
