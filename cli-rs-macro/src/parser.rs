mod arg_kind;
mod field_schema;
mod macro_input;
mod schema;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use self::{
    arg_kind::ArgKind, field_schema::FieldSchema, macro_input::ParserMacroInput, schema::Schema,
};

pub fn parser(input: TokenStream) -> syn::Result<TokenStream> {
    let ParserMacroInput {
        ty_name,
        schemas: _schemas,
    } = syn::parse2::<ParserMacroInput>(input)?;

    Ok(quote! {
        struct #ty_name {
        }

        impl #ty_name {
            fn parse(args: impl Iterator<Item = String>) -> Self {
                let tokens = cli_rs::parse_into_tokens(args).collect::<Vec<_>>();

                if tokens.iter().any(|token| *token == cli_rs::Token::Long("help".to_owned())) {
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

#[allow(dead_code)]
pub fn parser_old(input: TokenStream) -> syn::Result<TokenStream> {
    let ParserMacroInput { schemas, .. } = syn::parse2::<ParserMacroInput>(input)?;

    let mut dump_code = TokenStream::new();

    for Schema {
        kind,
        field_schemas,
    } in schemas
    {
        let kind_str = kind.to_string();
        dump_code.extend(quote! { println!("[{}]", #kind_str); });

        for FieldSchema { ty: path, .. } in field_schemas {
            let path_str = path.to_token_stream().to_string();

            dump_code.extend(match kind {
                ArgKind::PosArg => quote! {
                    println!(
                        "    {}: {}",
                        <#path as cli_rs::AsPosArg>::name(),
                        <#path as cli_rs::AsPosArg>::description()
                    );
                },

                ArgKind::ArgOpt => quote! {
                    println!(
                        "    {}: {}",
                        <#path as cli_rs::AsArgOpt>::flag(),
                        <#path as cli_rs::AsArgOpt>::description()
                    );
                },

                ArgKind::Opt => quote! {
                    println!(
                        "    {}: {}",
                        <#path as cli_rs::AsOpt>::flag(),
                        <#path as cli_rs::AsOpt>::description()
                    );
                },

                ArgKind::Group => quote! {
                    println!("    {}", #path_str);
                },
            });

            dump_code.extend(quote! { println!(); });
        }
    }

    Ok(dump_code)
}
