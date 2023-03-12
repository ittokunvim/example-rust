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
            .route("/user", web::post().to(index))
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

## ルートの設定

リソースには一連のルートが含まれています。
各ルートは順番にガードとハンドラのセットを持っています。
新しいルートは、`Resource::route()`メソッドで作成でき、新しい`Route`インスタンスへの参照を返します。
デフォルトのルートはガードを含まないため、全てのリクエストにマッチします。
デフォルトのハンドラーは`HttpNotFound`です。

アプリケーションは、リソース登録とルート登録の際に定義されたルート基準に基づいて、受信したリクエストをルーティングします。
リソースは、`Resource::route()`によって登録された順番で、それが含む全てのルートにマッチします。

```rust
App::new().service(
    web::resource("/url-dispatch/path").route(
        web::route()
            .guard(guard::Get())
            .guard(guard::Header("content-type", "text/plain"))
            .to(HttpResponse::Ok),
    ),
)
```

この例では、*GET*リクエストに対して、`Content-Type`ヘッダーが含まれ、値に`text/plain`が入っています。
もしヘッダーの値が`text/plain`で、パスが`/url-dispatch/path`に等しい場合に`HttpResponse::Ok()`が返されます。

マッチしなければ、"NOT FOUND"が返されます。

`ResourceHandler::route()`は、`Route`オブジェクトを返します。
`Route`は、ビルダー的なパターンで設定することができ、以下の設定方法があります。

- `Route::guard()`、新しいガードを登録します。
- `Route::method()`、メソッドガードを登録します。
- `Route::to()`、ルートの非同期ハンドラ関数を登録します。登録できるハンドラは1つだけで、通常はハンドラの登録は最後の設定操作となります。

## ルートマッチング

ルート設定の主な目的は、リクエストのパスをURLのパスパターンと照合することです。
`path`はリクエストされたURLのパス部分を表します。

actix-webがこれを行う方法は非常にシンプルです。
リクエストがシステムに入る時、システムに存在する各リソース構成宣言に対して、リクエストのパスを宣言されたパターンに照らし合わせます。
このチェックは、`App::service()`メソッドでルートが宣言された順番に行われます。
リソースが見つからない場合、デフォルトのリソースがマッチしたリソースとして使用されます。

ルート設定が宣言されると、ルートガード引数を含むことができます。
ルート宣言に関連付けられた全てのルートガードは、チェック中に与えられたリクエストにルート設定を使用するために`true`出なければなりません。
ルート設定に与えられたルートガード引数のうち、いずれかのガードがチェック中に`false`を返した場合、
そのルートはスキップされ、ルートマッチは順序付けられたルートセットを通して継続されます。

一致するルートがあれば、ルートマッチ処理は停止し、そのルートに関連するハンドラが起動されます。
全てのルートパターンを使い切った後、マッチするルートがない場合、*NOT FOUND*応答が返されます。

## リソースパターン

actix-webがpattern引数で使用するパターンマッチング言語の構文は単純明快です。

ルート設定に使用するパターンは、スラッシュ文字で開始します。
パターンがスラッシュ文字ではじまらない場合、マッチング時に暗黙のスラッシュが先頭に付加されます。
例えば、以下のパターンは等価です。

```txt
{foo}/bar/baz
/{foo}/bar/baz
```

可変部（置き換えマーカー）は`{identifier}`という形式で指定されます。
これは次のスラッシュ文字までの任意の文字を受け入れ、`HttpRequest.match_info()`オブジェクトの名前として使用することを意味します。

パターン中の置き換えマーカーは、正規表現`[^{}/]+`にマッチします。

`match_info`は、ルーティングパターンに基づいたURLから抽出された動的な部分を表す`Params`オブジェクトです。
`request.match_info`として提供されます。
例えば以下のパターンは、1つのリテラルセグメント(foo)と2つの置き換えマーカー(baz, bar)を定義しています。
そしてそのパターンの下は、それに一致するURLになります。

```txt
foo/{bar}/{baz}

foo/1/2         -> Params {'baz': '1', 'bar': '2'}
foo/abc/def     -> Params {'baz': 'abc', 'bar': 'def'}

foo/1/2/        -> No match (trailing slash)
bar/abc/def     -> First segment literal mismatch
```

セグメント内のセグメント置き換えマーカーに対するマッチングは、パターン内のセグメント内の最初の非英数字文字までしか行われません。
そのため以下のパターンを使用した場合、次のような結果になります。

```txt
/foo/{name}.html

/foo/biz.html    -> Params {'name': 'biz'} 
/foo/index       -> No match (trailing .html)

/foo/{name}.{ext}

/foo/biz.html     -> Params {'name': 'biz', 'ext': 'html'}
/foo/test.txt     -> Params {'name': 'test', 'ext': 'txt'}
/foo/indexhtml    -> No match (trailing extension)
```

置き換えマーカーには、パスセグメントがマーカーに一致するかどうかを判断するために使用される正規表現をオプションで指定することができます。
置き換えマーカーが、正規表現で定義された特定の文字の集合にのみ一致するように指定するには、置き換えマーカーの構文を少し拡張したものを使用する必要があります。
中括弧の中で置き換えマーカー名の後にコロンが続き、その後に正規表現を書く必要があります。
置き換えマーカー`[^/]+`に関連づけられたデフォルトの正規表現は、スラッシュ以外の1つ、または複数の文字にマッチします。
例えば、置き換えマーカー`{foo}`は、より詳細には`{foo:[^/]+}`と表記されることがあります。
これを任意の正規表現に変えて、任意の文字列にマッチさせることができます。
例えば、`{foo:\d+}`は数字のみマッチします。

セグメント置き換えマーカーにマッチするためには、セグメントに少なくとも1つの文字が含まれていなければなりません。

> パスはURL引用されず、パターンにマッチする前に有効なUnicode文字列にデコードされ、マッチしたパスセグメントを表す値もURL引用されないことになります。

```txt
foo/{bar}

http://example.com/foo/La%20Pe%C3%B1a

Params {'bar': 'La Pe\xf1a'}
```

パスセグメント内の文字列は、actix-webに提供されたパスのデコード値である必要があります。
パターン内でURLエンコードされた値を使用するのは避けた方が良いです。

```txt
/Foo%20Bar/{baz}    -> Bad

/Foo Bar/{baz}      -> Good

foo/{bar}/{tail:.*} -> Good

foo/1/2/            -> Params {'bar': '1', 'tail': '2/'}
foo/abc/def/a/b/c   -> Params {'bar': 'abc', 'tail': 'def/a/b/c'}
```

## ルートのスコープ



## マッチ情報



### パス情報抽出



## リソースURL生成



## 外部リソース



## パス正規化とリダイレクト機能



### Prefixを使用したアプリケーションの構成



## カスタムルートガード



### ガード値を変更



## Not Foundレスポンスの変更




