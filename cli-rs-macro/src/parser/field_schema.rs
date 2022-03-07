use convert_case::{Case, Casing};
use syn::{parse, Ident, Token, TypePath};

pub struct FieldSchema {
    pub ident: Ident,
    pub ty: TypePath,
}

impl parse::Parse for FieldSchema {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let ty = input.parse::<TypePath>()?;

        let ident = if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            input.parse::<Ident>()?
        } else {
            Ident::new(
                &ty.path
                    .segments
                    .last()
                    .unwrap()
                    .ident
                    .to_string()
                    .to_case(Case::Snake),
                proc_macro2::Span::call_site(),
            )
        };

        Ok(FieldSchema { ident, ty })
    }
}
