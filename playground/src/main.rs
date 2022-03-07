use cli_rs::{ArgOpt, FromKebabStr, Group, Opt, PosArg};

/// ソースファイルのパス
#[derive(Debug, PosArg)]
struct Input(String);

/// ソースコードを標準入力から読み込む
#[derive(Debug, Opt)]
#[opt(long = "stdin")]
struct StdinOpt;

#[allow(dead_code)]
#[derive(Debug, Group)]
enum InputGroup {
    File(Input),
    Stdin(StdinOpt),
}

/// ソースファイルの形式
#[derive(Debug, ArgOpt, FromKebabStr)]
enum InputFormat {
    Json,

    #[allow(dead_code)]
    Yaml,
}

impl Default for InputFormat {
    fn default() -> Self {
        InputFormat::Json
    }
}

/// 出力するファイルのパス
#[derive(Debug, ArgOpt)]
#[arg_opt(short = 'o')]
struct Output(String);

/// 標準出力に出力する
#[derive(Debug, Opt)]
#[opt(long = "stdout")]
struct StdoutOpt;

#[allow(dead_code)]
#[derive(Debug, Group)]
enum OutputGroup {
    File(Output),
    Stdout(StdoutOpt),
}

#[derive(Opt)]
struct Verbose;

cli_rs::parser!(
    Cli,

    pos_arg Input: input

    arg_opt(use_default = yes) InputFormat: input_format

    arg_opt Output: output

    opt Verbose: verbose
);

fn main() {
    let _cli = Cli::parse(std::env::args());
}
