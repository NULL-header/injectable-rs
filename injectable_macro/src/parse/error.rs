use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("The inject macro can not use with an empty trait.")]
    Empty,
    #[error("The code is not a trait, or it is wrong with syntax.")]
    NotTrait,
}

impl ParseError {
    pub fn make_syn_error<T>(self, token: Option<T>) -> syn::Error
    where
        T: quote::ToTokens,
    {
        syn::Error::new_spanned(token, self.to_string())
    }
}
