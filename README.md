# cli-compose

> Next-generation, type-safe CLI parser for Rust

[clap](https://crates.io/crates/clap) クレートの代替となることを目指して開発を進めています。

まだ初期段階なので、未実装あるいは動作が不安定な部分が含まれていることをご了承ください。

## 設計

### 第1案：パーサ実装のみを生やす

コマンドライン引数に関する型定義はユーザーが通常の構文で記述する

「``parser!`` マクロが展開時に型定義の属性を取得して、それに応じてコード生成すること」ができるならこのAPIが望ましいが、Rust の通常の言語機能では実現しそうにない

「derive マクロ側で一部の属性を外部ファイルに書き出して、 ``parser!`` マクロでそれを読み込む」という方法でなら実現するかもしれない

<details>
<summary>コード例</summary>

```rust
use cli_compose::{ArgOpt, FromKebabStr, Group, Opt, PosArg};

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
#[arg_opt(use_default)]
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
#[group(count = one, explicit = true)]
enum OutputGroup {
    File(Output),
    Stdout(StdoutOpt),
}

#[derive(Opt)]
struct Verbose;

cli_compose::parser!(
    Cli,

    group InputGroup: input

    group OutputGroup: output

    arg_opt InputFormat: input_format

    opt Verbose: verbose
);

pub fn main() {
    let cli = Cli::parse(std::env::args());

    println!("Input: {:?}", cli.input);
    println!("InputFormat: {:?}", cli.input_format);
    println!("Output: {:?}", cli.output);
    println!("Verbose: {:?}", cli.verbose);
}
```
</details>

### 第2案：DSL で型定義・パーサ実装を生やす

<details>
<summary>コード例</summary>

```rust
use cli_compose::FromKebabStr;

/// ソースファイルの形式
#[derive(Debug, FromKebabStr)]
enum InputFormat {
    Json,
    Yaml,
}

impl Default for InputFormat {
    fn default() -> Self {
        InputFormat::Json
    }
}

cli_compose::parser!(
    Cli,

    def Input = pos_arg String [ desc = "入力ファイルのパス" ];

    def StdinOpt = opt [ long = "stdin", desc = "標準入力から読み込む" ];

    -- input = group { Input, StdinOpt } [ explicit = true ];

    -- input_format =
        arg_opt
            InputFormat
            [ use_default = true, desc = "入力ファイルの形式" ];

    def Output = arg_opt String [ short = 'o', desc = "出力ファイルのパス" ];

    def StdoutOpt = opt [ long = "stdout", desc = "標準出力に出力する" ];

    -- output = group { Output, StdoutOpt } [ explicit = true ];

    -- verbose = opt [ desc = "詳細を標準エラーに出力する" ];
);

pub fn main() {
    let cli = Cli::parse(std::env::args());

    println!("Input: {:?}", cli.input);
    println!("InputFormat: {:?}", cli.input_format);
    println!("Output: {:?}", cli.output);
    println!("Verbose: {:?}", cli.verbose);
}
```
</details>

### 各コード例の実行時の挙動

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
