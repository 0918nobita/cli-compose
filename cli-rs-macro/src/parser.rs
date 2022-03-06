use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Ident, Token, TypePath,
};

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
    binds: Vec<TypePath>,
}

impl Parse for Schema {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;

        let kind = ArgKind::try_from(ident)?;

        let content;
        syn::braced!(content in input);

        let binds = content
            .parse_terminated::<TypePath, Token![,]>(TypePath::parse)?
            .iter()
            .cloned()
            .collect::<Vec<_>>();

        Ok(Self { kind, binds })
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
                todo!()
            }
        }
    })
}

#[allow(dead_code)]
pub fn parser_old(input: TokenStream) -> syn::Result<TokenStream> {
    let ParserMacroInput { schemas, .. } = syn::parse2::<ParserMacroInput>(input)?;

    let mut dump_code = TokenStream::new();

    for Schema { kind, binds } in schemas {
        let kind_str = kind.to_string();
        dump_code.extend(quote! { println!("[{}]", #kind_str); });

        for path in binds {
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
