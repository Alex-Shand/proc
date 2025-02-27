use common::{
    proc_macro2::TokenStream,
    quote::{quote, ToTokens},
    syn::{Ident, Path},
};

use crate::arg_spec::ArgSpec;

pub(crate) struct ArgumentParser<'a> {
    crate_: &'a Path,
    item: &'a Ident,
    args: &'a Ident,
    name: &'a Ident,
    attribute: &'a Ident,
    arg_spec: &'a ArgSpec,
}

impl<'a> ArgumentParser<'a> {
    pub(crate) fn new(
        crate_: &'a Path,
        item: &'a Ident,
        args: &'a Ident,
        name: &'a Ident,
        attribute: &'a Ident,
        arg_spec: &'a ArgSpec,
    ) -> Self {
        Self {
            crate_,
            item,
            args,
            name,
            attribute,
            arg_spec,
        }
    }
}

impl ToTokens for ArgumentParser<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_ = self.crate_;
        let item = self.item;
        let args = self.args;
        let name = self.name;
        let attribute = self.attribute;
        let item_type = self.arg_spec.item_type();
        tokens.extend(quote! {
            let #item = #crate_::syn::parse_macro_input!(#item as #item_type);
            let (#item, #args) = #crate_::proc_derive_last_argument_must_implement_meta_derive_input(
                #item,
                stringify!(#attribute),
            );
        });
        if self.arg_spec.is_empty() {
            return tokens.extend(quote! {
                if !#args.is_empty() {
                    return #crate_::syn::Error::new_spanned(
                        &#args[0],
                        concat!(
                            "#[derive(",
                            stringify!(#name),
                            ")] expects no item level attributes",
                        ),
                    ).into_compile_error().into();
                }
            });
        }
        let arg_spec = self.arg_spec;
        let matcher = arg_spec.matcher();
        let crate_resolve = arg_spec.crate_resolve();
        tokens.extend(quote! {
            let arg_spec = #arg_spec;
            let #matcher = match #crate_::meta::Meta::parse_attrs(arg_spec, &#args[..]) {
                Ok(result) => result,
                Err(e) => return e.into_compile_error().into()
            };
            #crate_resolve
        });
    }
}
