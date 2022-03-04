//! CLI 開発支援ライブラリ

use derive_more::Display;

pub use cli_rs_macro::{parse, ArgOpt, FromKebabStr, Group, Opt, PosArg};

#[derive(Display)]
#[display(fmt = "--{}", _0)]
pub struct LongFlag(String);

impl LongFlag {
    pub fn new(s: &str) -> Self {
        Self(s.to_owned())
    }
}

#[derive(Display)]
#[display(fmt = "-{}", _0)]
pub struct ShortFlag(char);

impl ShortFlag {
    pub fn new(c: char) -> Self {
        Self(c)
    }
}

pub trait AsPosArg: Sized {
    fn name() -> String;

    fn description() -> String;
}

pub trait AsArgOpt: Sized {
    fn long() -> LongFlag;

    fn short() -> Option<ShortFlag>;

    fn description() -> String;

    fn parse(s: &str) -> Option<Self>;
}

pub trait AsOpt: Sized {
    fn long() -> LongFlag;

    fn short() -> Option<ShortFlag>;

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

pub fn parse_into_tokens(args: impl Iterator<Item = String>) -> impl Iterator<Item = Token> {
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
