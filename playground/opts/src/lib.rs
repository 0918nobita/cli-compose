use cli_compose::schema::{ArgOpt, Cli, FromKebabStr, Opt, PosArg};

/// 入力ファイルのパス
#[derive(Debug, PosArg)]
pub struct Input(String);

/// 標準入力から読み取る
#[derive(Debug, Opt)]
#[opt(long = "stdin")]
pub struct StdinOpt;

/// 入力ファイルの形式
#[derive(Debug, ArgOpt, FromKebabStr)]
#[arg_opt(use_default = true)]
pub enum InputFormat {
    Json,
    Yaml,
}

/// 出力ファイルのパス
#[derive(Debug, ArgOpt)]
#[arg_opt(short = 'o')]
pub struct Output(String);

/// 標準出力に出力する
#[derive(Debug, Opt)]
#[opt(long = "stdout")]
pub struct StdoutOpt;

/// 詳細を標準エラーに出力する
#[derive(Debug, Opt)]
#[opt(short = 'V')]
pub struct Verbose;

/// CLI ツールの説明文
#[derive(Cli)]
#[cli(name = "example", version = from_crate)]
pub struct Cli;
