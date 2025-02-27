/// Docs
#[proc_macro_attribute]
pub fn test(
    __proc_internal_args: ::proc_macro::TokenStream,
    __proc_internal_item: ::proc_macro::TokenStream,
) -> ::proc_macro::TokenStream {
    #[allow(unreachable_pub)]
    #[allow(unnecessary_wraps)]
    #[allow(clippy::needless_pass_by_value)]
    /// Docs
    pub fn test(item: InputType) -> Result<OutputType> {
        ::core::panicking::panic("not yet implemented")
    }
    common::proc_attribute_function_must_return_proc_result(
            test(
                match ::syn::parse::<_>(__proc_internal_item) {
                    ::syn::__private::Ok(data) => data,
                    ::syn::__private::Err(err) => {
                        return ::syn::__private::TokenStream::from(
                            err.to_compile_error(),
                        );
                    }
                },
            ),
        )
        .into()
}
