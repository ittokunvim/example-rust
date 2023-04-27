# セットアップ

> Ref: https://www.sea-ql.org/sea-orm-tutorial/ch01-01-project-setup.html

このセクションではフォルダ構造やクレートの依存関係など、プロジェクトのセットアップを行います。

チュートリアルを通してMySQLデータベースを使用しますが、SeaORMの全ての機能は、前述したようにデータベースの実装に依存するものではありません。

### 依存関係に`sea-orm`を追加

```bash
cargo init bakery-backend
```

```toml
# Cargo.toml

...

[dependencies]
sea-orm = { version = "^0.9.0", features = ["sqlx-mysql", "runtime-async-std-native-tls", "macros"] }
```

MySQLを使用するためDBドライバ機能`sqlx-mysql`を使用します。

`runtime-async-std-native-tls`は、このプロジェクトのために任意に選択された非同期ランタイムです。
詳細は[ドキュメント](https://www.sea-ql.org/SeaORM/docs/install-and-config/database-and-async-runtime/#async_runtime)を参照してください。

`macros`は、いくつかの`Derive`マクロを使用することができるオプション機能です。

### データベースを接続

`async/await`による非同期プログラミングを活用できるように、`futures`を依存関係として追加します。

```bash
cargo add futures
```

```rust
use futures::executor::block_on;
use sea_orm::{Database, DbErr};

const DB_URL: &str = "postgres://bakery:password@localhost";
const DB_NAME: &str = "bakery";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DB_URL).await?;

    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
```

データベースの設定がされている場合、上手く動作するはずです。

パニックになる場合は、データベースのURLが間違っている可能性があります。

ハングアップする場合は、データベースが立ち上がっていない可能性があります。

### データベースを作成

MySQL, PostgreSQLの場合、特定のデータベースインスタンスを作成することができます。
名前は`bakery`にしましょう。

```rust
use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};

// ...

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DB_URL).await?;

    let _db = &match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS {};", DB_NAME),
            ))
            .await?;

            let url = format!("{}/{}", DB_URL, DB_NAME);
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS {};", DB_NAME),
            ))
            .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE {};", DB_NAME),
            ))
            .await?;

            let url = format!("{}/{}", DB_URL, DB_NAME);
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => db,
    };

    Ok(())
}
```

このスニペットはSeaORMがデータベースにとらわれないものであることを示しています。
1種類のデータベースしか使用されないことが確実な場合のみ、選択したデータベースのケースを処理することができます。
