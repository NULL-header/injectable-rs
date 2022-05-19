extern crate proc_macro;

mod defined;

use proc_macro::TokenStream;

#[proc_macro_derive()]
pub fn parse_error() {}
