mod arg;
mod attr_meta;
mod doc;
mod flag;
mod flag_arg;
mod from_kebab_str;
mod group;
mod kebab_case;
mod parse;

use proc_macro::TokenStream;

macro_rules! wrap_derive_fn {
    ($f:expr, $input:expr) => {
        $f($input.into())
            .unwrap_or_else(syn::Error::into_compile_error)
            .into()
    };
}

#[proc_macro_derive(FromKebabStr)]
pub fn derive_from_kebab_str(input: TokenStream) -> TokenStream {
    wrap_derive_fn!(from_kebab_str::derive_from_kebab_str, input)
}

/// コマンドライン引数
#[proc_macro_derive(Arg, attributes(arg))]
pub fn derive_arg(input: TokenStream) -> TokenStream {
    wrap_derive_fn!(arg::derive_arg, input)
}

/// 値を持たないフラグ
#[proc_macro_derive(Flag, attributes(flag))]
pub fn derive_flag(input: TokenStream) -> TokenStream {
    wrap_derive_fn!(flag::derive_flag, input)
}

/// 値を要求するフラグ
#[proc_macro_derive(FlagArg, attributes(flag_arg))]
pub fn derive_flag_arg(input: TokenStream) -> TokenStream {
    wrap_derive_fn!(flag_arg::derive_flag_arg, input)
}

/// 引数グループ
#[proc_macro_derive(Group)]
pub fn derive_group(input: TokenStream) -> TokenStream {
    wrap_derive_fn!(group::derive_group, input)
}

/// コマンドライン引数をパースする
#[proc_macro]
pub fn parse(input: TokenStream) -> TokenStream {
    wrap_derive_fn!(parse::parse, input)
}
