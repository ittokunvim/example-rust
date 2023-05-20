# アプリ

> Ref: https://bevyengine.org/learn/book/getting-started/apps

Bevyのプログラムは[`App`](https://docs.rs/bevy_app/latest/bevy_app/struct.App.html)と呼ばれており、最もシンプルなBevyアプリは次のようなものです。

```rust
use bevy::prelude::*;

fn main() {
    App::new().run();
}
```

シンプルでしょう？上記のコードを`main.rs`に記述して実行してみましょう。

```bash
cargo run
```

そうです。何も起きません。これはまだアプリに何も指示をしていないためです。
アプリは、アプリケーションのロジックを実行できるからのシェルに過ぎません。
Bevy ECSを使って、アプリにロジックを追加してみましょう。
