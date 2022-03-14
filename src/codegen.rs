pub use cli_compose_macro::define_cli;

use crate::schema::{AsArgOpt, AsMultiSelect, AsOpt, AsPosArg, AsSingleSelect};

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

impl<T: AsPosArg> AsMember<PosArgTag> for T {
    fn kind() -> MemberKind {
        MemberKind::PosArg
    }
}

pub struct ArgOptTag;

impl<T: AsArgOpt> AsMember<ArgOptTag> for T {
    fn kind() -> MemberKind {
        MemberKind::ArgOpt
    }
}

pub struct OptTag;

impl<T: AsOpt> AsMember<OptTag> for T {
    fn kind() -> MemberKind {
        MemberKind::Opt
    }
}

pub struct SingleSelectTag;

impl<T: AsSingleSelect> AsMember<SingleSelectTag> for T {
    fn kind() -> MemberKind {
        MemberKind::SingleSelect
    }
}

pub struct MultiSelectTag;

impl<T: AsMultiSelect> AsMember<MultiSelectTag> for T {
    fn kind() -> MemberKind {
        MemberKind::MultiSelect
    }
}

pub struct CliBuilder<T> {
    cli: std::marker::PhantomData<T>,
}

impl<T> CliBuilder<T> {
    pub fn pos_arg<M: AsPosArg>(self) -> Self {
        self
    }

    pub fn arg_opt<M: AsArgOpt>(self) -> Self {
        self
    }

    pub fn opt<M: AsOpt>(self) -> Self {
        self
    }

    pub fn build(self, _result_type_name: &str) {}
}

// TODO: 関数として define_cli を実装する
pub fn define_cli2<T>() -> CliBuilder<T> {
    CliBuilder {
        cli: std::marker::PhantomData,
    }
}
