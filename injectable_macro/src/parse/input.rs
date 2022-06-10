use quote::quote;
use regex::Regex;
use syn::spanned::Spanned;
use thiserror::Error;
use to_syn_error::ToSynError;

#[derive(Error, Debug, ToSynError)]
enum InputError {
  #[error("The inject macro needs a trait as input.")]
  NotTrait,
  #[error("The inject macro needs some methods on the trait as input.")]
  Empty,
  #[error("The inject macro needs methods only.")]
  NotMethod,
  #[error("The inject macro needs instance methods only.")]
  StaticMethod,
}

pub struct Input(syn::ItemTrait);

type Args = syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>;

pub struct Field<'a> {
  pub name: &'a syn::Ident,
  pub output: &'a syn::ReturnType,
  // pub self_arg: &'a Args,
  pub args: &'a Args,
}

impl Input {
  pub fn new(i: syn::ItemTrait) -> Input {
    Input(i)
  }
  pub fn get_all(&self) -> &syn::ItemTrait {
    &self.0
  }
  pub fn name(&self) -> &syn::Ident {
    &self.0.ident
  }
  pub fn fields<'a>(&'a self) -> Vec<Field<'a>> {
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
    let items = &item_trait.items;
    if items.len() == 0 {
      return Err(InputError::Empty.to_syn_error(input.span()));
    };
    for field in items.iter() {
      let field = match field {
        syn::TraitItem::Method(f) => f,
        some => {
          return Err(InputError::NotMethod.to_syn_error(some.span()));
        }
      };
      let args = &field.sig.inputs;
      let instance_arg = match args.first() {
        Some(a) => a,
        None => {
          return Err(InputError::StaticMethod.to_syn_error(field.span()));
        }
      };
      let instance_arg = quote! {#instance_arg}.to_string();
      let reg_self = Regex::new("self").unwrap();
      if !reg_self.is_match(&instance_arg) {
        return Err(InputError::StaticMethod.to_syn_error(instance_arg.span()));
      }
    }
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
  fn not_method(assert: Assert) {
    let input = quote! {
        trait Mock{
            type Aa=&str;
        }
    };
    assert.error(input, InputError::NotMethod);
  }

  #[rstest]
  fn static_method_with_empty_arg(assert: Assert) {
    let input = quote! {
        trait Mock{
            fn something();
        }
    };
    assert.error(input, InputError::StaticMethod);
  }

  #[rstest]
  fn static_method_with_arg(assert: Assert) {
    let input = quote! {
        trait Mock{
            fn something(arg:&str);
        }
    };
    assert.error(input, InputError::StaticMethod);
  }

  #[rstest]
  fn get_fields_name(assert: Assert) {
    let input = quote! {
        trait Mock{
            fn something(&self);
        }
    };
    assert.ok(input, |input| {
      let fields = input.fields();
      if fields.len() > 1 {
        panic!("fields too many on logic.");
      }
      let result = &fields[0];
      assert_eq!(result.name.to_string(), "something".to_string());
    });
  }

  #[rstest]
  fn get_fields_output_with_empty(assert: Assert) {
    let input = quote! {
        trait Mock{
            fn something(&self);
        }
    };
    assert.ok(input, |input| {
      let fields = input.fields();
      if fields.len() > 1 {
        panic!("fields too many on logic.");
      }
      let result = fields[0].output;
      let result = quote!(#result).to_string();
      assert_eq!(result, "".to_string());
    });
  }

  #[rstest]
  fn get_fields_output_with_type(assert: Assert) {
    let input = quote! {
        trait Mock{
            fn something(&self)->&str;
        }
    };
    assert.ok(input, |input| {
      let fields = input.fields();
      if fields.len() > 1 {
        panic!("fields too many on logic.");
      }
      let result = fields[0].output;
      let result = quote!(#result).to_string();
      assert_eq!(result, "-> & str".to_string());
    });
  }
}
