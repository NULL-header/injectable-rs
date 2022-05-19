use proc_macro2::Span;

pub trait ToSynError: std::fmt::Display {
  fn to_syn_error(&self, span: Span) -> syn::Error {
    syn::Error::new(span, self.to_string())
  }
}
