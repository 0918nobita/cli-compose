# cli-compose

> Next-generation, type-safe CLI parser for Rust

[clap](https://crates.io/crates/clap) クレートの代替となることを目指して開発を進めています。

まだ初期段階なので、未実装あるいは動作が不安定な部分が含まれていることをご了承ください。

## 開発目標

以下のように動作することを目指しています。

### ディレクトリ構造

```text
project
├ opts
│ ├ src
│ │ └ lib.rs
│ └ Cargo.toml
├ src
│ └ main.rs
├ build.rs
└ Cargo.toml
```

### `Cargo.toml`

```toml
[workspace]
members = [ "opts" ]

[package]
name = "cli-example"
edition = "2021"

[dependencies]
opts = { path = "./opts" }

[dependencies.cli-compose]
git = "https://github.com/0918nobita/cli-compose"
package = "cli-compose"
features = [ "runtime" ]

[build-dependencies.cli-compose]
git = "https://github.com/0918nobita/cli-compose"
package = "cli-compose"
features = [ "codegen" ]
```

### `opts/Cargo.toml`

```toml
[package]
name = "opts"
edition = "2021"

[dependencies.cli-compose]
git = "https://github.com/0918nobita/cli-compose"
package = "cli-compose"
features = [ "schema" ]
```

### `opts/src/lib.rs`

```rust
use cli_compose::schema::{ArgOpt, FromKebabStr, SingleSelect, Opt, PosArg};

// ドキュメンテーションコメントはヘルプメッセージとして扱われます

/// ソースファイルのパス
#[derive(Debug, PosArg)]
pub struct Input(String);

/// ソースコードを標準入力から読み込む
#[derive(Debug, Opt)]
#[opt(long = "stdin")] // オプション名の上書き
pub struct StdinOpt;

/// 入力関連の設定
#[derive(SingleSelect)]
pub enum InputGroup {
    Input(Input),
    StdinOpt(StdinOpt),
}

/// ソースファイルの形式
#[derive(Debug, ArgOpt, FromKebabStr)]
#[arg_opt(use_default)]
pub enum InputFormat {
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
pub struct Output(String);

/// 標準出力に出力する
#[derive(Debug, Opt)]
#[opt(long = "stdout")]
pub struct StdoutOpt;

/// 出力関連の設定
#[derive(SingleSelect)]
pub enum OutputGroup {
    Output(Output),
    StdoutOpt(StdoutOpt),
}

#[derive(Opt)]
pub struct Verbose;
```

### `build.rs`

```rust
use cli_compose::codegen::define_cli;

fn main() {
    define_cli! {
        Cli,

        version = from_crate,

        description = from_crate,

        members = {
            input = opts::InputGroup,

            // input_format = opts::InputFormat は↓のように略記できる
            opts::InputFormat,

            output = opts::OutputGroup,

            opts::Verbose,
        },
    }
}

/*
// define_cli! マクロが $OUT_DIR/cli_compose/cli.rs を生成する
struct Cli {
    input: opts::InputGroup,
    input_format: opts::InputFormat,
    output: opts::OutputGroup,
    verbose: Option<opts::Verbose>,
}

impl Cli {
    fn parse(args: impl Iterator<Item = String>) -> Self {
        ...
    }
}
*/
```

### `src/main.rs`

```rust
use cli_compose::runtime::use_cli;

// $OUT_DIR/cli_compose/cli.rs を include する
use_cli!(Cli);

pub fn main() {
    let cli = Cli::parse(std::env::args());

    println!("Input: {:?}", cli.input);
    println!("InputFormat: {:?}", cli.input_format);
    println!("Output: {:?}", cli.output);
    println!("Verbose: {:?}", cli.verbose);
}
```

### 実行時の挙動

`$ cargo run -- --verbose --stdin --stdout`：

```text
Input: Stdin(StdinOpt)
InputFormat: Json
Output: Stdout(StdoutOpt)
Verbose: Some(Verbose)
```

`$ cargo run -- --input-format yaml -o output.txt input.yaml`：

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
