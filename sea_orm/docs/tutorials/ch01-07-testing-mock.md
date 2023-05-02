# モックテスト

> Ref: https://www.sea-ql.org/sea-orm-tutorial/ch01-07-mock-testing.html

実際のデータベースを使わずにアプリケーションのロジックを検証したいことがあります。
そのためSeaORMでは開発時に使用する`MockDatabase`インターフェイスを用意しています。

例えばデータベース層はアプリケーションロジック層から独立している必要があるため、ユニットテストに実際のデータベースをセットアップして使用することは避けたいです。
モックインターフェイスを使用することで、データベース層の動作が安定し、正しくなるため、発生したエラーはアプリケーション・ロジック層のバグに起因するものしか無くなります。

また、開発環境の移植性を重視する場合、じつデータベースは好ましくない場合があります。
モックインターフェイスを効果的に使用することで、実際のデータベースのセットアップやメンテナンスが不要となり、アプリケーションロジックの開発者は事実上どこでも作業を行うことができるようになります。

### `mock`機能を追加

**Cargo.toml**

```toml
[dependencies.sea-orm]
version = "0.11.3"
features = [
	"sqlx-postgres",
	"runtime-async-std-native-tls",
	"macros",
    "mock",
]
```

### クエリ結果を定義

まずモックデータベースに何を返させるかを定義します。

以下のコードは`append_query_result`関数にベクターを渡し、クエリの結果を返します。

```rust
let db: &DatabaseConnection = &MockDatabase::new(DatabaseBackend::Postgres)
    .append_query_results(vec![
        vec![bakery::Model {
            id: 1,
            name: "Happy Bakery".to_owned(),
            profit_margin: 0.0,
        }],
        vec![
            bakery::Model {
                id: 1,
                name: "Happy Bakery".to_owned(),
                profit_margin: 0.0,
            },
            bakery::Model {
                id: 2,
                name: "Sad Bakery".to_owned(),
                profit_margin: 100.0,
            },
            bakery::Model {
                id: 3,
                name: "La Boulangerie".to_owned(),
                profit_margin: 17.89,
            },
        ],
    ])
    .append_query_results(vec![vec![
        chef::Model {
            id: 1,
            name: "Jolie".to_owned(),
            contact_details: None,
            bakery_id: 3,
        },
        chef::Model {
            id: 2,
            name: "Charles".to_owned(),
            contact_details: None,
            bakery_id: 3,
        },
        chef::Model {
            id: 3,
            name: "Madeleine".to_owned(),
            contact_details: None,
            bakery_id: 3,
        },
        chef::Model {
            id: 4,
            name: "Frederic".to_owned(),
            contact_details: None,
            bakery_id: 3,
        },
    ]])
    .into_connection();
```

注意として、クエリ結果に複数のモデルが含まれていて、`Entity::find().one(db)`が呼び出された時、最初のモデルのみが返されます。
クエリに含まれる残りのモデルは破棄されます。

### クエリを使う

クエリ結果をモックして、アプリケーションロジックの他の部分に渡すことができます。

```rust
let happy_bakery = Bakery::find().one(db).await?;
assert_eq!(
    happy_bakery.unwrap(),
    bakery::Model {
        id: 1,
        name: "Happy Bakery".to_owned(),
        profit_margin: 0.0,
    }
);

let all_bakeries = Bakery::find().all(db).await?;
assert_eq!(
    all_bakeries,
    vec![
        bakery::Model {
            id: 1,
            name: "Happy Bakery".to_owned(),
            profit_margin: 0.0,
        },
        bakery::Model {
            id: 2,
            name: "Sad Bakery".to_owned(),
            profit_margin: 100.0,
        },
        bakery::Model {
            id: 3,
            name: "La Boulangerie".to_owned(),
            profit_margin: 17.89,
        },
    ]
);

let la_boulangerie_chefs = Chef::find().all(db).await?;
assert_eq!(
    la_boulangerie_chefs,
    vec![
        chef::Model {
            id: 1,
            name: "Jolie".to_owned(),
            contact_details: None,
            bakery_id: 3,
        },
        chef::Model {
            id: 2,
            name: "Charles".to_owned(),
            contact_details: None,
            bakery_id: 3,
        },
        chef::Model {
            id: 3,
            name: "Madeleine".to_owned(),
            contact_details: None,
            bakery_id: 3,
        },
        chef::Model {
            id: 4,
            name: "Frederic".to_owned(),
            contact_details: None,
            bakery_id: 3,
        },
    ]
);
```

### モックの実行結果

CRUD操作の結果をモックするには、`append_exec_results`メソッドを使用します。

上記と類似性が高いため、このチュートリアルでは説明しません。ドキュメントを参照してください。
