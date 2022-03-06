# Group (グループ)

[位置指定引数](./01-pos-arg.md)、[引数付きオプション](./02-opt-arg.md)、および[引数なしオプション](./03-opt.md)をまとめて、そのメンバー全体を修飾できます。

## 「うち１つだけ」というルールを設ける場合

列挙した引数のうち１つだけを指定するように強制し、パースに成功すると列挙型の値で返します。

```rust
use cli_rs::{Group, Opt};

#[derive(PosArg)]
struct Input(String);

#[derive(Opt)]
#[opt(long = "stdin")]
struct StdinOpt;

#[derive(Group)]
#[group(one)]
enum InputGroup {
    File(Input),
    Stdin(StdinOpt),
}

cli_rs::parser!(
    Cli,
    group(explicit = yes) {
        input: InputGroup,
    }
);

fn main() {
    Cli::parse(std::env::args());
}
```

## 「省略、または１つだけ」というルールを設ける場合

```rust
cli_rs::parser!(
    Cli,
    group(required = no, explicit = yes) {
        input: InputGroup,
    }
);
```

## 「１つ以上」というルールを設ける場合

WIP

<!--
```rust
#[derive(Group)]
#[group(at-least-one)]

```
-->

## グループ修飾子について

<!--
### count

グループのメンバーで指定できる個数を設定します。

#### 取りうる値とその意味

- `one`：１つだけ指定
- `zero-or-one`：省略または１つだけ指定
- `at-least-one`：少なくとも１つ指定
- `any`：任意の個数指定

#### デフォルト値

`one`

-->

### required

WIP

#### 取りうる値とその意味

WIP

#### デフォルト値

`yes`

### explicit

ヘルプメッセージでグループの存在を明示するかどうかを設定します。

#### 取りうる値とその意味

- `yes`：明示する
- `no`：明示しない

#### デフォルト値

`yes`
