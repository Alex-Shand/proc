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

/// Wrapper macro for defining attribute macros
///
/// # Meta Arguments
/// - `crate`: (Optional) Set the path to the [`proc`](crate) crate. Will be
///   inferred from `Cargo.toml` if not present.
/// - `host`: (Optional) If present the _generated_ macro will have the same
///   `crate` argument as this macro with the same semantics as this macro. The
///   value of `host` should be the default name of the 'host' crate, where
///   required helpers may be found.
///
/// # Item
/// `#[proc::attribute]` expects to be applied to a fn item with at least one
/// argument. The function must return [`Result<T>`](Result) where `T`
/// implements [`ToTokens`](quote::ToTokens). An [`Err`] return value from the
/// function is converted into a compile error. See
/// [`ResultFormatter`](util::ResultFormatter) for an alternative way to
/// generate compile errors in situations where returning an [`Err`] is
/// inconvenient.
///
/// Allowed argument forms are as follows:
///
/// Without the `host` meta argument the last argument to the function should
/// have a type which implements [`Parse`](syn::parse::Parse). This defines the
/// item(s) which the macro may recieve as input and will typically be an
/// appropriate object from [`syn`]. Any arguments other than the last, if
/// present, must have one of the types defined in the `meta` module. These will
/// become meta arguments of the generated macro, see individual documentation
/// of each type for details of how they parse arguments.
///
/// ```rust,ignore
/// #[proc::attribute]
/// fn my_attribute(item: proc::syn::ItemFn) -> proc::Result<proc::TokenStream> {
///     todo!()
/// }
///
/// // With meta arguments
/// #[proc::attribute]
/// fn my_attribute2(
///     arg: proc::meta::Required<proc::syn::LitStr>,
///     item: proc::syn::ItemStruct,
/// ) -> proc::Result<proc::TokenStream> {
///     todo!()
/// }
///
/// // Generated macros may be used like so
/// #[my_attribute]
/// fn some_function() {}
///
/// #[my_attribute2(arg = "this is a string")]
/// struct SomeStruct
/// ```
///
/// With the `host` meta argument present the function should have at least two
/// arguments. The first should have type [Path] and will contain the path to
/// the designated host crate when the macro is invoked. The last argument
/// remains the same as above. As with the above, any other arguments will be
/// populated by meta arguments to the generated macro
///
/// ```rust,ignore
/// #[proc::attribute(host = "my_crate")]
/// fn my_attribute(
///     crate_: proc::Path,
///     arg: proc::meta::Required<proc::syn::LitStr>,
///     item: proc::syn::ItemEnum
/// ) -> proc::Result<proc::TokenStream> {
///     todo!()
/// }
///
/// // Allowing the macro to find the host crate on its own
/// #[my_attribute(arg = "this is also a string")]
/// enum SomeEnum {}
///
/// // Or specifying one manually
/// #[my_attribute(crate = an_alias_for_my_crate, arg = "yet another string")]
/// enum AnotherEnum {}
/// ```
pub use proc_attribute::attribute;
pub use proc_common::{
    get_crate, meta, proc_attribute_function_must_return_proc_result,
    proc_derive_function_must_return_proc_result,
    proc_derive_last_argument_must_implement_meta_derive_input,
    proc_macro_function_must_return_proc_result,
    proc_macro2::{Span, TokenStream},
    quote, syn,
    syn::{DeriveInput, ItemEnum, ItemFn, ItemStruct, Path, Result},
    util,
};
/// Wrapper macro for defining derive macros
///
/// # Meta Arguments
/// - `crate`: (Optional) Set the path to the [`proc`](crate) crate. Will be
///   inferred from `Cargo.toml` if not present.
/// - `name`: (Required) The name of the generated derive macro. Typically this
///   would be the same as the trait name which the derive macro is intended to
///   implement e.g `MyTrait`.
/// - `attribute`: (Optional) The name of the derive macro's support attribute
///   (used automatically as a container for item level meta-arguments and may
///   optionally be used for field/variant level meta arguments). If absent it
///   defaults to the value of `name` converted to `snake_case`.
/// - `host`: (Optional) If present the _generated_ macro will have the same
///   `crate` argument as this macro with the same semantics as this macro. The
///   value of `host` should be the default name of the 'host' crate, where
///   required helpers may be found.
///
/// # Item
/// The item requirements for `#[proc::derive]` are identical to
/// [`#[proc::attribute]`](macro@attribute) with the additional condition that
/// the item (last) argument must have a type which implements
/// [DeriveInput](meta::DeriveInput). See documentation for
/// [`#[proc::attribute]`](macro@attribute) for details of the other
/// requirements.
pub use proc_derive::derive;
/// Wrapper macro for defining function-like macros
///
/// # Item
/// `#[proc::function]` expects to be applied to a fn item with at exactly one
/// argument which implements [`Parse`](syn::parse::Parse) The function must
/// return [`Result<T>`](Result) where `T` implements
/// [`ToTokens`](quote::ToTokens). An [`Err`] return value from the function is
/// converted into a compile error. See
/// [`ResultFormatter`](util::ResultFormatter) for an alternative way to
/// generate compile errors in situations where returning an [`Err`] is
/// inconvenient.
pub use proc_function::function;
/// Derive macro for [Parse](syn::parse::Parse)
///
/// # Meta Arguments
/// - `crate`: (Optional) Set the path to the [`proc`](crate) crate. Will be
///   inferred from `Cargo.toml` if not present.
///
/// # Item
/// `#[derive(Parse)]` can be applied to Structs or Enums. Structs are parsed
/// memberwise in declaration order. Enums are parsed into the first matching
/// variant working top to bottom.
pub use proc_parse::Parse;
