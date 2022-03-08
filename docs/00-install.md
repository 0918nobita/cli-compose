# cli-compose のインストール方法

まだ初回リリースをしていないため、git リモートリポジトリを参照する方法でのみインストールできます。

`Cargo.toml` で以下のように追記してください。

```toml
[dependencies.cli-compose]
git = "https://github.com/0918nobita/cli-compose"
package = "cli-compose"
```
