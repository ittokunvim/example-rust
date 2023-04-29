# マイグレーション(API)

> Ref: https://www.sea-ql.org/sea-orm-tutorial/ch01-03-migration-api.html

プログラム的にマイグレーションを設定・実行したい場合は、`Migrator API`を使用します。

CLIツールをインストールすることなく、マイグレーションを実行する方法について見ていきます。

### 準備

依存関係に`sea-orm-migration`を追加します。

```toml
[dependencies]
futures = "0.3.28"
sea-orm-migration = "0.11.3"

[dependencies.sea-orm]
version = "0.11.3"
features = [
	"sqlx-postgres",
	"runtime-async-std-native-tls",
	"macros"
]
```

`migrator`モジュールを作成します。

**src/main.rs**

```rust
mod migrator;

use futures::executor::block_on;
use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};

// ...
```

**src/migrator/mod.rs**

```rust
use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![]
    }
}
```

### マイグレーションを定義

`Migration`をファイルに定義し、`migrator/mod.rs`にインクルードします。

ファイル名は、`m<date>_<6-digit>_<description>.rs`の形式にします。

Code: [src/migrator/m20230430_000001_create_bakery_table.rs](https://github.com/ittokun/example-rust/tree/main/sea_orm/bakery-backend/src/migrator/m20230430_000001_create_bakery_table.rs)

Code: [src/migrator/m20230430_000002_create_chef_table.rs](https://github.com/ittokun/example-rust/tree/main/sea_orm/bakery-backend/src/migrator/m20230430_000002_create_chef_table.rs)

Code: [src/migrator/mod.rs](https://github.com/ittokun/example-rust/tree/main/sea_orm/bakery-backend/src/migrator/mod.rs)

## マイグレーションを実行する

`Migratortrait API`を使用してマイグレーションを実行します。
これは`SchemaManager`でデータベーススキーマが正しいか確認することです。

Code: [src/main.rs](https://github.com/ittokun/example-rust/tree/main/sea_orm/bakery-backend/src/main.rs)
