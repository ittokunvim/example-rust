# とりあえず動かす

> Ref: https://bevyengine.org/learn/book/getting-started

このセクションでは、Bevyの開発環境のセットアップと、簡単なBevyアプリの書き方を説明します。

### クイックスタート

すぐにBevyを動かしたい場合で、すでにRustが動作している場合は、以下のガイドをご覧ください。

「高速コンパイル」の設定は次のページにあるので、そちらを先に読んでもいいかもしれません。

#### 例

1. [Bevyレポジトリ](https://github.com/bevyengine/bevy)をクローンします。

```bash
git clone https://github.com/bevyengine/bevy
```

2. Bevyリポジトリに移動します。

```bash
cd bevy
```

3. Bevyのバージョンを切り替えます。

```bash
# use the latest Bevy release
git checkout latest
# or a specific version
git checkout v0.10.0
```

4. `example`フォルダにあるサンプルコードを実行します。

```bash
cargo run --example breakout
```

### Bevyを依存関係として追加する

Bevyは[crates.io/crates/bevy](https://crates.io/crates/bevy)でライブラリとして提供されています。

プロジェクトに追加する最も簡単な方法は、`cargo add`を使用することです。

```bash
cargo add bevy
```

または、`Cargo.toml`に手動で追加することもできます。

```bash
[dependencies]
bevy = "0.10" # make sure this is the latest version
```

必ず最新の`bevy crate`バージョンを使用しましょう。
