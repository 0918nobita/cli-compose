pub use cli_rs_macro::{parse, Arg, ArgGroup, Flag, FlagArg};

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
