//! proc macro helpers

#![warn(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_results)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
//#![deny(dead_code)]
#![warn(clippy::pedantic)]

pub use proc0::attribute;
pub use proc1::function;
pub use proc_common::{error_at, import_crate};
pub use proc_macro2;
pub use proc_macro2::TokenStream;
pub use quote;
pub use syn;
pub use trybuild;

/// Generally useful imports
pub mod prelude {
    pub use super::{error_at, import_crate, proc_macro2, quote, quote::quote, syn, TokenStream};
}

/// Helpers for testing proc macros
pub mod testutils {
    use std::fs;
    use std::path::Path;

    use anyhow::{anyhow, Context, Result};
    use pretty_assertions::assert_eq;

    pub use proc2::tests;
    pub use proc_common::testutils::TokenStream;

    const PROC_START_MARKER: &str = "//PROC: Start";
    const PROC_END_MARKER: &str = "//PROC: End";

    #[doc(hidden)]
    pub fn expand_and_compare<F>(run_macro: F, input: &str, output: &str)
    where
        F: Fn(proc_macro2::TokenStream) -> Result<TokenStream, String>,
    {
        let input = read_tokens(Path::new(input))
            .unwrap_or_else(|_| panic!("Failed to tokenize the input file '{}'", input));
        let output = TokenStream(
            read_tokens(Path::new(output))
                .unwrap_or_else(|_| panic!("Failed to tokenize output file '{}'", output)),
        );
        assert_eq!(run_macro(input).expect("Macro failed"), output);
    }

    fn read_tokens(path: &Path) -> Result<proc_macro2::TokenStream> {
        fs::read_to_string(path)
            .with_context(|| format!("Failed to read file '{}'", path.display()))?
            .lines()
            .skip_while(|line| *line != PROC_START_MARKER)
            .take_while(|line| *line != PROC_END_MARKER)
            .collect::<Vec<_>>()
            .join("\n")
            .parse()
            .map_err(|e| anyhow!("{}", e))
    }
}

/// Helpers for writing custom syntax parsers
pub mod parseutils {
    pub use proc_common::keywords;
}
