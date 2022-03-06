//! CLI 開発支援ライブラリ

use derive_more::Display;

pub use cli_rs_macro::{parser, ArgOpt, FromKebabStr, Group, Opt, PosArg};

#[derive(Display)]
pub enum Flag {
    #[display(fmt = "--{}", _0)]
    LongOnly(String),

    #[display(fmt = "-{}", _0)]
    ShortOnly(char),

    #[display(fmt = "--{}, -{}", _0, _1)]
    BothLongAndShort(String, char),
}

pub trait AsPosArg: Sized {
    fn name() -> String;

    fn description() -> String;
}

pub trait AsArgOpt: Sized {
    fn flag() -> Flag;

    fn description() -> String;

    fn parse(s: &str) -> Option<Self>;
}

pub trait AsOpt: Sized {
    fn flag() -> Flag;

    fn description() -> String;
}

pub trait AsGroup: Sized {
    fn name() -> String;

    fn description() -> String;
}

#[derive(Debug)]
pub enum Token {
    Long(String),
    Short(char),
    Value(String),
}

pub fn parse_into_tokens<A>(args: A) -> impl Iterator<Item = Token>
where
    A: Iterator<Item = String>,
{
    args.skip(1).flat_map(|arg| {
        if let Some(flag) = arg.strip_prefix("--") {
            return vec![Token::Long(flag.to_owned())];
        }
        if let Some(cs) = arg.strip_prefix('-') {
            return cs.chars().map(Token::Short).collect::<Vec<_>>();
        }
        vec![Token::Value(arg)]
    })
}
