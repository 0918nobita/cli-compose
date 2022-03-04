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
#[arg_opt(default)]
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

fn main() {
    cli_rs::parse!(
        std::env::args(),
        pos_arg {
            input = Input,
        }
        arg_opt {
            input_format = InputFormat,
            output = Output,
        }
        opt {
            verbose = Verbose,
        }
    );
}
