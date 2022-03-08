# cli-rs

> Next-generation, type-safe CLI parser for Rust

[clap](https://crates.io/crates/clap) クレートの代替となることを目指して開発を進めています。

まだ初期段階なので、未実装あるいは動作が不安定な部分が含まれていることをご了承ください。

## 開発目標

以下のように動作することを目指しています。

```rust
use cli_rs::FromKebabStr;

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

cli_rs::parser!(
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
