# 非同期プログラミング

> Ref: https://www.sea-ql.org/SeaORM/docs/introduction/async/

Rustの非同期プログラミングは最近の開発で、`Rust 1.39`で安定化されたばかりです。
非同期エコシステムは急速に発展しており、`seaORM`は非同期サポートを念頭に置いて1から作られた最初のクレートの1つです。

まず最初に学ぶべきは、[Future](https://rust-lang.github.io/async-book/02_execution/02_future.html)トレイトです。
これは将来的に何らかの値を計算して返す関数のプレースホルダーです。
`Future`は遅延型なので、実際に処理を行うには`.await`を呼び出す必要があります。
例えば、`future::join_all`で複数のクエリを並列に実行するなど、`Future`を使えば少ないプログラミング労力で並列処理を実現することができます。

2つ目にRustの`async`は、シンタックスシュガーを含む[マルチスレッドプログラミング](https://rust-lang.github.io/async-book/03_async_await/01_chapter.html)です。
`Future`はスレッド間を移動する可能性があるので、`async`本体で使用する変数はスレッド間を移動できるもの、つまり[Send](https://doc.rust-lang.org/nomicon/send-and-sync.html)でなければなりません。

3つ目にRustには複数の非同期ランタイムが存在します。おそらく、最も広く使われているランタイムは次の通りです。

- [actix](https://crates.io/crates/actix)
- [async-std](https://crates.io/crates/async-std)
- [tokio](https://crates.io/crates/tokio)

SeaORMの基盤ドライバである[`SQLx`](https://crates.io/crates/sqlx)は、この3つをサポートしています。

これらの概念は、非同期Rustを使いこなすために覚えておくことです。
