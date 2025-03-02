use common::{
    proc_macro2::TokenStream,
    quote::{quote, ToTokens},
    syn::{Ident, Path},
};

use crate::arg_spec::ArgSpec;

pub(crate) struct ArgumentParser<'a> {
    crate_: &'a Path,
    name: &'a Ident,
    args: &'a Ident,
    arg_spec: &'a ArgSpec,
}

impl<'a> ArgumentParser<'a> {
    pub(crate) fn new(
        crate_: &'a Path,
        name: &'a Ident,
        args: &'a Ident,
        arg_spec: &'a ArgSpec,
    ) -> Self {
        Self {
            crate_,
            name,
            args,
            arg_spec,
        }
    }
}

impl ToTokens for ArgumentParser<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_ = self.crate_;
        let name = self.name;
        let args = self.args;
        if self.arg_spec.is_empty() {
            return tokens.extend(quote! {
                if !#args.is_empty() {
                    return #crate_::syn::Error::new_spanned(
                        #args,
                        concat!("#[", stringify!(#name), "] expects no arguments"),
                    ).into_compile_error().into();
                }
            });
        }
        let arg_spec = self.arg_spec;
        let matcher = arg_spec.matcher();
        let crate_resolve = arg_spec.crate_resolve();
        tokens.extend(quote! {
            let arg_spec = #arg_spec;
            let #matcher = match #crate_::meta::parse_bare(arg_spec, #args.into()) {
                Ok(result) => result,
                Err(e) => return e.into_compile_error().into()
            };
            #crate_resolve
        });
    }
}
