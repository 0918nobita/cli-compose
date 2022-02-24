pub use cli_rs_macro::{Arg, ArgGroup, Flag, FlagArg};

pub trait ToArg: Sized {
    fn name() -> String;
    fn description() -> String;
    fn parse(str: &str) -> Option<Self>;
}
