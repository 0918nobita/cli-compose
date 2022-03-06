use syn::parse;

use super::field_schema::FieldSchema;
use crate::parser::arg_kind::ArgKind;

pub struct Schema {
    pub kind: ArgKind,
    pub field_schemas: Vec<FieldSchema>,
}

impl parse::Parse for Schema {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;

        let kind = ArgKind::try_from(ident)?;

        let content;
        syn::braced!(content in input);

        let field_schemas = content
            .parse_terminated::<FieldSchema, syn::Token![,]>(FieldSchema::parse)?
            .into_iter()
            .collect::<Vec<_>>();

        Ok(Self {
            kind,
            field_schemas,
        })
    }
}
