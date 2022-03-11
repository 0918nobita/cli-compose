use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse, Ident};

struct CliDef {
    ident: Ident,
}

impl parse::Parse for CliDef {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        Ok(Self { ident })
    }
}

pub fn define_cli(input: TokenStream) -> syn::Result<TokenStream> {
    let CliDef { ident } = syn::parse2(input)?;

    let contents = quote::quote! {
        #[allow(dead_code)]
        struct #ident {
            input: String,
            output: Option<String>,
            stdin: Option<playground_opts::StdinOpt>,
            stdout: Option<playground_opts::StdoutOpt>,
            verbose: Option<playground_opts::Verbose>,
        }

        #[allow(dead_code)]
        impl #ident {
            pub fn parse(args: impl Iterator<Item = String>) {
                let tokens = cli_compose::runtime::parse_into_tokens(args).collect::<Vec<_>>();
                println!("{:?}", tokens);
            }
        }
    }
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
