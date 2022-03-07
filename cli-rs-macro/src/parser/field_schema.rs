use syn::{parse, Ident, TypePath};

pub struct FieldSchema {
    pub ident: Ident,
    pub ty: TypePath,
}

impl parse::Parse for FieldSchema {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let ty = input.parse::<TypePath>()?;

        input.parse::<syn::Token![:]>()?;

        let ident = input.parse::<Ident>()?;

        Ok(FieldSchema { ident, ty })
    }
}
