use std::iter;

use common::{
    proc_macro2::TokenStream,
    quote::{quote, ToTokens},
    syn::{
        self, Error, FnArg, Ident, ItemFn, LitStr, PatType, Path, Result,
        Signature,
    },
};

use crate::argument::Argument;

pub(crate) struct ArgSpec {
    crate_: Path,
    host: Option<LitStr>,
    args: Vec<Argument>,
}

impl ArgSpec {
    pub(crate) fn new(
        sig: &Signature,
        crate_: Path,
        host: Option<LitStr>,
    ) -> Result<Self> {
        let inputs = sig.inputs.iter().collect::<Vec<_>>();
        if let Some(a @ FnArg::Receiver(_)) = inputs.first() {
            return Err(Error::new_spanned(
                a,
                "proc::attribute cannot be applied to receiver methods",
            ));
        }

        // If host was passed to the macro then the first argument is reserved
        // for the injected crate. The last argument is always the item. Any
        // others have to be parsed
        let args = match (&host, &inputs[..]) {
            (Some(_), [c, args @ .., _]) => iter::once(Ok(Argument::crate_(
                extract_arg(c),
                &crate_,
            )))
            .chain(
                args.iter()
                    .copied()
                    .map(|a| Argument::new(extract_arg(a), &crate_)),
            )
            .collect::<Result<_>>()?,
            (None, [args @ .., _]) => args
                .iter()
                .copied()
                .map(|a| Argument::new(extract_arg(a), &crate_))
                .collect::<Result<_>>()?,
            (_, []) => return Err(Error::new_spanned(sig, "proc::attribute logic function must have at least one argument")),
            (Some(_), [_]) => return Err(Error::new_spanned(sig, "proc::attribute function must have two arguments if host is used")),
        };

        Ok(ArgSpec { crate_, host, args })
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.args.is_empty()
    }

    pub(crate) fn patch_parsable_args(&self, mut function: ItemFn) -> ItemFn {
        // Last argument is the macro item, we checked it exists already
        let item = function.sig.inputs.last().unwrap();
        let args = self.args.iter().map(Argument::as_logic_argument);
        function.sig.inputs = syn::parse_quote!(#(#args,)* #item);
        function
    }

    pub(crate) fn idents(&self) -> impl Iterator<Item = Ident> + use<'_> {
        self.args.iter().map(Argument::ident)
    }

    pub(crate) fn matcher(&self) -> TokenStream {
        let idents = self.idents().collect::<Vec<_>>();
        if idents.len() == 1 {
            let ident = &idents[0];
            quote!(#ident)
        } else {
            quote!((#(#idents),*))
        }
    }

    pub(crate) fn crate_resolve(&self) -> TokenStream {
        let Some(host) = &self.host else {
            return TokenStream::new();
        };
        let crate_ = &self.crate_;
        quote! {
            let crate_ = if let Some(c) = crate_ {
                c
            } else {
                match #crate_::get_crate(#host) {
                    Ok(c) => c,
                    Err(e) => return e.into_compile_error().into()
                }
            };
        }
    }
}

impl ToTokens for ArgSpec {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self
            .args
            .iter()
            .map(Argument::as_parser_object)
            .collect::<Vec<_>>();
        tokens.extend(if args.len() == 1 {
            let arg = &args[0];
            quote!(#arg)
        } else {
            quote!((#(#args),*))
        });
    }
}

fn extract_arg(arg: &FnArg) -> &PatType {
    match arg {
        FnArg::Receiver(_) => unreachable!(),
        FnArg::Typed(arg) => arg,
    }
}
