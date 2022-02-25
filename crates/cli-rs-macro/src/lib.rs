mod arg;
mod arg_group;
mod attr_meta;
mod doc;
mod flag;
mod flag_arg;
mod kebab_case;

use std::fmt;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Expr, Path, Token};

#[proc_macro_derive(Arg, attributes(arg))]
/// コマンドライン引数
pub fn derive_arg(input: TokenStream) -> TokenStream {
    arg::derive_arg(input)
}

#[proc_macro_derive(Flag, attributes(flag))]
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

struct ArgTypes {
    args: Expr,
    arg_types: Vec<Path>,
}

impl fmt::Debug for ArgTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let arg_type_names = &self
            .arg_types
            .iter()
            .map(|path| format!("{}", path.to_token_stream()))
            .collect::<Vec<_>>();
        write!(
            f,
            "ArgTypes(`{}`, {:?})",
            &self.args.to_token_stream(),
            arg_type_names
        )
    }
}

impl syn::parse::Parse for ArgTypes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let args = input.call(Expr::parse_without_eager_brace)?;
        input.parse::<Token![,]>()?;
        let arg_types = input
            .parse_terminated::<Path, syn::Token![,]>(Path::parse)?
            .into_iter()
            .collect::<Vec<_>>();
        Ok(ArgTypes { args, arg_types })
    }
}

#[proc_macro]
/// コマンドライン引数をパースする
pub fn parse(input: TokenStream) -> TokenStream {
    let ArgTypes { args, arg_types } = syn::parse_macro_input!(input as ArgTypes);

    let arg_types = arg_types
        .iter()
        .map(|path| {
            quote! {
                println!("{:?}", <#path as cli_rs::ToArgMeta>::metadata());
            }
        })
        .collect::<proc_macro2::TokenStream>();

    quote! {
        #arg_types
        println!("args: {:?}", #args);
    }
    .into()
}
