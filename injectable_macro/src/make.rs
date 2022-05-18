use super::parse::Input;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn make(input: Input) -> TokenStream2 {
  let body = input.body;
  quote! {
      #body
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use rstest::*;

  #[fixture]
  fn input() -> Input {
    let input = quote! {
      trait Mock {
        fn some();
      }
    };
    syn::parse2(input).unwrap()
  }

  #[rstest]
  fn tmp(input: Input) {
    let result = make(input);
    dbg!(result.to_string());
    panic!("holder");
  }
}
