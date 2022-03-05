use derive_more::Display;
use proc_macro2::TokenStream;

#[derive(Debug, Display)]
pub enum ArgOptErrKind {
    #[display(
        fmt = "#[derive(ArgOpt)] can only be applied to structs with single unnamed field or enums"
    )]
    InvalidTypeDef,
}

pub struct ArgOptErr {
    kind: ArgOptErrKind,
    tokens: TokenStream,
}

impl ArgOptErr {
    pub fn new(kind: ArgOptErrKind, tokens: TokenStream) -> Self {
        Self { kind, tokens }
    }
}

impl From<ArgOptErr> for syn::Error {
    fn from(err: ArgOptErr) -> syn::Error {
        syn::Error::new_spanned(err.tokens, err.kind.to_string())
    }
}
