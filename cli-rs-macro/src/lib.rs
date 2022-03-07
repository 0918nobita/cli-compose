mod deriver;
mod doc;
mod parser;

use proc_macro::TokenStream;

macro_rules! wrap_derive_fn {
    ($f:expr, $input:expr) => {
        $f($input.into())
            .unwrap_or_else(syn::Error::into_compile_error)
            .into()
    };
}

/// フィールドを持たないヴァリアントのみで構成される列挙型に対して、
/// ケバブケースの文字列から変換できるように [`std::str::FromStr`] を実装する
#[proc_macro_derive(FromKebabStr)]
pub fn derive_from_kebab_str(input: TokenStream) -> TokenStream {
    wrap_derive_fn!(deriver::derive_from_kebab_str, input)
}

/// 位置指定引数
#[proc_macro_derive(PosArg, attributes(pos_arg))]
pub fn derive_pos_arg(input: TokenStream) -> TokenStream {
    wrap_derive_fn!(deriver::derive_pos_arg, input)
}

/// 引数付きオプション
#[proc_macro_derive(ArgOpt, attributes(arg_opt))]
pub fn derive_arg_opt(input: TokenStream) -> TokenStream {
    wrap_derive_fn!(deriver::derive_arg_opt, input)
}

/// 引数なしオプション
#[proc_macro_derive(Opt, attributes(opt))]
pub fn derive_opt(input: TokenStream) -> TokenStream {
    wrap_derive_fn!(deriver::derive_opt, input)
}

/// グループ
#[proc_macro_derive(Group)]
pub fn derive_group(input: TokenStream) -> TokenStream {
    wrap_derive_fn!(deriver::derive_group, input)
}

/// コマンドライン引数をパースする
#[proc_macro]
pub fn parser(input: TokenStream) -> TokenStream {
    wrap_derive_fn!(parser::parser, input)
}
