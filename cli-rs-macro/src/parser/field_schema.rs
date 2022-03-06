use syn::{parse, Ident, TypePath};

pub struct FieldSchema {
    pub ident: Ident,
    pub ty: TypePath,
}

impl parse::Parse for FieldSchema {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;

        input.parse::<syn::Token![:]>()?;

        let ty = input.parse::<TypePath>()?;

        Ok(FieldSchema { ident, ty })
    }
}
