# マイグレーション(CLI)

> Ref: https://www.sea-ql.org/sea-orm-tutorial/ch01-02-migration-cli.html

このセクションと次のセクションでは新しいおもちゃのデータベーススキーマを作成し、SeaORMの醍醐味を学びます。

本説ではマイグレーションを用いた以下のようなシンプルなスキーマを定義します。

![Schema](https://www.sea-ql.org/sea-orm-tutorial/assets/er_diagram.png)

### sea-orm-cli

初心者の方は、`sea-orm-cli`を使用してマイグレーションを定義・実行することをお勧めします。

```bash
cargo install sea-orm-cli

sea-orm-cli migrate -h
```

`migration`フォルダを初期化します。

```bash
sea-orm-cli migrate init
```

### マイグレーションを定義

`Bakery`テーブルと`Chef`テーブルを定義するためにマイグレーションファイルを更新します。

ファイル名は、`m<date>_<6-digit-index>_<description>.rs`の形式に従って名付ける必要があります。

マイグレーションの定義の詳細については、[SchemaManager](https://docs.rs/sea-orm-migration/*/sea_orm_migration/manager/struct.SchemaManager.html)をご覧ください。

```
- m20220101_000001_create_table.rs
+ m20220101_000001_create_bakery_table.rs
+ m20220101_000002_create_chef_table.rs
```

Code: [migration/src/m20230429_000001_create_bakery_table.rs](https://github.com/ittokun/example-rust/tree/main/sea_orm/bakery-backend/migration/src/m20230429_000001_create_bakery_table.rs)

Code: [migration/src/m20230429_000002_create_chef_table.rs](https://github.com/ittokun/example-rust/tree/main/sea_orm/bakery-backend/migration/src/m20230429_000002_create_chef_table.rs)

Code: [migration/src/lib.rs](https://github.com/ittokun/example-rust/tree/main/sea_orm/bakery-backend/migration/src/lib.rs)

`migration`クレートで以下の機能が有効になっているか確認してください。
データベースドライバーの機能は、使用するデータベースと一致している必要があります。

```toml
# migration/Cargo.toml

[dependencies.sea-orm-migration]
version = "0.11.0"
features = [
  "runtime-tokio-rustls",  # `ASYNC_RUNTIME` feature
  "sqlx-postgres",         # `DATABASE_DRIVER` feature
]
```

### マイグレーションを実行

以下のコードを実行してマイグレーションして見ましょう。

```bash
DATABASE_URL="postgres://postgres:password@postgres:5432/bakeries_db" sea-orm-cli migrate refresh
```
