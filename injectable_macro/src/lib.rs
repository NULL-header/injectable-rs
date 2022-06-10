extern crate proc_macro;
mod make;
mod parse;
use parse::{Arg, Input};
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn injectable(arg: TokenStream, input: TokenStream) -> TokenStream {
  let arg: Arg = match syn::parse(arg) {
    Ok(a) => a,
    Err(e) => {
      return e.to_compile_error().into();
    }
  };
  let input: Input = match syn::parse(input) {
    Ok(a) => a,
    Err(e) => {
      return e.to_compile_error().into();
    }
  };
  let all = input.get_all();
  let trait_name = input.name();
  let target_name = arg.get_name();
  let fields = &input.fields();
  let fields: Vec<_> = fields
    .iter()
    .map(|e| {
      let name = e.name;
      let output = e.output;
      let output = match output {
        syn::ReturnType::Type(e, a) => quote_spanned! {a.span()=>#e #a},
        _ => quote_spanned! {output.span()=>},
      };
      let span = name.span();
      quote_spanned! {span=>
          fn #name()#output {

          }
      }
    })
    .collect();

  quote! {
      #all
      impl #trait_name for #target_name{
          #(#fields)*
      }
  }
  .into()
}
