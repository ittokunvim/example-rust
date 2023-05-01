# リレーションセレクト

> Ref: https://www.sea-ql.org/sea-orm-tutorial/ch01-06-relational-select.html

前節では単一のエンティティに対してセレクトを実行する方法を見てきました。

しかしリレーショナルデータベースは、エンティティを関係で結び、異なるエンティティ間でクエリを実行することができます。

例えば、あるパン屋さんの、そこで働く全てのシェフを見つけるといったことができます。

以前、以下のようなコードを実行し、パン屋とシェフをデータベースに挿入しました。

```rust
let la_boulangerie = bakery::ActiveModel {
    name: Set("La Boulangerie".to_owned()),
    profit_margin: Set(0.0),
    ..Default::default()
};
let bakery_res = Bakery::insert(la_boulangerie).exec(db).await?;

for chef_name in ["Jolie", "Charles", "Madeleine", "Frederic"] {
    let chef = chef::ActiveModel {
        name: Set(chef_name.to_owned()),
        bakery_id: Set(bakery_res.last_insert_id),
        ..Default::default()
    };
    Chef::insert(chef).exec(db).await?;
}
```

パン屋「ラ・ブーランジェリー」で働くシェフは4人で、以下のように見つけることができます。

```rust
let la_boulangerie = Bakery::find_by_id(bakery_res.last_insert_id)
    .one(db)
    .await?
    .unwrap();

let chefs = la_boulangerie.find_related(Chef).all(db).await?;
let mut chef_names = chefs.iter().map(|chef| &chef.name).collect::<Vec<_>>();
chef_names.sort_unstable();

assert_eq!(chef_names, ["Charles", "Frederic", "Jolie", "Madeleine"]);
```

より高度な使用方法については、[ドキュメント](https://www.sea-ql.org/SeaORM/docs/basic-crud/select/#find-related-models)をご覧ください。
