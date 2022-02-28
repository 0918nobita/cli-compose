# CLI 開発支援ライブラリ

[clap](https://crates.io/crates/clap) クレートの代替となることを狙って開発を進めています。

clap クレートの Derive API が単一の struct または enum に対する derive でコマンドラインパーサ全体の実装を生やすのに対し、このライブラリでは **「フラグ」「フラグ引数」「引数」「グループ」単位で derive してそれぞれに適した型安全なパーサ実装を生やし、自由に合成できる** ようにしています。この API で必要十分だと考えており、シンプルさのために、clap クレートの Builder API 相当の API は提供しないことにしています。

まだ初期段階なので、未実装あるいは動作が不安定な部分が含まれていることをご了承ください。

( ``cli-rs`` は仮称であり、正式名称は思いついていません。何か良さげな名前があれば、提案していただけると嬉しいです。)

```rust
use cli_rs::{Arg, FlagArg, Flag, Group};

#[derive(Debug, Arg)]
struct Input(String);

#[derive(Debug, Flag)]
#[flag(long = "stdin")]
struct StdinFlag;

#[derive(Debug, Group)]
enum InputGroup {
    File(Input),
    Stdin(StdinFlag),
}

#[derive(Debug, FlagArg)]
#[flag_arg(short = 'o')]
struct Output(String);

#[derive(Debug, Flag)]
#[flag(long = "stdout")]
struct StdoutFlag;

#[derive(Debug, Group)]
enum OutputGroup {
    File(Output),
    Stdout(StdoutFlag),
}

#[derive(Flag)]
struct Verbose;

pub fn main() {
    cli_rs::parse!(
        group {
            input = InputGroup,
            output = OutputGroup,
        }
        flag {
            verbose = Verbose,
        }
    );

    println!("Input: {:?}", input);
    println!("Output: {:?}", output);
    println!("Verbose: {:?}", verbose);
}
```

`cargo run -- --verbose --stdin --stdout` :

```
Input: Stdin(StdinFlag)
Output: Stdout(StdoutFlag)
Verbose: Some(Verbose)
```

`cargo run -- -o output.txt input.txt` :

```
Input: File("input.txt")
Output: File("output.txt")
Verbose: None
```
