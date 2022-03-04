use derive_more::Display;
use proc_macro2::TokenStream;

#[derive(Debug, Display)]
pub enum PosArgErrKind {
    #[display(fmt = "#[derive(PosArg)] can only be applied to structs with single unnamed field")]
    InvalidStruct,

    #[display(fmt = "Literals in #[pos_arg(..)] are not allowed")]
    UnexpectedLit,

    #[display(fmt = "Metadata in #[pos_arg(..)] is invalid")]
    InvalidMeta,

    #[display(fmt = "#[pos_arg(name = ..)] must be a string literal")]
    InvalidNameValue,

    #[display(fmt = "Unexpected key in #[pos_arg(..)]")]
    UnexpectedKey,
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

pub type PosArgResult<T> = Result<T, PosArgErr>;
