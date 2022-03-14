mod cli_def;
mod cli_defs;

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use self::{cli_def::CliDef, cli_defs::CliDefs};

fn codegen_from_cli_def(cli_def: &CliDef) -> TokenStream {
    let filename = cli_def.cli_ty.to_string().to_case(Case::Snake);

    let sharp = syn::Token![#]([proc_macro2::Span::call_site()]);

    let members = cli_def
        .members
        .iter()
        .map(|member| {
            let member_name = member.into_token_stream().to_string();
            quote! {
                format!(
                    "{kind: >6} │ {name}",
                    kind = format!("{:?}", <#member as cli_compose::codegen::AsMember<_>>::kind()),
                    name = #member_name
                ),
            }
        })
        .collect::<TokenStream>();

    let members = quote! {
        vec![#members]
            .into_iter()
            .map(|member| quote::quote!{ #sharp member , })
            .collect::<proc_macro2::TokenStream>()
    };

    let cli_ty = &cli_def.cli_ty;
    let res_ty = &cli_def.res_ty;

    // evaluated at runtime of build.rs
    quote! {
        let mut dest = dest_dir.join(#filename);
        dest.set_extension("rs");

        let members = #members;
        let members = quote::quote! { &[#sharp members] };

        std::fs::write(
            &dest,
            quote::quote! {
                #[allow(dead_code)]
                struct #res_ty;

                #[allow(dead_code)]
                impl cli_compose::runtime::AsCli<#res_ty> for #cli_ty {
                    fn parse(args: impl Iterator<Item = String>) -> #res_ty {
                        let tokens = cli_compose::runtime::parse_into_tokens(args).collect::<Vec<_>>();
                        println!("tokens: {:?}", tokens);
                        println!("───────┬───────────────────────────────");
                        for member in #sharp members {
                            println!("{}", member);
                        }
                        todo!()
                    }
                }
            }.to_string()
        ).unwrap();
    }
}

pub fn define_cli(input: TokenStream) -> syn::Result<TokenStream> {
    let defs: CliDefs = syn::parse2(input)?;

    let codegen = defs
        .into_iter()
        .map(codegen_from_cli_def)
        .collect::<TokenStream>();

    Ok(quote! {
        let out_dir = std::env::var("OUT_DIR").expect("$OUT_DIR is not set");

        let mut dest_dir = std::path::Path::new(&out_dir).join("cli_compose");

        std::fs::create_dir_all(&dest_dir).expect("Failed to create cli_compose directory");

        #codegen
    })
}
