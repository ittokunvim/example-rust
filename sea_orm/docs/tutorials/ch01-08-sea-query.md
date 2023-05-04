# SeaQuery (Optional)

> Ref: https://www.sea-ql.org/sea-orm-tutorial/ch01-08-sql-with-sea-query.html

SQLの柔軟性を好む場合は、SeaQueryを使用してあらゆるクエリや操作に対してSQLライクなステートメントを構築することができます。

SeaQueryはSeaORMに内蔵されているので、余分な設定は必要ありません。

### ステートメントを挿入

**SQL**

```sql
INSERT INTO `bakery` (`name`, `profit_margin`) VALUES (`SQL Bakery`, -100)
```

**SeaQuery**

```rust
use sea_query::{Alias, Query};

let columns: Vec<Alias> = ["name", "profit_margin"]
    .into_iter()
    .map(Alias::new)
    .collect();

let mut stmt = Query::insert();
stmt.into_table(bakery::Entity).columns(columns);

// Invoke `values_panic()` for each row
stmt.values_panic(["SQL Bakery".into(), (-100.0).into()]);

let builder = db.get_database_backend();
db.execute(builder.build(&stmt)).await?;
```

### ステートメントを選択

**SQL**

```sql
SELECT `chef`.`name` FROM `chef` JOIN `bakery` ON `chef`.`bakery_id` = `bakery`.`id` ORDER BY `chef`.`name` ASC
```

**SeaQuery**

一部のカラムにしか興味がない場合は、クエリ結果を保持するための構造体を定義します。
この構造体は`FromQueryResult`という特性から派生している必要があります。

全てのカラムから取得する場合は`chef::Model`を使用することができます。

```rust
use sea_query::{Alias, Expr, JoinType, Order, Query};

#[derive(FromQueryResult)]
struct ChefNameResult {
    name: String,
}

// ...

let column = (chef::Entity, Alias::new("name"));

let mut stmt = Query::select();
stmt.column(column.clone()) // Use `expr_as` instead of `column` if renaming is necessary
    .from(chef::Entity)
    .join(
        JoinType::Join,
        bakery::Entity,
        Expr::tbl(chef::Entity, Alias::new("bakery_id"))
            .equals(bakery::Entity, Alias::new("id")),
    )
    .order_by(column, Order::Asc);

let builder = db.get_database_backend();
let chef = ChefNameResult::find_by_statement(builder.build(&stmt))
    .all(db)
    .await?;

let chef_names = chef.into_iter().map(|b| b.name).collect::<Vec<_>>();

assert_eq!(
    chef_names,
    vec!["Charles", "Frederic", "Jolie", "Madeleine"]
);
```

### デバッグする

SeaQueryで生成されたステートメントのSQLを確認するには、`stmt.to_string(query_builder)`を使用します。

```rust
println!("{}", stmt.to_string(MysqlQueryBuilder));
```
