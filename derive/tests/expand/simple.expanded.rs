/// Docs
#[proc_macro_derive(MyDerive, attributes(my_derive))]
pub fn derive(
    __proc_internal_item: ::proc_macro::TokenStream,
) -> ::proc_macro::TokenStream {
    #[allow(unreachable_pub)]
    #[allow(unnecessary_wraps)]
    #[allow(clippy::needless_pass_by_value)]
    /// Docs
    pub fn derive(item: InputType) -> Result<OutputType> {
        ::core::panicking::panic("not yet implemented")
    }
    let __proc_internal_item = match ::syn::parse::<InputType>(__proc_internal_item) {
        ::syn::__private::Ok(data) => data,
        ::syn::__private::Err(err) => {
            return ::syn::__private::TokenStream::from(err.to_compile_error());
        }
    };
    let (__proc_internal_item, __proc_internal_args) = common::proc_derive_last_argument_must_implement_meta_derive_input(
        __proc_internal_item,
        "my_derive",
    );
    if !__proc_internal_args.is_empty() {
        return common::syn::Error::new_spanned(
                &__proc_internal_args[0],
                "#[derive(MyDerive)] expects no item level attributes",
            )
            .into_compile_error()
            .into();
    }
    common::proc_derive_function_must_return_proc_result(derive(__proc_internal_item))
        .into()
}
