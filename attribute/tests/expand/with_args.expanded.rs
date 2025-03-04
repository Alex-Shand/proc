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
    pub fn test(
        arg1: <Required<Arg1> as proc_common::meta::Meta>::Item,
        arg2: <Optional<Arg2> as proc_common::meta::Meta>::Item,
        arg3: <Switch as proc_common::meta::Meta>::Item,
        item: InputType,
    ) -> Result<OutputType> {
        ::core::panicking::panic("not yet implemented")
    }
    let arg_spec = (
        <Required<Arg1>>::new("arg1"),
        <Optional<Arg2>>::new("arg2"),
        <Switch>::new("arg3"),
    );
    let (arg1, arg2, arg3) = match proc_common::meta::parse_bare(
        arg_spec,
        __proc_internal_args.into(),
    ) {
        Ok(result) => result,
        Err(e) => return e.into_compile_error().into(),
    };
    proc_common::proc_attribute_function_must_return_proc_result(
            test(
                arg1,
                arg2,
                arg3,
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
