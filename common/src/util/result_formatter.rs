use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Result;

/// Wrapper to implement [`ToTokens`] for [`Result`]. The [`Ok`] variant is
/// formatted according to its [`ToTokens`] implementation. The [`Err`] variant
/// is converted to a call to [`compile_error`]
#[derive(Debug)]
pub struct ResultFormatter<T: ToTokens>(pub Result<T>);

impl<T: ToTokens> ToTokens for ResultFormatter<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.0 {
            Ok(result) => result.to_tokens(tokens),
            Err(e) => tokens.extend(e.to_compile_error()),
        }
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use quote::ToTokens;

    use super::ResultFormatter;

    #[test]
    fn result_formatter_success() {
        let code: syn::File = syn::parse_quote! {
            fn main() {}
        };
        let formatter = ResultFormatter(Ok(code));
        assert_eq!("fn main () { }", formatter.into_token_stream().to_string());
    }

    #[test]
    fn result_formatter_error() {
        let formatter: ResultFormatter<syn::File> =
            ResultFormatter(Err(syn::Error::new(Span::call_site(), "AHHHHH!")));
        assert_eq!(
            r#":: core :: compile_error ! { "AHHHHH!" }"#,
            formatter.into_token_stream().to_string()
        );
    }
}
