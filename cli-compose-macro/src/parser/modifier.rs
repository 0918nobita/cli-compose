use syn::{parse, Ident};

#[derive(Clone)]
pub struct Modifier {
    pub name: Ident,
    pub value: Ident,
}

impl parse::Parse for Modifier {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;

        input.parse::<syn::Token![=]>()?;

        let value = input.parse::<Ident>()?;

        Ok(Self { name, value })
    }
}
