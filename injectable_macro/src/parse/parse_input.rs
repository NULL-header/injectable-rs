use super::error::ParseError;

#[derive(Debug)]
pub struct Input {
    body: syn::ItemTrait,
}

pub fn parse_input(input: syn::parse::ParseStream) -> Result<Input, ParseError> {
    let body: syn::ItemTrait = match input.parse() {
        Ok(b) => b,
        _ => {
            return Err(ParseError::NotTrait);
        }
    };
    if body.attrs.len() == 0 {
        return Err(ParseError::Empty);
    }
    Ok(Input { body })
}

#[cfg(test)]
mod test {
    use super::*;
    use quote::quote;

    #[derive(Debug)]
    struct Mock {
        result: Result<Input, ParseError>,
    }

    impl syn::parse::Parse for Mock {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let result = match parse_input(input) {
                Err(e) => {
                    return Err(e.make_syn_error(None as Option<syn::ItemTrait>));
                }
                r => r,
            };
            return Ok(Self { result });
        }
    }

    #[test]
    fn not_trait() {
        let input = quote! {
            1+1
        };
        let result: syn::Result<Mock> = syn::parse2(input);
        match result {
            Ok(_) => {
                panic!("unknown error");
            }
            Err(e) => {
                assert_eq!(e.to_string(), ParseError::NotTrait.to_string());
            }
        }
    }

    #[test]
    fn empty() {
        let input = quote! {
          trait Mock {}
        };
        let result: syn::Result<Mock> = syn::parse2(input);
        match result {
            Ok(_) => {
                panic!("unknown error");
            }
            Err(e) => {
                assert_eq!(e.to_string(), ParseError::Empty.to_string());
            }
        }
    }
}
