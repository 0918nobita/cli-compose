pub use cli_rs_macro::Arg;

pub trait ToArg: Sized {
    fn name() -> String;
    fn description() -> String;
    fn parse(str: &str) -> Option<Self>;
}
