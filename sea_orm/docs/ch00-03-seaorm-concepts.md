# SeaORMの概念

> Ref: https://www.sea-ql.org/SeaORM/docs/introduction/sea-orm/

SeaORMでは、テーブルの集合体を持つデータベースを`Schema`と呼びます。

各テーブルはSeaORMの`Entity`に対応しており、関連するテーブルに対して`CRUD`(Create, Read, Update, Delete)操作を行う時に役立ちます。

`Entity`トレイトは、実行時にそのプロパティ(Column, relation, PrimaryKey)を検査するためのAPIを提供します。

各テーブルには複数のカラムがあり、それらは属性と呼ばれます。

これらの属性とその値は、Rustの構造体(Model)にまとめられ、操作できるようになっています。

しかし`Model`は読み込み操作のみです。挿入、更新、削除を行うには各属性にメタデータを付加する`ActiveModel`を使用する必要があります。

最後にSeaORMにはシングルトン（グローバルコンテキスト）は存在しません。
アプリケーションコードは、`DatabaseConnection`の所有権を管理する責任があります。
そして、すぐにSeaORMを始められるように以下のウェブフレームワークの統合例を提供しています。

- [Rocket](https://github.com/SeaQL/sea-orm/tree/master/examples/rocket_example)
- [Actix](https://github.com/SeaQL/sea-orm/tree/master/examples/actix_example)
- [axum](https://github.com/SeaQL/sea-orm/tree/master/examples/axum_example)
- [poem](https://github.com/SeaQL/sea-orm/tree/master/examples/poem_example)
