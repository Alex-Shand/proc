use common::{
    proc_macro2::TokenStream,
    quote::{quote, ToTokens},
    syn::{Ident, Path},
};

use crate::arg_spec::ArgSpec;

pub(crate) struct ArgumentParser<'a> {
    crate_: &'a Path,
    attribute: &'a Ident,
    arg_spec: &'a ArgSpec,
}

impl<'a> ArgumentParser<'a> {
    pub(crate) fn new(
        crate_: &'a Path,
        attribute: &'a Ident,
        arg_spec: &'a ArgSpec,
    ) -> Self {
        Self {
            crate_,
            attribute,
            arg_spec,
        }
    }
}

impl ToTokens for ArgumentParser<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_ = self.crate_;
        let attribute = self.attribute;
        let item_type = self.arg_spec.item_type();
        tokens.extend(quote! {
            let item = #crate_::syn::parse_macro_input!(item as #item_type);
            let (item, top_attributes) = #crate_::proc_derive_last_argument_must_implement_meta_derive_input(
                item,
                stringify!(#attribute),
            );
        });
        if self.arg_spec.is_empty() {
            return;
        }
        let arg_spec = self.arg_spec;
        let matcher = arg_spec.matcher();
        let crate_resolve = arg_spec.crate_resolve();
        tokens.extend(quote! {
            let arg_spec = #arg_spec;
            let #matcher = match #crate_::meta::Meta::parse_attrs(arg_spec, &top_attributes[..]) {
                Ok(result) => result,
                Err(e) => return e.into_compile_error().into()
            };
            #crate_resolve
        });
    }
}
