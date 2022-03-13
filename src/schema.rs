use derive_more::Display;

pub use cli_compose_macro::{ArgOpt, Cli, FromKebabStr, MultiSelect, Opt, PosArg, SingleSelect};

#[derive(Display)]
pub enum Flag {
    #[display(fmt = "--{}", _0)]
    LongOnly(String),

    #[display(fmt = "-{}", _0)]
    ShortOnly(char),

    #[display(fmt = "--{}, -{}", _0, _1)]
    BothLongAndShort(String, char),
}

#[derive(Debug)]
pub enum MemberKind {
    PosArg,
    ArgOpt,
    Opt,
    SingleSelect,
    MultiSelect,
}

pub trait AsMember<Tag> {
    fn kind() -> MemberKind;
}

pub struct PosArgTag;

pub trait AsPosArg: Sized {
    fn name() -> String;

    fn description() -> String;

    fn parse(s: &str) -> Option<Self>;
}

impl<T: AsPosArg> AsMember<PosArgTag> for T {
    fn kind() -> MemberKind {
        MemberKind::PosArg
    }
}

pub struct ArgOptTag;

pub trait AsArgOpt: Sized {
    fn flag() -> Flag;

    fn description() -> String;

    fn parse(s: &str) -> Option<Self>;
}

impl<T: AsArgOpt> AsMember<ArgOptTag> for T {
    fn kind() -> MemberKind {
        MemberKind::ArgOpt
    }
}

pub struct OptTag;

pub trait AsOpt {
    fn flag() -> Flag;

    fn description() -> String;
}

impl<T: AsOpt> AsMember<OptTag> for T {
    fn kind() -> MemberKind {
        MemberKind::Opt
    }
}

pub struct SingleSelect;

pub trait AsSingleSelect {
    fn name() -> String;

    fn description() -> String;
}

impl<T: AsSingleSelect> AsMember<SingleSelect> for T {
    fn kind() -> MemberKind {
        MemberKind::SingleSelect
    }
}

pub struct MultiSelect;

pub trait AsMultiSelect {
    fn name() -> String;

    fn description() -> String;
}

impl<T: AsMultiSelect> AsMember<MultiSelect> for T {
    fn kind() -> MemberKind {
        MemberKind::MultiSelect
    }
}
