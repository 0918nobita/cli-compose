use derive_more::Display;
use proc_macro2::TokenStream;

#[derive(Debug, Display)]
pub enum OptErrKind {
    #[display(fmt = "#[derive(Opt)] can only be applied to empty structs")]
    InvalidStruct,

    #[display(fmt = "Literals in #[opt(..)] are not allowed")]
    UnexpectedLit,

    #[display(fmt = "Metadata in #[opt(..)] is invalid")]
    InvalidMeta,

    #[display(fmt = "#[opt(long = ..)] must be a string literal")]
    InvalidLongValue,

    #[display(fmt = "#[opt(short = ..)] must be a char literal")]
    InvalidShortValue,

    #[display(fmt = "Unexpected key in #[opt(..)]")]
    UnexpectedKey,
}

pub struct OptErr {
    kind: OptErrKind,
    tokens: TokenStream,
}

impl OptErr {
    pub fn new(kind: OptErrKind, tokens: TokenStream) -> Self {
        Self { kind, tokens }
    }
}

impl From<OptErr> for syn::Error {
    fn from(err: OptErr) -> Self {
        syn::Error::new_spanned(err.tokens, err.kind.to_string())
    }
}

pub type OptResult<T> = Result<T, OptErr>;
