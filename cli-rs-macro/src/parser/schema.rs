use std::fmt;

use syn::{
    parse::{Parse, ParseStream},
    Ident, Token,
};

use super::{field_schema::FieldSchema, modifiers::Modifiers};
use crate::parser::{arg_kind::ArgKind, modifier::Modifier};

pub struct Schema {
    pub kind: ArgKind,
    pub modifiers: Modifiers,
    pub data: SchemaData,
}

impl fmt::Debug for Schema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Schema({:?}, {:?}, {:?})",
            self.kind, self.modifiers, self.data
        )
    }
}

#[derive(Debug)]
pub enum SchemaData {
    Single(FieldSchema),
    Multiple(Vec<FieldSchema>),
}

impl Parse for Schema {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;

        let kind = ArgKind::try_from(ident)?;

        let modifiers = if input.peek(syn::token::Paren) {
            let parenthesized;
            syn::parenthesized!(parenthesized in input);

            let modifiers = parenthesized
                .parse_terminated::<Modifier, Token![,]>(Modifier::parse)?
                .into_iter()
                .collect::<Vec<_>>();

            match Modifiers::try_from_slice(modifiers.as_slice()) {
                Ok(modifiers) => modifiers,
                Err(modifier) => {
                    return Err(syn::Error::new_spanned(modifier.name, "Duplicate modifier"))
                }
            }
        } else {
            Modifiers::default()
        };

        let data = if input.peek(syn::token::Brace) {
            let braced;
            syn::braced!(braced in input);

            let field_schemas = braced
                .parse_terminated::<FieldSchema, Token![,]>(FieldSchema::parse)?
                .into_iter()
                .collect::<Vec<_>>();

            SchemaData::Multiple(field_schemas)
        } else {
            let field_schema = input.parse::<FieldSchema>()?;
            SchemaData::Single(field_schema)
        };

        Ok(Self {
            kind,
            modifiers,
            data,
        })
    }
}
