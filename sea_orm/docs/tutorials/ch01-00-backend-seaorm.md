# SeaORMでバックエンドを構築

> Ref: https://www.sea-ql.org/sea-orm-tutorial/ch01-00-build-backend-getting-started.html

ソースコードの全文は[GitHub](https://github.com/SeaQL/sea-orm-tutorial/tree/master/bakery-backend)で公開されています。

この章ではSeaORMを使ってバックエンドアプリケーションを構築します。
これはデータベースとの通信レイヤーとして機能します。

このアプリケーションはパン屋さんのデータベースのインターフェイスをシミュレートするものです。
エンティティは`Bakery, Chef`の2つだけにして、スキーマは後で見ていきます。

### データベースを選ぶ

バックエンドの構築を開始する前に、データベースが稼働していることを確認したいです。
データベースのセットアップは、このチュートリアルの範囲を超えています。

SeaORM自体は、MySQL, PostgreSQL, SQLiteを含む異なるデータベース実装に不可知論的です。

ただし選択するデータベースによって以下の注意が必要です。

- 適切なDBドライバ機能を有効にする必要がある
- 有効なデータベースのURLを使用すること

| Database           | Example Database URL                |
| ------------------ | ----------------------------------- |
| MySQL              | mysql://root:root@localhost:3306    |
| PostgreSQL         | postgres://root:root@localhost:5432 |
| SQLite (in file)   | sqlite:./sqlite.db?mode=rwc         |
| SQLite (in memory) | sqlite::memory:                     |

次節ではその使い方と場所を具体的に紹介します。

データベースは`PostgreSQL`を使用し、プロジェクトのセットアップに進みます。
