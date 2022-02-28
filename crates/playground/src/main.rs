use cli_rs::{Arg, Flag, FlagArg, Group};

#[derive(Debug, Arg)]
/// ソースファイルのパス
struct Input(String);

#[derive(Debug, Flag)]
#[flag(long = "stdin")]
/// ソースコードを標準入力から読み込む
struct StdinFlag;

#[allow(dead_code)]
#[derive(Debug, Group)]
enum InputGroup {
    File(Input),
    Stdin(StdinFlag),
}

#[derive(Debug, FlagArg)]
#[flag_arg(short = 'o')]
/// 出力するファイルのパス
struct Output(String);

#[derive(Debug, Flag)]
#[flag(long = "stdout")]
/// 標準出力に出力する
struct StdoutFlag;

#[allow(dead_code)]
#[derive(Debug, Group)]
enum OutputGroup {
    File(Output),
    Stdout(StdoutFlag),
}

#[derive(Flag)]
struct Verbose;

fn main() {
    cli_rs::parse!(
        std::env::args(),
        group { input = InputGroup, output = OutputGroup }
        flag { verbose = Verbose }
    );
}
