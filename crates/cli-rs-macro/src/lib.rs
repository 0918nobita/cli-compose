mod arg;
mod arg_group;
mod flag;
mod flag_arg;

use proc_macro::TokenStream;

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
