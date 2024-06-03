extern crate proc_macro;

mod generate;
mod parse;

use generate::generate;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[macro_use]
extern crate quote;

#[proc_macro_derive(I18nError, attributes(i18n_key, i18n_delegate, i18n_language_codes))]
pub fn derive_i18n_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    generate(&input).into()
}
