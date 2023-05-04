mod entities;
mod migrator;

use futures::executor::block_on;
use sea_orm::*;
use sea_orm_migration::prelude::*;
use sea_query::{Alias, Query};

use entities::{prelude::*, *};
use migrator::Migrator;

const DB_URL: &str = "postgres://postgres:password@postgres:5432";
const DB_NAME: &str = "bakeries_db";

#[derive(FromQueryResult)]
struct ChefNameResult {
    name: String,
}

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DB_URL).await?;

    let db = &match db.get_database_backend() {
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

    let schema_manager = SchemaManager::new(db);

    Migrator::refresh(db).await?;
    assert!(schema_manager.has_table("bakery").await?);
    assert!(schema_manager.has_table("chef").await?);

    let happy_bakery = bakery::ActiveModel {
        name: Set("Happy Bakery".to_owned()),
        profit_margin: Set(0.0),
        ..Default::default()
    };
    let result = Bakery::insert(happy_bakery).exec(db).await?;

    let sad_bakery = bakery::ActiveModel {
        id: ActiveValue::Set(result.last_insert_id),
        name: Set("Sad Bakery".to_owned()),
        profit_margin: NotSet,
    };
    sad_bakery.update(db).await?;

    let john = chef::ActiveModel {
        name: Set("John".to_owned()),
        bakery_id: Set(result.last_insert_id),
        ..Default::default()
    };
    Chef::insert(john).exec(db).await?;

    let bakeries = Bakery::find().all(db).await?;
    assert_eq!(bakeries.len(), 1);

    let sad_bakery = Bakery::find_by_id(result.last_insert_id).one(db).await?;
    assert_eq!(sad_bakery.unwrap().name, "Sad Bakery");

    let sad_bakery = Bakery::find()
        .filter(bakery::Column::Name.eq("Sad Bakery"))
        .one(db)
        .await?;
    assert_eq!(sad_bakery.unwrap().id, 1);

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

    let la_boulangerie = Bakery::find_by_id(bakery_res.last_insert_id)
        .one(db)
        .await?
        .unwrap();

    let chefs = la_boulangerie.find_related(Chef).all(db).await?;
    let mut chef_names = chefs.iter().map(|chef| &chef.name).collect::<Vec<_>>();
    chef_names.sort_unstable();

    assert_eq!(chef_names, ["Charles", "Frederic", "Jolie", "Madeleine"]);

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

    let column = (chef::Entity, Alias::new("name"));
    let mut stmt = Query::select();
    stmt.column(column.clone())
        .from(chef::Entity)
        .join(
            JoinType::Join,
            bakery::Entity,
            Expr::col(Alias::new("bakery_id")).equals((bakery::Entity, Alias::new("id"))),
        )
        .order_by(column, Order::Asc);
    let builder = db.get_database_backend();
    let chef = ChefNameResult::find_by_statement(builder.build(&stmt))
        .all(db)
        .await?;
    let chef_names = chef.into_iter().map(|chef| chef.name).collect::<Vec<_>>();
    assert_eq!(
        chef_names,
        vec!["Charles", "Frederic", "Jolie", "Madeleine"]
    );

    let query = Query::delete()
        .from_table(bakery::Entity)
        .cond_where(Cond::any().add(Expr::col(bakery::Column::Name).eq("SQL Bakery")))
        .to_owned();
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"DELETE FROM "bakery" WHERE "name" = 'SQL Bakery'"#
    );

    la_boulangerie.delete(db).await?;

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

    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
