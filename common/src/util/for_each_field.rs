use std::{borrow::Cow, iter};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Field, Fields, Ident, Index, Type};

use super::standard_ident;

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

/// Iterator over fields with [Index]
///
/// Same functionality as [ForEachField] without any opinions on formatting
pub fn fields(fields: &Fields) -> impl Iterator<Item = (Index, &Field)> {
    match fields {
        Fields::Unit => ThreeIterators::One(iter::empty()),
        Fields::Named(fields) => ThreeIterators::Two(
            fields
                .named
                .iter()
                .enumerate()
                .map(|(i, f)| (Index::from(i), f)),
        ),
        Fields::Unnamed(fields) => ThreeIterators::Three(
            fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, f)| (Index::from(i), f)),
        ),
    }
}

/// Iterator over field name and type
///
/// Field names are generated using [standard_ident]
pub fn field_names_and_types(
    fields: &Fields,
) -> impl Iterator<Item = (Cow<'_, Ident>, &Type)> {
    self::fields(fields).map(|(idx, field)| {
        let ident = standard_ident(&idx, field);
        (ident, &field.ty)
    })
}

enum ThreeIterators<
    'a,
    I1: Iterator<Item = (Index, &'a Field)>,
    I2: Iterator<Item = (Index, &'a Field)>,
    I3: Iterator<Item = (Index, &'a Field)>,
> {
    One(I1),
    Two(I2),
    Three(I3),
}

impl<'a, I1, I2, I3> Iterator for ThreeIterators<'a, I1, I2, I3>
where
    I1: Iterator<Item = (Index, &'a Field)>,
    I2: Iterator<Item = (Index, &'a Field)>,
    I3: Iterator<Item = (Index, &'a Field)>,
{
    type Item = (Index, &'a Field);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ThreeIterators::One(i) => i.next(),
            ThreeIterators::Two(i) => i.next(),
            ThreeIterators::Three(i) => i.next(),
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
