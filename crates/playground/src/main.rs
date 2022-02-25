use cli_rs::{Arg, ArgGroup, Flag, FlagArg};

#[derive(Debug, Arg)]
/// ソースファイルのパス
struct Input(String);

#[derive(Debug, Flag)]
/// ソースコードを標準入力から読み込む
struct StdinFlag;

#[allow(dead_code)]
#[derive(Debug, ArgGroup)]
enum InputGroup {
    File(Input),
    Stdin(StdinFlag),
}

#[derive(Debug, FlagArg)]
/// 出力するファイルのパス
struct Output(String);

#[derive(Debug, Flag)]
/// 標準出力に出力する
struct StdoutFlag;

#[allow(dead_code)]
#[derive(Debug, ArgGroup)]
enum OutputGroup {
    File(Output),
    Stdout(StdoutFlag),
}

#[derive(Flag)]
struct Verbose;

fn main() {
    cli_rs::parse!(Input, Verbose);
}
