# エンティティ

> Ref: https://www.sea-ql.org/sea-orm-tutorial/ch01-04-entity-generation.html

スキーマが定義されたデータベースができたので、`sea-orm-cli`でエンティティを生成します。

`sea-orm-cli`は、データベースのURLからスキーマを発見し、適切なエンティティファイルを生成することができます。

```bash
sea-orm-cli generate entity \
  -u postgres://postgres:password@postgres:5432/bakeries_db \
  -o src/entities
```

上記のコマンドを実行すると、以下のファイルが自動生成されるはずです。

```
bakery-backend
├── Cargo.toml
├── migration
│   └── ...
└── src
    ├── entities
    │   ├── chef.rs
    │   ├── bakery.rs
    │   ├── mod.rs
    │   └── prelude.rs
    └── main.rs
```

次節からは`chef.rs, bakery.rs`に焦点を当てます。これらはそれぞれ`Chef, Bakery`というテーブルを表すエンティティです。
