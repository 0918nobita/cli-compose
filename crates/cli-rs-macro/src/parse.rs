use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Expr, Pat, Path, Token,
};

struct ArgBind {
    #[allow(dead_code)]
    pat: Pat,
    path: Path,
}

impl Parse for ArgBind {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let pat = input.parse::<Pat>()?;

        input.parse::<Token![=]>()?;

        let path = input.parse::<Path>()?;

        Ok(Self { pat, path })
    }
}

struct ArgTypes {
    args: Expr,
    arg_binds: Vec<ArgBind>,
}

impl Parse for ArgTypes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args = input.call(Expr::parse_without_eager_brace)?;

        input.parse::<Token![,]>()?;

        let arg_binds = input
            .parse_terminated::<ArgBind, syn::Token![,]>(ArgBind::parse)?
            .into_iter()
            .collect::<Vec<_>>();

        Ok(Self { args, arg_binds })
    }
}

// TODO: ArgMeta ベクタをもとにして、トークン列をパースする
pub fn parse(input: TokenStream) -> TokenStream {
    let ArgTypes { args, arg_binds } = syn::parse_macro_input!(input as ArgTypes);

    let arg_meta = arg_binds
        .iter()
        .map(|ArgBind { path, .. }| {
            quote! { <#path as cli_rs::ToArgMetadatum>::metadatum(), }
        })
        .collect::<proc_macro2::TokenStream>();
    let arg_meta = quote! { vec![#arg_meta] };

    quote! {
        {
            let arg_meta = #arg_meta;
            println!("arg_meta:");
            for item in arg_meta.iter() {
                println!("    {:?}", item);
            }

            let tokens = cli_rs::parse_into_tokens(#args).collect::<Vec<_>>();
            println!("tokens: {:?}", tokens);
        }
    }
    .into()
}
