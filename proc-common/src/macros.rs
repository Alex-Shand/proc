#[macro_export]
macro_rules! error_at {
    ($target:expr, $message:expr) => {{
        $crate::syn::Error::new(
            $crate::syn::spanned::Spanned::span(&$target),
            $message
        ).to_compile_error()
    }};
}

#[macro_export]
macro_rules! keywords {
    ($($keyword:ident),+ $(,)?) => {
        mod kw {
            $($crate::syn::custom_keyword!{$keyword})+
        }
    }
}

#[macro_export]
macro_rules! tests {
    ($macro:path: pass = $pass:literal fail = $fail:literal) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test() {
                let t = $crate::trybuild::TestCases::new();
                t.pass(format!("{}/*.rs", $pass));
                t.compile_fail(format!("{}/*.rs", $fail));
                $crate::check_expansion($pass, $macro);
            }
        }
    };
}

#[macro_export]
macro_rules! q {
    ($($tt:tt)*) => {
        $crate::testutils::TokenStream($crate::quote::quote!($($tt)*))
    }
}

#[macro_export]
macro_rules! compile_error {
    ($msg:literal) => {
        $crate::q!(compile_error! {$msg})
    }
}
