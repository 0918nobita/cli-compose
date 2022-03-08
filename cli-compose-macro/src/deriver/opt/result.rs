use derive_more::Display;
use proc_macro2::TokenStream;

#[derive(Debug, Display)]
pub enum OptErrKind {
    #[display(fmt = "#[derive(Opt)] can only be applied to empty structs")]
    InvalidStruct,
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
