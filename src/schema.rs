use derive_more::Display;

pub use cli_compose_macro::{ArgOpt, FromKebabStr, Group, Opt, PosArg};

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

    fn parse(s: &str) -> Option<Self>;
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
