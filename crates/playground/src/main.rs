use cli_rs::{Arg, ArgGroup, Flag, FlagArg};

#[derive(Debug, Arg)]
/// ソースファイルのパス
struct InputArg(String);

#[derive(Debug, Flag)]
#[flag(long = "stdin")]
/// ソースコードを標準入力から読み込む
struct StdinFlag;

#[allow(dead_code)]
#[derive(Debug, ArgGroup)]
enum InputGroup {
    File(InputArg),
    Stdin(StdinFlag),
}

#[derive(Debug, FlagArg)]
/// 出力するファイルのパス
struct OutputFlagArg(String);

#[derive(Debug, Flag)]
#[flag(long = "stdout")]
/// 標準出力に出力する
struct StdoutFlag;

#[allow(dead_code)]
#[derive(Debug, ArgGroup)]
enum OutputGroup {
    File(OutputFlagArg),
    Stdout(StdoutFlag),
}

#[derive(Flag)]
struct VerboseFlag;

fn main() {
    cli_rs::parse!(StdinFlag, StdoutFlag, InputArg, VerboseFlag, OutputFlagArg);
}
