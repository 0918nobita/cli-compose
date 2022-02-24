pub use cli_rs_macro::{parse, Arg, ArgGroup, Flag, FlagArg};

pub trait ToArg: Sized {
    fn name() -> String;
    fn description() -> String;
    fn parse(str: &str) -> Option<Self>;
}
