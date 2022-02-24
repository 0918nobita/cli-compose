mod arg;
mod arg_group;
mod flag;
mod flag_arg;

use std::fmt;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Path, Token};

#[proc_macro_derive(Arg)]
/// コマンドライン引数
pub fn derive_arg(input: TokenStream) -> TokenStream {
    arg::derive_arg(input)
}

#[proc_macro_derive(Flag)]
/// 値を持たないフラグ
pub fn derive_flag(input: TokenStream) -> TokenStream {
    flag::derive_flag(input)
}

#[proc_macro_derive(FlagArg)]
/// 値を要求するフラグ
pub fn derive_flag_arg(input: TokenStream) -> TokenStream {
    flag_arg::derive_flag_arg(input)
}

#[proc_macro_derive(ArgGroup)]
/// 引数グループ
pub fn derive_arg_group(input: TokenStream) -> TokenStream {
    arg_group::derive_arg_group(input)
}

struct ArgTypes(Vec<Path>);

impl fmt::Debug for ArgTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let types = &self
            .0
            .iter()
            .map(|path| format!("{}", path.to_token_stream()))
            .collect::<Vec<_>>();
        write!(f, "ArgTypes {:?}", types)
    }
}

impl syn::parse::Parse for ArgTypes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let types = input
            .parse_terminated::<Path, Token![,]>(Path::parse)?
            .into_iter()
            .collect::<Vec<_>>();
        Ok(ArgTypes(types))
    }
}

#[proc_macro]
/// コマンドライン引数をパースする
pub fn parse(input: TokenStream) -> TokenStream {
    let ArgTypes(types) = syn::parse_macro_input!(input as ArgTypes);

    let types = types
        .iter()
        .map(|path| {
            quote! {
                println!(
                    "{}: {}",
                    <#path as cli_rs::ToArg>::name(),
                    <#path as cli_rs::ToArg>::description()
                );
            }
        })
        .collect::<proc_macro2::TokenStream>();

    quote! {
        #types
        println!("args: {:?}", std::env::args().collect::<Vec<_>>());
    }
    .into()
}
