use cli_compose::{ArgOpt, FromKebabStr, Opt};

#[derive(Debug, ArgOpt)]
pub struct Input(String);

#[derive(Debug, Opt)]
#[opt(long = "stdin")]
pub struct StdinOpt;

#[derive(Debug, ArgOpt, FromKebabStr)]
#[arg_opt(use_default = true)]
pub enum InputFormat {
    Json,
    Yaml,
}

#[derive(Debug, ArgOpt)]
#[arg_opt(short = 'o')]
pub struct Output(String);

#[derive(Debug, Opt)]
#[opt(long = "stdout")]
pub struct StdoutOpt;

#[derive(Debug, Opt)]
#[opt(short = 'V')]
pub struct Verbose;
