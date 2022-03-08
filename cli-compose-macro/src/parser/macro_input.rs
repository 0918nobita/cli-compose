use syn::{parse, Ident};

use super::schema::Schema;

pub struct ParserMacroInput {
    pub ty_name: Ident,
    pub schemas: Vec<Schema>,
}

impl parse::Parse for ParserMacroInput {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let ty_name = input.call(Ident::parse)?;

        input.parse::<syn::Token![,]>()?;

        let mut schemas = Vec::<Schema>::new();
        while !input.is_empty() {
            let schema = input.parse::<Schema>()?;
            schemas.push(schema);
        }

        Ok(Self { ty_name, schemas })
    }
}
