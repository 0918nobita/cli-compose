mod arg;
mod attr_meta;
mod doc;
mod flag;
mod flag_arg;
mod group;
mod kebab_case;
mod parse;

use proc_macro::TokenStream;

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

#[proc_macro_derive(FlagArg, attributes(flag_arg))]
/// 値を要求するフラグ
pub fn derive_flag_arg(input: TokenStream) -> TokenStream {
    flag_arg::derive_flag_arg(input)
}

#[proc_macro_derive(Group)]
/// 引数グループ
pub fn derive_group(input: TokenStream) -> TokenStream {
    group::derive_group(input)
}

#[proc_macro]
/// コマンドライン引数をパースする
pub fn parse(input: TokenStream) -> TokenStream {
    parse::parse(input)
}
