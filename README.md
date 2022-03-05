# CLI 開発支援ライブラリ

[clap](https://crates.io/crates/clap) クレートの代替となることを目指して開発を進めています。

clap クレートの Derive API が単一の struct または enum に対する derive でコマンドラインパーサ全体の実装を生やすのに対し、このライブラリでは **「位置指定引数」「引数付きオプション」「引数なしオプション」「グループ」単位で derive してそれぞれに適した型安全なパーサ実装を生やし、自由に合成できる** ようにしています。この API で必要十分だと考えており、シンプルさのために、clap クレートの Builder API 相当の API は提供しないことにしています。

まだ初期段階なので、未実装あるいは動作が不安定な部分が含まれていることをご了承ください。

( ``cli-rs`` は仮称であり、正式名称は思いついていません。何か良さげな名前があれば、提案していただけると嬉しいです。)

## 開発目標

以下のように動作することを目指しています。

```rust
use cli_rs::{ArgOpt, FromKebabStr, Group, Opt, PosArg};

// ドキュメンテーションコメントはヘルプメッセージとして扱われます

/// ソースファイルのパス
#[derive(Debug, PosArg)]
struct Input(String);

/// ソースコードを標準入力から読み込む
#[derive(Debug, Opt)]
#[opt(long = "stdin")] // オプション名の上書き
struct StdinOpt;

#[derive(Debug, Group)]
enum InputGroup {
    File(Input),
    Stdin(StdinOpt),
}

/// ソースファイルの形式
#[derive(Debug, ArgOpt, FromKebabStr)]
enum InputFormat {
    Json,
    Yaml,
}

impl Default for InputFormat {
    fn default() -> Self {
        InputFormat::Json
    }
}

/// 出力するファイルのパス
#[derive(Debug, ArgOpt)]
#[arg_opt(short = 'o')] // 短縮名の指定
struct Output(String);

/// 標準出力に出力する
#[derive(Debug, Opt)]
#[opt(long = "stdout")]
struct StdoutOpt;

#[derive(Debug, Group)]
enum OutputGroup {
    File(Output),
    Stdout(StdoutOpt),
}

#[derive(Opt)]
struct Verbose;

pub fn main() {
    cli_rs::parse!(
        std::env::args(),

        group(count = one, explicit = yes) {
            input = InputGroup,
            output = OutputGroup,
        }

        arg_opt(use_default = yes) {
            input_format = InputFormat,
        }

        opt {
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
Input: Stdin(StdinOpt)
InputFormat: Json
Output: Stdout(StdoutOpt)
Verbose: Some(Verbose)
```

`$ cargo run -- --input-format yaml -o output.txt input.yaml` :

```text
Input: File("input.txt")
InputFormat: Yaml
Output: File("output.txt")
Verbose: None
```

## サンプルの実行方法

```bash
cargo run -p playground
```
