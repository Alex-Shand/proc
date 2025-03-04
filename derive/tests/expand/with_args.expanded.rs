/// Docs
#[proc_macro_derive(MyDerive, attributes(my_derive))]
pub fn derive(
    __proc_internal_item: ::proc_macro::TokenStream,
) -> ::proc_macro::TokenStream {
    #[allow(unreachable_pub)]
    #[allow(unnecessary_wraps)]
    #[allow(clippy::needless_pass_by_value)]
    /// Docs
    pub fn derive(
        arg1: <Required<Arg1> as proc_common::meta::Meta>::Item,
        arg2: <Optional<Arg2> as proc_common::meta::Meta>::Item,
        arg3: <Switch as proc_common::meta::Meta>::Item,
        item: InputType,
    ) -> Result<OutputType> {
        ::core::panicking::panic("not yet implemented")
    }
    let __proc_internal_item = match ::syn::parse::<InputType>(__proc_internal_item) {
        ::syn::__private::Ok(data) => data,
        ::syn::__private::Err(err) => {
            return ::syn::__private::TokenStream::from(err.to_compile_error());
        }
    };
    let (__proc_internal_item, __proc_internal_args) = proc_common::proc_derive_last_argument_must_implement_meta_derive_input(
        __proc_internal_item,
        "my_derive",
    );
    let arg_spec = (
        <Required<Arg1>>::new("arg1"),
        <Optional<Arg2>>::new("arg2"),
        <Switch>::new("arg3"),
    );
    let (arg1, arg2, arg3) = match proc_common::meta::parse_attrs(
        arg_spec,
        &__proc_internal_args[..],
    ) {
        Ok(result) => result,
        Err(e) => return e.into_compile_error().into(),
    };
    proc_common::proc_derive_function_must_return_proc_result(
            derive(arg1, arg2, arg3, __proc_internal_item),
        )
        .into()
}
