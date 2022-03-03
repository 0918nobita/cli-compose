# Group (グループ)

[位置指定引数](./01-pos-arg.md)、[引数付きオプション](./02-opt-arg.md)、および[引数なしオプション](./03-opt.md)をまとめて、全体に対して修飾子を適用できます。

```rust
#[derive(PosArg)]
struct Input(String);

#[derive(Opt)]
#[opt(long = "stdin")]
struct StdinOpt;

enum InputGroup {
    File(Input),
    Stdin(StdinOpt),
}

#[derive(ArgOpt)]
#[arg_opt(short = 'o')]
struct Output(String);

#[derive(Opt)]
#[opt(long = "stdout")]
struct StdoutOpt;

enum OutputGroup {
    File(Output),
    Stdout(StdoutOpt),
}

fn main() {
    cli_rs::parse!(
        std::env::args(),
        group(count = one, explicit = yes) {
            input = InputGroup,
            output = OutputGroup,
        }
    );
}
```

## 修飾子について

## count

グループのメンバーで指定できる個数を設定します。

### 取りうる値とその意味

- `one`：１つだけ指定
- `zero-or-one`：省略または１つだけ指定
- `at-least-one`：少なくとも１つ指定
- `any`：任意の個数指定

### デフォルト値

`one`

## explicit

ヘルプメッセージでグループの存在を明示するかどうかを設定します。

### 取りうる値とその意味

- `yes`：明示する
- `no`：明示しない

### デフォルト値

`yes`

WIP
