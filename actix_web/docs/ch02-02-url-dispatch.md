# URL Dispatch

> Ref: https://actix.rs/docs/url-dispatch

URL Dispatchは、単純なパターンマッチング言語を使用してURLをハンドラーコードにマッピングする簡単な方法を提供するものです。
パターンの1つが要求に関連づけられたパス情報と一致する場合、特定のハンドラーオブジェクトが呼び出されます。

> リクエストハンドラは、リクエストから抽出できる0個以上のパラメータを受け取り、HttpResponseに変換できる型を返す関数です。

## リソース

リソース構成は、新しいリソースをアプリケーションに追加する行為です。
リソースには、URL生成に使用される識別子として機能する名前があります。
リソースには、URLのPATH部分（`http://example.com/foo/bar?q=value`でいう`foo/bar`）と照合するためのパターンもあります。

`App::route()`メソッドは、ルートを登録する簡単な方法を提供します。
このメソッドは、単一のルートをアプリケーションルーティングテーブルに追加します。
そして、パスパターン、HTTPメソッド、ハンドラ関数を受け入れます。
`route()`メソッドは、同じパスに対して複数回呼び出される可能性があります。
その場合、複数のルートが同じリソースパスに登録されます。

```rust
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(api::index))
            .route("/user", web::post().to(handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

`App::route()`は、ルートを登録する簡単な方法を提供しますが、完全なリソース構成にアクセスするには、別の方法を使用する必要があります。
`App::service()`は、単一のリソースをアプリケーションルーティングテーブルに追加します。
このメソッドは、パスパターン、ガード、1つ以上のルートを受け入れます。

```rust
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

pub fn main() {
    App::new()
    .service(web::resource("/prefix").to(index))
        .service(
            web::resource("/user/{name}")
            .name("user_detail")
                .guard(guard::Header("content-type", "application/json"))
                .route(web::get().to(HttpResponse::Ok))
                .route(web::put().to(HttpResponse::Ok)),
        );
}
```
