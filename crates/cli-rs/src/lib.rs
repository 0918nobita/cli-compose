pub use cli_rs_macro::Arg;

pub trait ToArg {
    fn name() -> String;
    fn description() -> String;
}
