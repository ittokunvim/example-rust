# CRUDの基本操作

> Ref: https://www.sea-ql.org/sea-orm-tutorial/ch01-05-basic-crud-operations.html

このセクションでは定義したスキーマを使用して基本的な操作を行う方法を見ていきます。

### エンティティを使う

エンティティはデータベース内のテーブルをRustで表現したものです。
SeaORMでは、これらのエンティティを利用して、プログラム的にデータベースに対する操作を実行することができます。

**src/main.rs**

```rust
mod entities;

// ...

use entities::{prelude::*, *};
```

### 挿入と更新

エンティティの`ActiveModel`を利用して、挿入・更新操作を行うことができます。

`Bakery`テーブルに"Happy Bakery"という新しいベーカリーを挿入して見ましょう。

```rust
let happy_bakery = bakery::ActiveModel {
    name: Set("Happy Bakery".to_owned()),
    profit_margin: Set(0.0),
    ..Default::default()
};
let result = Bakery::insert(happy_bakery).exec(db).await?;
```

"Happy Bakery"を"Sad Bakery"に変更する場合、以下のように行います。

```rust
let sad_bakery = bakery::ActiveModel {
    id: ActiveValue::Set(result.last_insert_id),
    name: Set("Sad Bakery".to_owned()),
    profit_margin: NotSet,
};
sad_bakery.update(db).await?;
```

"Sad Bakery"にシェフを加える場合、以下のように行います。

```rust
let john = chef::ActiveModel {
    name: Set("John".to_owned()),
    bakery_id: Set(result.last_insert_id),
    ..Default::default()
};
Chef::insert(john).exec(db).await?;
```

### 探す(single entity)

データベースに登録されているパン屋のすべて、または一部を以下のように検索することができます。

```rust
let bakeries = Bakery::find().all(db).await?;
assert_eq!(bakeries.len(), 1);

let sad_bakery = Bakery::find_by_id(result.last_insert_id).one(db).await?;
assert_eq!(sad_bakery.unwrap().name, "Sad Bakery");

let sad_bakery = Bakery::find()
    .filter(bakery::Column::Name.eq("Sad Bakery"))
    .one(db)
    .await?;
assert_eq!(sad_bakery.unwrap().id, 1);
```

複数のエンティティのリレーショナルセレクトについては、次節で説明します。

### 削除する

作成したパン屋を削除するには以下のように行います。

```rust
let john = chef::ActiveModel {
    id: ActiveValue::Set(1),
    ..Default::default()
};
john.delete(db).await?;

let sad_bakery = bakery::ActiveModel {
    id: ActiveValue::Set(1),
    ..Default::default()
};
sad_bakery.delete(db).await?;

let bakeries = Bakery::find().all(db).await?;
assert!(bakeries.is_empty());
```
