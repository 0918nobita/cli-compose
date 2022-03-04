use proc_macro2::TokenStream;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArgOptError {
    #[error("#[derive(ArgOpt)] can only be applied to structs with single unnamed field or enums")]
    InvalidTypeDef(TokenStream),

    #[error("Literals in #[arg_opt(..)] is not allowed")]
    UnexpectedLit(TokenStream),

    #[error("Metadata in #[arg_opt(..)] is invalid")]
    InvalidMeta(TokenStream),

    #[error("#[arg_opt(long = ..)] must be a string literal")]
    InvalidLongValue(TokenStream),

    #[error("#[arg_opt(short = ..)] must be a char literal")]
    InvalidShortValue(TokenStream),

    #[error("Unexpected key in #[arg_opt(..)]")]
    UnexpectedKey(TokenStream),
}

impl From<ArgOptError> for syn::Error {
    fn from(err: ArgOptError) -> syn::Error {
        match &err {
            ArgOptError::InvalidTypeDef(tokens)
            | ArgOptError::UnexpectedLit(tokens)
            | ArgOptError::InvalidMeta(tokens)
            | ArgOptError::InvalidLongValue(tokens)
            | ArgOptError::InvalidShortValue(tokens)
            | ArgOptError::UnexpectedKey(tokens) => {
                syn::Error::new_spanned(tokens, format!("{}", err))
            }
        }
    }
}

pub type ArgOptResult<T> = Result<T, ArgOptError>;
