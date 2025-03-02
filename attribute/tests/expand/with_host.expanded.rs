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
    pub fn test(crate_: Path, item: InputType) -> Result<OutputType> {
        ::core::panicking::panic("not yet implemented")
    }
    let arg_spec = <common::meta::Optional<common::Path>>::new("crate");
    let crate_ = match common::meta::parse_bare(arg_spec, __proc_internal_args.into()) {
        Ok(result) => result,
        Err(e) => return e.into_compile_error().into(),
    };
    let crate_ = if let Some(c) = crate_ {
        c
    } else {
        match common::get_crate("foo") {
            Ok(c) => c,
            Err(e) => return e.into_compile_error().into(),
        }
    };
    common::proc_attribute_function_must_return_proc_result(
            test(
                crate_,
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
