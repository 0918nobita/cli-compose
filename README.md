# cli-compose

> Composable, strict CLI parser with compile-time analysis for Rust

[![Test](https://github.com/0918nobita/cli-compose/actions/workflows/test.yml/badge.svg)](https://github.com/0918nobita/cli-compose/actions/workflows/test.yml) [![Clippy](https://github.com/0918nobita/cli-compose/actions/workflows/clippy.yml/badge.svg)](https://github.com/0918nobita/cli-compose/actions/workflows/clippy.yml) [![Rustfmt](https://github.com/0918nobita/cli-compose/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/0918nobita/cli-compose/actions/workflows/rustfmt.yml) [![codecov](https://codecov.io/gh/0918nobita/cli-compose/branch/main/graph/badge.svg?token=PBAO6WHOKE)](https://codecov.io/gh/0918nobita/cli-compose)

Please note that this is still at an early stage of development. Hence this may contain bugs, unimplemented features, or unstable features.

## Implementation goals

### Directory structure

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
name = "example-cli"
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

// All derivers treat doc comments as help messages.

/// Source file path
#[derive(Debug, PosArg)]
pub struct Input(String);

/// Reads source code from stdin
#[derive(Debug, Opt)]
#[opt(long = "stdin")] // overrides its long name
pub struct StdinOpt;

/// Settings related to input
#[derive(SingleSelect)]
pub enum InputGroup {
    Input(Input),
    StdinOpt(StdinOpt),
}

/// Source file format
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

/// Output file path
#[derive(Debug, ArgOpt)]
#[arg_opt(short = 'o')] // configures its short name
pub struct Output(String);

/// Outputs to stdout
#[derive(Debug, Opt)]
#[opt(long = "stdout")]
pub struct StdoutOpt;

/// Settings related to output
#[derive(SingleSelect)]
pub enum OutputGroup {
    Output(Output),
    StdoutOpt(StdoutOpt),
}

#[derive(Opt)]
pub struct Verbose;

#[derive(Cli)]
#[cli(
    name = "example",
    version = from_crate,
    desc = from_crate
)]
pub struct ExampleCli;
```

### `build.rs`

```rust
use cli_compose::codegen::define_cli;
use opts::{Cli, InputFormat, InputGroup, OutputGroup, Verbose};

fn main() {
    // generates `$OUT_DIR/cli_compose/example_cli.rs`,
    // which defines `ExampleCliResult` struct
    // and implements `cli_compose::runtime::AsCli<ExampleCliResult>` trait for `ExampleCli` struct
    define_cli::<Cli>("opts")
        .unwrap()
        .single_select::<InputGroup>()
        .arg_opt::<InputFormat>()
        .single_select::<OutputGroup>()
        .opt::<Verbose>()
        .build("ExampleCliResult")
        .unwrap();
    }
}
```

### `src/main.rs`

```rust
use cli_compose::runtime::{use_cli, AsCli};

// includes `$OUT_DIR/cli_compose/example_cli.rs`
// and imports `ExampleCli` struct
use_cli! { example_cli::ExampleCli }

pub fn main() {
    let res = Cli::parse(std::env::args()); // res: ExampleCliResult

    println!("Input: {:?}", res.input);
    println!("InputFormat: {:?}", res.input_format);
    println!("Output: {:?}", res.output);
    println!("Verbose: {:?}", res.verbose);
}
```

### Results

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

## Example program to test features already implemented

```bash
cargo run -p playground
```
