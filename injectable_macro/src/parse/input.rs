use super::error::{ParseError, ParseErrorKind};

#[derive(Debug)]
pub struct Input {
  body: syn::ItemTrait,
}

impl Input {
  pub fn make_fields(&self) -> Vec<&syn::Ident> {
    let mut fields: Vec<&syn::Ident> = Vec::new();
    self.body.items.iter().for_each(|e| match e {
      syn::TraitItem::Method(e) => {
        fields.push(&e.sig.ident);
      }
      _ => {}
    });
    fields
  }
}

fn parse_input(input: syn::parse::ParseStream) -> Result<Input, ParseError> {
  let body: syn::ItemTrait = match input.parse() {
    Ok(b) => b,
    _ => {
      let kind = ParseErrorKind::NotTrait;
      let error = ParseError::new(kind, None);
      return Err(error);
    }
  };
  if body.items.len() == 0 {
    let kind = ParseErrorKind::Empty;
    let token = Box::new(body);
    let error = ParseError::new(kind, Some(token));
    return Err(error);
  }
  Ok(Input { body })
}

impl syn::parse::Parse for Input {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    match parse_input(input) {
      Err(e) => Err(e.into()),
      Ok(i) => Ok(i),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use quote::quote;

  #[test]
  fn not_trait() {
    let input = quote! {
        1+1
    };
    let result: syn::Result<Input> = syn::parse2(input);
    match result {
      Ok(_) => {
        panic!("unknown error");
      }
      Err(e) => {
        assert_eq!(e.to_string(), ParseErrorKind::NotTrait.to_string());
      }
    }
  }

  #[test]
  fn empty() {
    let input = quote! {
      trait Mock {}
    };
    let result: syn::Result<Input> = syn::parse2(input);
    match result {
      Ok(_) => {
        panic!("unknown error");
      }
      Err(e) => {
        assert_eq!(e.to_string(), ParseErrorKind::Empty.to_string());
      }
    }
  }

  #[test]
  fn fields() {
    let input = quote! {
      trait Mock {
        fn some();
      }
    };
    let result: syn::Result<Input> = syn::parse2(input);
    match result {
      Ok(i) => {
        let fields = i.make_fields();
        let some = fields[0].to_string();
        assert_eq!(some, "some");
      }
      Err(_) => {
        panic!("unknown error");
      }
    };
  }
}
