use derive_more::Display;
use proc_macro2::TokenStream;

#[derive(Debug, Display)]
pub enum ArgOptErrorKind {
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

pub struct ArgOptError {
    kind: ArgOptErrorKind,
    tokens: TokenStream,
}

impl ArgOptError {
    pub fn new(kind: ArgOptErrorKind, tokens: TokenStream) -> Self {
        Self { kind, tokens }
    }
}

impl From<ArgOptError> for syn::Error {
    fn from(err: ArgOptError) -> syn::Error {
        syn::Error::new_spanned(err.tokens, err.kind.to_string())
    }
}

pub type ArgOptResult<T> = Result<T, ArgOptError>;
