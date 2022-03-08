use derive_more::Display;
use proc_macro2::TokenStream;

#[derive(Debug, Display)]
pub enum PosArgErrKind {
    #[display(fmt = "#[derive(PosArg)] can only be applied to structs with single unnamed field")]
    InvalidStruct,
}

pub struct PosArgErr {
    kind: PosArgErrKind,
    tokens: TokenStream,
}

impl PosArgErr {
    pub fn new(kind: PosArgErrKind, tokens: TokenStream) -> Self {
        Self { kind, tokens }
    }
}

impl From<PosArgErr> for syn::Error {
    fn from(err: PosArgErr) -> Self {
        syn::Error::new_spanned(err.tokens, err.kind.to_string())
    }
}
