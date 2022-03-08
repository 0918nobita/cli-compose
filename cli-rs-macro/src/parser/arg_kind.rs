use syn::Ident;

#[derive(Debug)]
pub enum ArgKind {
    PosArg,
    ArgOpt,
    Opt,
    Group,
}

impl TryFrom<Ident> for ArgKind {
    type Error = syn::Error;

    fn try_from(ident: Ident) -> Result<Self, Self::Error> {
        match &*ident.to_string() {
            "pos_arg" => Ok(Self::PosArg),

            "arg_opt" => Ok(Self::ArgOpt),

            "opt" => Ok(Self::Opt),

            "group" => Ok(Self::Group),

            _ => Err(syn::Error::new_spanned(
                ident,
                "expected `pos_arg`, `arg_opt`, `opt`, or `group`",
            )),
        }
    }
}

impl ToString for ArgKind {
    fn to_string(&self) -> String {
        match self {
            Self::PosArg => "Positional arguments",

            Self::ArgOpt => "Options with argument",

            Self::Opt => "Options without argument",

            Self::Group => "Groups",
        }
        .to_owned()
    }
}
