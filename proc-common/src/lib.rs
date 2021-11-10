use proc_macro2::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::format_ident;

pub use quote;
pub use syn;
pub use trybuild;

mod macros;

pub mod procutils;
pub mod testutils;

fn get_crate_path<S: AsRef<str>>(name: S) -> TokenStream {
    let ident = format_ident!("{}", name.as_ref());
    quote::quote!(::#ident)
}

pub fn import_crate(name: &'static str) -> TokenStream {
    match crate_name(name) {
        Ok(FoundCrate::Itself) => quote::quote!(crate),
        Ok(FoundCrate::Name(name)) => get_crate_path(name),
        Err(_) => get_crate_path(name),
    }
}
