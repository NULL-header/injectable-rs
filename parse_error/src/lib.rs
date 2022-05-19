extern crate proc_macro;

mod defined;
mod parse;

use defined::ToSynError;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(ToSynError)]
pub fn parse_error_derive(input: TokenStream) -> TokenStream {
  // let enum_input: syn::DataEnum = match syn::parse(input) {
  //   Ok(i) => i,
  //   Err(e) => {
  //     panic!("");
  //   }
  // };
  quote! {}.into()
}
