use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Ident, Token, TypePath,
};

struct FieldSchema {
    #[allow(dead_code)]
    ident: Ident,
    ty: TypePath,
}

impl Parse for FieldSchema {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;

        input.parse::<Token![:]>()?;

        let ty = input.parse::<TypePath>()?;

        Ok(FieldSchema { ident, ty })
    }
}

enum ArgKind {
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

struct Schema {
    kind: ArgKind,
    field_schemas: Vec<FieldSchema>,
}

impl Parse for Schema {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;

        let kind = ArgKind::try_from(ident)?;

        let content;
        syn::braced!(content in input);

        let field_schemas = content
            .parse_terminated::<FieldSchema, Token![,]>(FieldSchema::parse)?
            .into_iter()
            .collect::<Vec<_>>();

        Ok(Self {
            kind,
            field_schemas,
        })
    }
}

struct ParserMacroInput {
    ty_name: Ident,
    schemas: Vec<Schema>,
}

impl Parse for ParserMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty_name = input.call(Ident::parse)?;

        input.parse::<Token![,]>()?;

        let mut schemas = Vec::<Schema>::new();
        while !input.is_empty() {
            let schema = input.parse::<Schema>()?;
            schemas.push(schema);
        }

        Ok(Self { ty_name, schemas })
    }
}

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
