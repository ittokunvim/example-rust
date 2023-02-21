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

## アプリケーションの作成

`actix-web`は、Rustでウェブサーバーやアプリケーションを構築するための様々なプリミティブを提供します。
ルーティング、ミドルウェア、リクエストの前処理、レスポンスの後処理などです。

全ての`actix-web`サーバーは、Appインスタンスを中心に構築されています。
これは、リソースやミドルウェアのルートを登録するために使用されます。
また同じスコープ内のすべてのハンドラで共有されるアプリケーションの状態も保存されます。

アプリケーションのスコープは、すべてのルートの名前空間として機能します。
つまり特定のアプリケーションスコープのすべてのルートは、同じURLパスのプレフィックスを持ちます。
アプリケーションのプレフィックスは、常に先頭のスラッシュを含んでいます。
提供されたプレフィクスが先頭のスラッシュを含んでいない場合、自動的に挿入されます。
プレフィクスは値のパスセグメントで構成されている必要があります。

```rust
async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/app")
                    .route("/index.html", web::get().to(index)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

この例では、`/app`という接頭子を持つアプリケーションと`index.html`というリソースを作成します。以下のコマンドで動作を確認してみましょう。

```bash
curl http://localhost:8080/app/index.html
```

### 状態

アプリケーションの状態は、同じスコープ内のすべてのルートとリソースで共有されます。
状態は`web::Data<T>`を使ってアクセスします。またミドルウェアからでもアクセス可能です。

```rust
struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

以下のコマンドを実行して、"Hello Actix Web"と出力されるか確認してみましょう。

```bash
curl http://localhost:8080/
```

### 共有変数状態

`HttpServer`は、アプリケーションインスタンスではなく、アプリケーションファクトリを受け付けます。
`HttpServer`は、各スレッドに対してアプリケーションのインスタンスを構築します。
そのためアプリケーションのデータを複数回構築する必要があります。
異なるスレッド間でデータを共有したい場合は、`Send + Sync`のような共有可能なオブジェクトを使用する必要があります。

内部的には、`web::Data`は`Arc`を使用しています。
そのため2つ以上のArcを作らないように、`App::app_data()`を使って登録する前にデータを作成する必要があります。

```rust
struct AppStateWithCounter {
    counter: Mutex<i32>,
}

#[get("/")]
async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Hello {app_name}, Request number: {counter}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        app_name: String::from("Actix Web"),
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

主な点

- `HttpServer::new`に渡されたクロージャの内部で初期化された状態は、ローカルのワーカースレッドにあり、変更すると同期が解除される可能性がある。
- グローバルに共有される状態を実現するには、`HttpServer::new`に渡されたクロージャの外部で状態を作成し、移動とクローンをする必要があります。

## スコープを使ったアプリケーションの構成

`web::scope()`メソッドは、リソースグループのプレフィクスを設定することが出来ます。
このスコープは、リソース設定によって追加されるすべてのリソースパターンの前に付加されるプレフィクスを表します。
これを使用すると、同じリソース名を維持したまま、一連のルートをマウントするのに役立ちます。

```rust
#[actix_web::main]
async fn main() {
    let scope = web::scope("/users").service(show_users);
    App::new().service(scope);
}
```

上記の例では、`show_users`ルートは`/show`ではなく`/users/show`という有効なルートパターンを持ちます。
これは、アプリケーションのスコープ引数がパターンの前に追加されるからです。
そしてルートはURLパスが`/users/show`の場合にのみマッチし、ルート名`show_users`で`HttpRequest.url_for()`関数が呼ばれると、その同じパスのURLが生成されることになります。

## アプリガードとバーチャルホスティング

ガードは、リクエストオブジェクトの参照を受け取り、`true, false`を返す単純な関数です。
形式的にガードは、`Guard`トレイトを実装した任意のオブジェクトです。

以下の例は、ガードで提供されているHeaderを使って、リクエストのヘッダ情報に基づいたフィルタを行っています。

```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "www.rust-lang.org"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("www") })),
            )
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "users.rust-lang.org"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("user") })),
            )
    })
}
```

## 設定

シンプルさと再利用性のために、`App, web::Scope`には`configure`メソッドがあります。
この関数は、設定の一部を別のモジュール、あるいはライブラリに移動させるのに便利です。
例えば、リソースの設定の一部を別のモジュールに移動させることが出来ます。

```rust
fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
        .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed))
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(move || {
        App::new()
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

以下のコマンドを入力して、出力を確認してみましょう。

```bash
curl http://localhost:8080/app
curl http://localhost:8080/api/test
```
