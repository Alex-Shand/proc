//! proc
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
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::let_underscore_untyped)]
#![allow(clippy::similar_names)]

pub use attribute::attribute;
pub use common::{
    get_crate, meta, proc_attribute_function_must_return_proc_result,
    proc_derive_last_argument_must_implement_meta_derive_input,
    proc_macro2::TokenStream,
    quote, syn,
    syn::{DeriveInput, ItemEnum, ItemStruct, Path, Result},
};
pub use derive::derive;
pub use parse::Parse;
