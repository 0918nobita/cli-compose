use cli_rs::{Arg, Flag, FlagArg, Group};

#[derive(Debug, Arg)]
#[arg(name = "input")]
/// ソースファイルのパス
struct InputArg(String);

#[derive(Debug, Flag)]
#[flag(long = "stdin")]
/// ソースコードを標準入力から読み込む
struct StdinFlag;

#[allow(dead_code)]
#[derive(Debug, Group)]
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
#[derive(Debug, Group)]
enum OutputGroup {
    File(OutputFlagArg),
    Stdout(StdoutFlag),
}

#[derive(Flag)]
struct VerboseFlag;

fn main() {
    cli_rs::parse!(
        std::env::args().collect::<Vec<_>>(),
        StdinFlag,
        StdoutFlag,
        InputArg,
        VerboseFlag,
        OutputFlagArg
    );
}
