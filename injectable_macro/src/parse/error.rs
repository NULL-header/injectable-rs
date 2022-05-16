use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseErrorKind {
    #[error("The inject macro can not use with an empty trait.")]
    Empty,
    #[error("The code is not a trait, or it is wrong with syntax.")]
    NotTrait,
}

pub struct ParseError<'a> {
    kind: ParseErrorKind,
    token: Option<Box<dyn quote::ToTokens + 'a>>,
}

impl<'a> std::convert::Into<syn::Error> for ParseError<'a> {
    fn into(self) -> syn::Error {
        syn::Error::new_spanned(self.token, self.kind.to_string())
    }
}

impl<'a> ParseError<'a> {
    pub fn new(kind: ParseErrorKind, token: Option<Box<dyn quote::ToTokens + 'a>>) -> Self {
        ParseError { kind, token }
    }
}
