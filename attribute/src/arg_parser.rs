use common::{
    proc_macro2::TokenStream,
    quote::{quote, ToTokens},
    syn::{Ident, Path},
};

use crate::arg_spec::ArgSpec;

pub(crate) struct ArgumentParser<'a> {
    crate_: &'a Path,
    args: &'a Ident,
    arg_spec: &'a ArgSpec,
}

impl<'a> ArgumentParser<'a> {
    pub(crate) fn new(
        crate_: &'a Path,
        args: &'a Ident,
        arg_spec: &'a ArgSpec,
    ) -> Self {
        Self {
            crate_,
            args,
            arg_spec,
        }
    }
}

impl ToTokens for ArgumentParser<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.arg_spec.is_empty() {
            return;
        }
        let crate_ = self.crate_;
        let args = self.args;
        let arg_spec = self.arg_spec;
        let matcher = arg_spec.matcher();
        let crate_resolve = arg_spec.crate_resolve();
        tokens.extend(quote! {
            let arg_spec = #arg_spec;
            let #matcher = match #crate_::meta::Meta::parse_bare(arg_spec, #args.into()) {
                Ok(result) => result,
                Err(e) => return e.into_compile_error().into()
            };
            #crate_resolve
        });
    }
}
