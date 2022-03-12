use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse, Ident};

struct CliDef {
    cli_ty: Ident,
    res_ty: Ident,
}

impl parse::Parse for CliDef {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let cli_ty = input.parse()?;

        input.parse::<syn::Token![->]>()?;

        let res_ty = input.parse()?;

        Ok(Self { cli_ty, res_ty })
    }
}

struct CliDefs(Vec<CliDef>);

impl<'a> std::iter::IntoIterator for &'a CliDefs {
    type Item = &'a CliDef;
    type IntoIter = std::slice::Iter<'a, CliDef>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl parse::Parse for CliDefs {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let defs = input
            .parse_terminated::<CliDef, syn::Token![,]>(CliDef::parse)?
            .into_iter()
            .collect::<Vec<_>>();
        Ok(Self(defs))
    }
}

pub fn define_cli(input: TokenStream) -> syn::Result<TokenStream> {
    let defs: CliDefs = syn::parse2(input)?;

    let contents = defs.into_iter()
        .map(|CliDef { cli_ty, res_ty }| {
            quote! {
                #[allow(dead_code)]
                struct #res_ty {
                    input: String,
                    output: Option<String>,
                    stdin: Option<playground_opts::StdinOpt>,
                    stdout: Option<playground_opts::StdoutOpt>,
                    verbose: Option<playground_opts::Verbose>,
                }

                #[allow(dead_code)]
                impl cli_compose::runtime::AsCli<#res_ty> for #cli_ty {
                    fn parse(args: impl Iterator<Item = String>) -> #res_ty {
                        let tokens = cli_compose::runtime::parse_into_tokens(args).collect::<Vec<_>>();
                        println!("{:?}", tokens);
                        todo!()
                    }
                }
            }
        })
        .collect::<TokenStream>()
        .to_string();

    Ok(quote! {
        let out_dir = std::env::var("OUT_DIR").expect("$OUT_DIR is not set");

        let mut dest = std::path::PathBuf::from(&out_dir).join("cli_compose");

        std::fs::create_dir_all(&dest).expect("Failed to create cli_compose directory");

        dest.push("cli.rs");

        std::fs::write(&dest, #contents).unwrap_or_else(|err| {
            eprintln!("{}", err);
            panic!("Failed to write source file ({:?})", &dest);
        });
    })
}
