# Group (グループ)

[位置指定引数](./01-pos-arg.md)、[引数付きオプション](./02-opt-arg.md)、および[引数なしオプション](./03-opt.md)をまとめて、そのメンバー全体を修飾できます。

## 「うち１つだけ」というルールを設ける場合

列挙した引数のうち１つだけを指定するように強制し、パースに成功すると列挙型の値で返します。

位置指定引数、引数付きオプション、または引数なしオプションを唯一のフィールドとして持つヴァリアントで構成されている列挙型に `#[derive(Group)]` を付与し、属性 `#[group(one)]` を指定してください。

以下の例では、位置指定引数 `[input]` か引数なしオプション `--stdin` のどちらか１つだけを指定するように強制しています。
名前 `input` は、結果として `InputGroup` 列挙型の値で束縛されます。

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

fn main() {
    cli_rs::parse!(
        std::env::args(),

        group(explicit = yes) {
            input = InputGroup,
        }
    );
}
```

## 「省略、または１つだけ」というルールを設ける場合

属性 `#[group(one)]` を付与したグループを `parse!` マクロの引数においてオプショナルとして指定することで実現します。

```rust
...
fn main() {
    cli_rs::parse!(
        std::env::args(),

        group(required = no, explicit = yes) {
            input = InputGroup,
        }
    );
}
```

## 「１つ以上」というルールを設ける場合

`#[group(at-least-one)]` 属性を指定してください。

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
