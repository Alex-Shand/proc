use std::{borrow::Cow, fmt::Display};

use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens};
use syn::{Error, Field, Ident, Index};

pub use self::{
    crate_and::CrateAnd,
    enums::{EnumFieldMatcher, EnumMatcher, VariantExpander},
    for_each_field::{field_names_and_types, fields, ForEachField},
    result_formatter::ResultFormatter,
    struct_enum_deriver::{EnumData, StructData, StructEnumDeriver},
};

mod crate_and;
mod enums;
mod for_each_field;
mod result_formatter;
mod struct_enum_deriver;

/// Shorthand for constructing an [`Error`] then immediately calling
/// [`into_compile_error`](Error::into_compile_error)
pub fn compile_error(
    tokens: impl ToTokens,
    message: impl Display,
) -> TokenStream {
    Error::new_spanned(tokens, message).into_compile_error()
}

/// Convert the `idx`, `field` pair from [`ForEachField`] to a standard
/// [`Ident`]. If the field is named the result is the field's [`Ident`]. For an
/// unnamed field the returned ident is `_<idx>`.
#[must_use]
pub fn standard_ident<'a>(idx: &Index, field: &'a Field) -> Cow<'a, Ident> {
    if let Some(ident) = &field.ident {
        Cow::Borrowed(ident)
    } else {
        Cow::Owned(format_ident!("_{}", idx.index))
    }
}

#[cfg(test)]
mod tests {
    use pa::assert_eq;
    use quote::quote;

    #[test]
    fn compile_error() {
        let result = super::compile_error(quote!(), "AHHHHH!");
        assert_eq!(
            r#":: core :: compile_error ! { "AHHHHH!" }"#,
            result.to_string()
        );
    }
}
