# Actix Web

> Ref: https://actix.rs/docs/

Actix Webは`actix`アクターフレームワーク上で構築されていました。
現在はアクターフレームワークとはほとんど関係がなく、別のシステムを使用して構築されています。
`actix`はまだ維持されていますが、非同期・大気のエコシステムが成熟するにつれ、一般的なツールとしての有用は低下しています。
現時点では、`actix`の使用は、WebSocketエンドポイントにのみ必要になっています。

Actix Webは強力で実用的なフレームワークです。全ての意図と目的のために、いくつか捻りを加えたようなマイクロフレームワークとなっています。
Rustプログラマーであるなら、Actix Webはすぐになれるでしょうが、別のプログラミング言語を使用しているプログラマでも、Actix Webは簡単に習得できるはずです。

Actix Webで開発されたアプリケーションは、ネイティブ実行可能ファイルに含まれるHTTPサーバーを公開します。これを`nginx`などの別のHTTPサーバーの背後に配置するか、そのまま提供することが出来ます。
別のHTTPサーバーが全くない場合でも、Actix Webは、`HTTP/1, HTTP/2, TLS(HTTPS)`を提供するのに十分強力です。
これにより本番用の小さなサービスを構築するのに役立ちます。

## Rustのインストール

公式の[Rustガイド](https://doc.rust-lang.org/book/ch01-01-installation.html)の手順に従って、Rustをインストールしましょう。

Actix WebでサポートされているRustの最小バージョン(MSRV)は`1.59`です。`rustup update`コマンドを実行すると最新のRustバージョンが利用可能になります。

### やっはろー

まずCargoプロジェクトを作成します。

```bash
cargo new actix_web
cd actix_web
```

次に以下のコマンドで、`actix-web`を依存関係として追加します。

```bash
cargo add actix-web
```

リクエストハンドラは、0個以上のパラメータを受け付ける非同期関数を使用します。
これらのパラメータはリクエストから抽出され、HttpResponseに変換可能な型を返します。

```rust
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
```

上記のコードの`manual_hello`以外の関数には、組み込みマクロを使用してルーティング情報が直接関連付けられています。
これによりハンドラが応答するメソッドとパスを指定します。またマクロを使用せずに関連付けを行う方法もあります。

次に`App`インスタンスを作成し、リクエストハンドラを登録します。
`App::services`ルーティングマクロを使用するハンドラと、`App::route`パスとメソッドを宣言して手動でルーティングするハンドラを設定します。

```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

これで準備は完了です！最後にプログラムを実行して、サーバーが立ち上がっているか確認してみましょう。

```bash
cargo run
# get root
curl http://localhost:8080/
# post /echo
curl http://localhost:8080/echo -X POST -d "hello, world"
# get /hey
curl http://localhost:8080/hey
```
