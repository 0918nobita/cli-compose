# cli-compose

> Composable, strict CLI parser with compile-time analysis for Rust

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
```

### `build.rs`

```rust
use cli_compose::codegen::define_cli;

fn main() {
    // generates `$OUT_DIR/cli_compose/cli.rs`, which defines `Cli` struct
    define_cli! {
        Cli [version = from_crate, description = from_crate] {
            input = opts::InputGroup,

            // equivalent to `input_format = opts::InputFormat`
            opts::InputFormat,

            output = opts::OutputGroup,

            opts::Verbose,
        },
    }
}
```

### `src/main.rs`

```rust
// includes `$OUT_DIR/cli_compose/cli.rs`
cli_compose::runtime::use_cli!(Cli);

pub fn main() {
    let cli = Cli::parse(std::env::args());

    println!("Input: {:?}", cli.input);
    println!("InputFormat: {:?}", cli.input_format);
    println!("Output: {:?}", cli.output);
    println!("Verbose: {:?}", cli.verbose);
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
