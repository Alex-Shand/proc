/// Docs
#[proc_macro]
pub fn derive(
    __proc_internal_input: ::proc_macro::TokenStream,
) -> ::proc_macro::TokenStream {
    #[allow(unreachable_pub)]
    #[allow(unnecessary_wraps)]
    #[allow(clippy::needless_pass_by_value)]
    /// Docs
    pub fn derive(item: InputType) -> Result<OutputType> {
        ::core::panicking::panic("not yet implemented")
    }
    proc_common::proc_macro_function_must_return_proc_result(
            derive(
                match ::syn::parse::<_>(__proc_internal_input) {
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
