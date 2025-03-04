use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Field, Fields, Index};

/// Helper for handling struct fields in a standardised way.
///
/// The [`ToTokens`] implementation calls the provided closure with each field
/// (for non-unit structs) and renders the result comma separated surrounded by
/// {} or () as appropriate. Unit structs result in no tokens.
#[expect(missing_debug_implementations)]
pub struct ForEachField<'a, F: Fn(Index, &Field) -> TokenStream>(
    pub &'a Fields,
    pub F,
);

impl<F: Fn(Index, &Field) -> TokenStream> ToTokens for ForEachField<'_, F> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.0 {
            Fields::Unit => (),
            Fields::Named(fields) => {
                let fields = fields
                    .named
                    .iter()
                    .enumerate()
                    .map(|(i, f)| (self.1)(Index::from(i), f));
                tokens.extend(quote! { { #(#fields),* } });
            }
            Fields::Unnamed(fields) => {
                let fields = fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, f)| (self.1)(Index::from(i), f));
                tokens.extend(quote! { ( #(#fields),* ) });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use pa::assert_eq;
    use quote::{quote, ToTokens};
    use syn::ItemStruct;

    use super::ForEachField;

    #[rstest::rstest]
    #[case(syn::parse_quote!(struct Unit;), "")]
    #[case(
        syn::parse_quote!(struct Tuple(usize, String);),
        "([idx : 0] [field : usize] , [idx : 1] [field : String])",
    )]
    #[case(
        syn::parse_quote!(struct Named { first: usize, second: String }),
        "{ [idx : 0] [field : first : usize] , [idx : 1] [field : second : String] }",
    )]
    fn test(#[case] item: ItemStruct, #[case] expected: &str) {
        let result = ForEachField(
            &item.fields,
            |idx, field| quote!([idx: #idx][field: #field]),
        );
        assert_eq!(expected, result.into_token_stream().to_string());
    }
}
