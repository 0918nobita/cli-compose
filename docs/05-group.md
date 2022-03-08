# Group (グループ)

[位置指定引数](./01-pos-arg.md)、[引数付きオプション](./02-opt-arg.md)、および[引数なしオプション](./03-opt.md)をまとめて、そのメンバー全体を修飾できます。

## 「うち１つだけ」というルールを設ける場合

列挙した引数のうち１つだけを指定するように強制し、パースに成功すると列挙型の値で返します。

```rust
use cli_compose::{Group, Opt};

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

cli_compose::parser!(
    Cli,
    group(explicit = yes) InputGroup: input,
);

fn main() {
    Cli::parse(std::env::args());
}
```

## 「省略、または１つだけ」というルールを設ける場合

```rust
cli_compose::parser!(
    Cli,
    group(required = no, explicit = yes) InputGroup: input,
);
```

## 「１つ以上」というルールを設ける場合

WIP

## 修飾子

### required

その引数を必須にするかどうかを設定します。  
設定値によって、`parser!` マクロで指定した型 `T` に対する戻り値の型が変わります。

#### 取りうる値とその意味

- `yes`：必須。戻り値の型は `T` のままです。
- `no`：省略可能。戻り値の型は `Option<T>` に変わります。

#### デフォルト値

`yes`

### explicit

ヘルプメッセージでグループの存在を明示するかどうかを設定します。

#### 取りうる値とその意味

- `yes`：明示する
- `no`：明示しない

#### デフォルト値

`yes`
