use thiserror::Error;
use to_syn_error::ToSynError;

#[derive(Error, Debug, ToSynError)]
enum ArgError {
  #[error("The inject macro needs a ident as the arg.")]
  NotIdent,
  #[error("The inject macro needs the arg.")]
  Empty,
  #[error("The inject macro needs only one arg.")]
  TooMany,
}

pub struct Arg(syn::Ident);

impl Arg {
  pub fn new(a: syn::Ident) -> Arg {
    Arg(a)
  }
  pub fn get_name(&self) -> &syn::Ident {
    &self.0
  }
}

impl syn::parse::Parse for Arg {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    if input.is_empty() {
      return Err(ArgError::Empty.to_syn_error(input.span()));
    }
    let arg: syn::Ident = match input.parse() {
      Ok(a) => a,
      Err(e) => {
        return Err(ArgError::NotIdent.to_syn_error(e.span()));
      }
    };
    if !input.is_empty() {
      return Err(ArgError::TooMany.to_syn_error(input.span()));
    }
    Ok(Self::new(arg))
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use assert_parse::register_assert;
  use quote::quote;
  use rstest::rstest;

  register_assert!(Arg, ArgError);

  #[rstest]
  fn empty(assert: Assert) {
    let arg = quote! {};
    assert.error(arg, ArgError::Empty);
  }

  #[rstest]
  fn too_many(assert: Assert) {
    let arg = quote! {mock,1};
    assert.error(arg, ArgError::TooMany);
  }

  #[rstest]
  fn not_ident(assert: Assert) {
    let arg = quote! {1};
    assert.error(arg, ArgError::NotIdent);
  }

  #[rstest]
  fn ok(assert: Assert) {
    let arg = quote! {mock};
    assert.ok(arg, |_| {});
  }
}
