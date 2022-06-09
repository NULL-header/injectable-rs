use thiserror::Error;
use to_syn_error::ToSynError;

#[derive(Error, Debug, ToSynError)]
enum InputError {
  #[error("The inject macro needs a trait as input.")]
  NotTrait,
  #[error("The inject macro needs some methods on the trait as input.")]
  Empty,
}

// when this be made with parse, the trait must has some methods.
pub struct Input(syn::ItemTrait);

type Args = syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>;

pub struct Field<'a> {
  pub name: &'a syn::Ident,
  pub output: &'a syn::ReturnType,
  pub args: Args,
}

impl Input {
  pub fn new(i: syn::ItemTrait) -> Input {
    Input(i)
  }
  pub fn get_name(&self) -> &syn::Ident {
    &self.0.ident
  }
  pub fn get_all(&self) -> &syn::ItemTrait {
    &self.0
  }
  pub fn get_fields<'a>(&'a self) -> Vec<Field<'a>> {
    let fields = self.0.items.iter();
    let fields = fields.map(|e| match e {
      syn::TraitItem::Method(e) => Some(e),
      _ => None,
    });
    let fields = fields.fold(Vec::new(), |mut a, e| {
      if let Some(e) = e {
        let field = Field {
          name: &e.sig.ident,
          output: &e.sig.output,
          args: &e.sig.inputs,
        };
        a.push(field);
      }
      a
    });
    fields
  }
}

impl syn::parse::Parse for Input {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let item_trait: syn::ItemTrait = match input.parse() {
      Ok(i) => i,
      Err(e) => {
        return Err(InputError::NotTrait.to_syn_error(e.span()));
      }
    };
    if item_trait.items.len() == 0 {
      return Err(InputError::Empty.to_syn_error(input.span()));
    };
    Ok(Input::new(item_trait))
  }
}

#[cfg(test)]
mod test {

  use super::*;
  use assert_parse::register_assert;
  use quote::quote;
  use rstest::rstest;

  register_assert!(Input, InputError);

  #[rstest]
  fn not_trait(assert: Assert) {
    let input = quote! {1};
    assert.error(input, InputError::NotTrait);
  }

  #[rstest]
  fn empty(assert: Assert) {
    let input = quote! {
        trait Mock{}
    };
    assert.error(input, InputError::Empty);
  }

  #[rstest]
  fn get_fields(assert: Assert) {
    let input = quote! {
        trait Mock{
            fn something();
        }
    };
    assert.ok(input, |input| {
      let fields = input.get_fields();
      if fields.len() > 1 {
        panic!("fields too many on logic.");
      }
      let result = &fields[0];
      assert_eq!(result.name.to_string(), "something".to_string());
    });
  }
}
