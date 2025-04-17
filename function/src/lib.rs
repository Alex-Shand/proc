//! derive
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_results)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::let_underscore_untyped)]
#![allow(clippy::similar_names)]

use proc_common::{
    proc_macro2::TokenStream,
    quote::{format_ident, quote, ToTokens},
    syn::{Attribute, Error, FnArg, Ident, ItemFn, Path, Result, Signature},
};

// Documented in the re-export from proc
#[allow(missing_docs)]
#[proc_attribute::attribute(crate = proc_common, host = "proc")]
pub fn function(crate_: Path, item: ItemFn) -> Result<FunctionMacro> {
    FunctionMacro::new(crate_, item)
}

struct FunctionMacro {
    crate_: Path,
    input: Ident,
    logic: ItemFn,
}

impl FunctionMacro {
    fn new(crate_: Path, logic: ItemFn) -> Result<Self> {
        Self::validate(&logic.sig)?;
        Ok(Self {
            crate_,
            input: format_ident!("__proc_internal_input"),
            logic,
        })
    }

    fn validate(sig: &Signature) -> Result<()> {
        let err = "proc::function must be applied to a function with exactly one non-receiver argument";
        if sig.inputs.is_empty() {
            return Err(Error::new_spanned(&sig.ident, err));
        }
        if sig.inputs.len() != 1
            || matches!(sig.inputs.first().unwrap(), FnArg::Receiver(_))
        {
            return Err(Error::new_spanned(&sig.inputs, err));
        }
        Ok(())
    }

    fn attrs(&self) -> &[Attribute] {
        &self.logic.attrs
    }

    fn name(&self) -> &Ident {
        &self.logic.sig.ident
    }
}

impl ToTokens for FunctionMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_ = &self.crate_;
        let input = &self.input;
        let attrs = self.attrs();
        let name = self.name();
        let logic = &self.logic;
        tokens.extend(quote! {
            #(#attrs)*
            #[proc_macro]
            pub fn #name(#input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
                #[allow(unreachable_pub)]
                #[allow(unnecessary_wraps)]
                #[allow(clippy::needless_pass_by_value)]
                #logic
                #crate_::proc_macro_function_must_return_proc_result(
                    #name(#crate_::syn::parse_macro_input!(#input))
                ).into()
            }
        });
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn expand() {
        macrotest::expand("tests/expand/*.rs");
    }

    #[test]
    fn ui() {
        trybuild::TestCases::new().compile_fail("tests/ui/*.rs");
    }
}
