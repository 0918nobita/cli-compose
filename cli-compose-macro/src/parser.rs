mod arg_kind;
mod field_schema;
mod macro_input;
mod modifier;
mod modifiers;
mod schema;

use proc_macro2::TokenStream;
use quote::quote;

use self::{arg_kind::ArgKind, macro_input::ParserMacroInput};

pub fn parser(input: TokenStream) -> syn::Result<TokenStream> {
    let ParserMacroInput { ty_name, schemas } = syn::parse2::<ParserMacroInput>(input)?;

    let mut field_binds = std::collections::HashMap::<String, ArgKind>::new();

    for schema in schemas {
        for field in schema.field_schemas {
            if field_binds
                .insert(field.ident.to_string(), schema.kind.clone())
                .is_some()
            {
                return Err(syn::Error::new_spanned(field.ident, "Duplicate field name"));
            }
        }
    }

    Ok(quote! {
        struct #ty_name {
        }

        impl #ty_name {
            fn parse(args: impl Iterator<Item = String>) -> Self {
                let tokens = cli_compose::parse_into_tokens(args).collect::<Vec<_>>();

                if tokens.iter().any(|token| *token == cli_compose::Token::Long("help".to_owned())) {
                    let name = env!("CARGO_PKG_NAME");
                    let version = env!("CARGO_PKG_VERSION");
                    let description = env!("CARGO_PKG_DESCRIPTION");
                    println!("{} {}\n{}", name, version, description);
                    std::process::exit(0);
                }

                todo!()
            }
        }
    })
}
