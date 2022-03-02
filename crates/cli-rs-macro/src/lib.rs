mod arg;
mod attr_meta;
mod doc;
mod flag;
mod flag_arg;
mod group;
mod kebab_case;
mod parse;

use proc_macro::TokenStream;

#[proc_macro_derive(FromKebabStr)]
pub fn derive_from_kebab_str(_: TokenStream) -> TokenStream {
    quote::quote! {}.into()
}

/// コマンドライン引数
#[proc_macro_derive(Arg, attributes(arg))]
pub fn derive_arg(input: TokenStream) -> TokenStream {
    arg::derive_arg(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// 値を持たないフラグ
#[proc_macro_derive(Flag, attributes(flag))]
pub fn derive_flag(input: TokenStream) -> TokenStream {
    flag::derive_flag(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// 値を要求するフラグ
#[proc_macro_derive(FlagArg, attributes(flag_arg))]
pub fn derive_flag_arg(input: TokenStream) -> TokenStream {
    flag_arg::derive_flag_arg(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// 引数グループ
#[proc_macro_derive(Group)]
pub fn derive_group(input: TokenStream) -> TokenStream {
    group::derive_group(input.into()).into()
}

/// コマンドライン引数をパースする
#[proc_macro]
pub fn parse(input: TokenStream) -> TokenStream {
    parse::parse(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
