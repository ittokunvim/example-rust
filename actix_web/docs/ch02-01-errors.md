# エラー

> Ref: https://actix.rs/docs/errors

Actix Webは、Webハンドラからのエラー処理に、独自の`actix_web::error::Error`タイプと、`actix_web::error::ResponseError`トレイトを使用しています。

ハンドラが`ResponseError`を実装した`Result`で`Error`を返す場合、actix-webはそのエラーをHTTP応答として、対応する`actix_web::http::StatusCode`でレンダーします。
デフォルトでは、内部サーバーエラーが生成されます。（以下参照）

```rust
pub trait ResponseError {
    fn error_response(&self) -> Response<Body>;
    fn status_code(&self) -> StatusCode;
}
```

`Responder`は、互換性のある結果をHTTPレスポンスに変換します。

```rust
impl<T: Responder, E: Into<Error>> Responder for Result<T, E>
```

上記のコードの`Error`はactix-webのエラー定義であり、`ResponseError`を実装したエラーは自動的に変換することができます。

Actix Webは、いくつかの一般的な非actixエラーに対する`ResponseError`の実装を提供します。
例えば、ハンドラーが`io::Error`で応答した場合、そのエラーは`HttpInternalServerError`に変換されます。

```rust
use std::io;
use actix_files::NamedFile;

fn index(_req: HttpRequest) -> io::Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}
```

## カスタムエラーレスポンス

ここでは、`ResponseError`の例として、`derive_more`クレートによる宣言エラー列挙型を使用します。

```rust

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display(fmt = "my error: {}", name)]
struct CustomError {
    name: &'static str,
}

impl actix_web::error::ResponseError for CustomError {}

#[get("custom-error")]
async fn custom_error() -> Result<&'static str, CustomError> {
    Err(CustomError { name: "test" })
}
```

`ResponseError`は、`error_response()`のデフォルト実装で500をレンダリングするようになっています。
上記のインデックスハンドラが実行されるとこのような状態になります。

`error_response()`をオーバーライドして、より有用な結果を得ることができます。

```rust
#[derive(Debug, derive_more::Display)]
enum CustomErrorEnum {
    #[display(fmt = "internal error")]
    InternalError,
    #[display(fmt = "bad request")]
    BadClientData,
    #[display(fmt = "timeout")]
    Timeout,
}

impl error::ResponseError for CustomErrorEnum {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            CustomErrorEnum::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomErrorEnum::BadClientData => StatusCode::BAD_REQUEST,
            CustomErrorEnum::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

#[get("custom-error-enum")]
async fn custom_error_enum() -> Result<&'static str, CustomErrorEnum> {
    let internal_error = Err(CustomErrorEnum::InternalError)?;
    let _bad_client_data = Err(CustomErrorEnum::BadClientData)?;
    let _timeout = Err(CustomErrorEnum::Timeout)?;

    internal_error
}
```

## エラーヘルパー

Actix-webは、他のエラーから特定のHTTPエラーコードを生成するのに便利なエラーヘルパー関数のセットを持っています。
ここでは、`map_err`を使用して、`ResponseError`トレイトを実装していない`CustomError`を400に変換します。

```rust
#[derive(Debug)]
struct CustomError {
    name: &'static str,
}

#[get("/map-err")]
async fn map_err() -> Result<&'static str> {
    let result: Result<&'static str, CustomError> = Err(CustomError { name: "test error" });
    Ok(result.map_err(|e| error::ErrorBadRequest(e.name))?)
}
```
