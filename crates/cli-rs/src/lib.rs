pub use cli_rs_macro::{parse, Arg, Flag, FlagArg, Group};

#[derive(Debug)]
pub enum ArgMeta {
    Flag {
        long: String,
        short: Option<char>,
        description: String,
    },
    FlagArg {
        long: String,
        short: Option<char>,
        description: String,
    },
    Arg {
        name: String,
        description: String,
    },
    ArgGroup(Vec<ArgMeta>),
}

pub trait ToArgMeta {
    fn metadata() -> ArgMeta;
}

pub trait AsFlagArg: Sized {
    fn parse(s: &str) -> Option<Self>;
}
