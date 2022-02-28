# CLI 開発支援ライブラリ

[clap](https://crates.io/crates/clap) クレートの代替となることを目指して開発を進めています。

clap クレートの Derive API が単一の struct または enum に対する derive でコマンドラインパーサ全体の実装を生やすのに対し、このライブラリでは **「フラグ」「フラグ引数」「引数」「グループ」単位で derive してそれぞれに適した型安全なパーサ実装を生やし、自由に合成できる** ようにしています。この API で必要十分だと考えており、シンプルさのために、clap クレートの Builder API 相当の API は提供しないことにしています。

まだ初期段階なので、未実装あるいは動作が不安定な部分が含まれていることをご了承ください。

( ``cli-rs`` は仮称であり、正式名称は思いついていません。何か良さげな名前があれば、提案していただけると嬉しいです。)

↓初回リリースでは以下のように動作することを目指しています。

```rust
use cli_rs::{Arg, FlagArg, Flag, Group};

// ドキュメンテーションコメントはヘルプメッセージとして扱われます

/// ソースファイルのパス
#[derive(Debug, Arg)]
struct Input(String);

// #[flag(long = ..)]: フラグ名を「型名をケバブケースに自動変換したもの」から変更したい場合、文字列リテラルを指定して上書きできます

/// ソースコードを標準入力から読み込む
#[derive(Debug, Flag)]
#[flag(long = "stdin")]
struct StdinFlag;

// Group: どちらか一方を指定しないとエラーになります

#[derive(Debug, Group)]
enum InputGroup {
    File(Input),
    Stdin(StdinFlag),
}

// #[flag_arg(default)]: このフラグ引数が省略された場合、デフォルト値が返ります

/// ソースファイルの形式
#[derive(Debug, FlagArg)]
#[flag_arg(default)]
enum InputFormat {
    Json,
    Yaml,
}

impl Default for InputFormat {
    fn default() -> Self {
        InputFormat::Json
    }
}

// #[flag_arg(short = ..)]: フラグ引数の短縮名を指定できます

/// 出力するファイルのパス
#[derive(Debug, FlagArg)]
#[flag_arg(short = 'o')]
struct Output(String);

/// 標準出力に出力する
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
        flag_arg {
            input_format = InputFormat,
        }
        flag {
            verbose = Verbose,
        }
    );

    println!("Input: {:?}", input);
    println!("InputFormat: {:?}", input_format);
    println!("Output: {:?}", output);
    println!("Verbose: {:?}", verbose);
}
```

`$ cargo run -- --verbose --stdin --stdout` :

```text
Input: Stdin(StdinFlag)
InputFormat: Json
Output: Stdout(StdoutFlag)
Verbose: Some(Verbose)
```

`$ cargo run -- --input-format yaml -o output.txt input.yaml` :

```text
Input: File("input.txt")
InputFormat: Yaml
Output: File("output.txt")
Verbose: None
```
