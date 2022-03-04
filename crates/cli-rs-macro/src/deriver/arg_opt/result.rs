use derive_more::Display;
use proc_macro2::TokenStream;

#[derive(Debug, Display)]
pub enum ArgOptErrKind {
    #[display(
        fmt = "#[derive(ArgOpt)] can only be applied to structs with single unnamed field or enums"
    )]
    InvalidTypeDef,

    #[display(fmt = "Literals in #[arg_opt(..)] is not allowed")]
    UnexpectedLit,

    #[display(fmt = "Metadata in #[arg_opt(..)] is invalid")]
    InvalidMeta,

    #[display(fmt = "#[arg_opt(long = ..)] must be a string literal")]
    InvalidLongValue,

    #[display(fmt = "#[arg_opt(short = ..)] must be a char literal")]
    InvalidShortValue,

    #[display(fmt = "Unexpected key in #[arg_opt(..)]")]
    UnexpectedKey,
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

pub type ArgOptResult<T> = Result<T, ArgOptErr>;
