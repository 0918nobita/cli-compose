use cli_rs::{Arg, ArgGroup, Flag, FlagArg};

#[derive(Debug, Arg)]
/// ソースファイルのパス
struct InputArg(String);

#[derive(Flag)]
/// ソースコードを標準入力から読み込む
struct StdinFlag(bool);

#[allow(dead_code)]
#[derive(ArgGroup)]
enum InputGroup {
    File(InputArg),
    Stdin(StdinFlag),
}

#[derive(FlagArg)]
/// 出力するファイルのパス
struct OutputFlag(String);

#[derive(Flag)]
/// 標準出力に出力する
struct StdoutFlag(bool);

#[allow(dead_code)]
#[derive(ArgGroup)]
enum OutputGroup {
    File(OutputFlag),
    Stdout(StdoutFlag),
}

fn main() {
    cli_rs::parse!(InputArg);
}
