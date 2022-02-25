# CLI 開発支援ライブラリ

↓のように使えることを目指して開発を進めています。

```rust
use cli_rs::{Arg, ArgGroup, FlagArg, Flag};

#[derive(Debug, Arg)]
struct Input(String);

#[derive(Debug, Flag)]
#[flag(long = "stdin")]
struct StdinFlag;

#[derive(Debug, ArgGroup)]
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

#[derive(Debug, ArgGroup)]
enum OutputGroup {
    File(Output),
    Stdout(StdoutFlag),
}

#[derive(Flag)]
struct Verbose;

pub fn main() {
    cli_rs::parse!(
        input = InputGroup,
        output = OutputGroup,
        verbose = Verbose,
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
