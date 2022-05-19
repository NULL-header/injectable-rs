use proc_macro2::Span;
use thiserror::Error;

#[derive(Debug, Error)]
enum InputError {
  #[error("The macro needs an enum.")]
  NotEnum,
}

impl InputError {
  pub fn to_syn_error(&self, span: Span) -> syn::Error {
    syn::Error::new(span, self.to_string())
  }
}

pub struct Input(syn::ItemEnum);

impl syn::parse::Parse for Input {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let input: syn::ItemEnum = match input.parse() {
      Ok(i) => i,
      Err(e) => {
        return Err(InputError::NotEnum.to_syn_error(e.span()));
      }
    };
    Ok(Self(input))
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use assert_parse::make_assert;
  use quote::quote;
  use rstest::*;

  type Assert = assert_parse::Assert<Input, InputError>;

  #[fixture]
  fn assert() -> Assert {
    make_assert()
  }

  #[rstest]
  fn not_enum(assert: Assert) {
    let input = quote! {
      1
    };
    assert.error(input, InputError::NotEnum);
  }
}
