use std::fmt::Display;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Error;

pub use self::{
    enums::{EnumMatcher, VariantExpander},
    for_each_field::ForEachField,
    result_formatter::ResultFormatter,
    struct_enum_deriver::{EnumData, StructData, StructEnumDeriver},
};

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
