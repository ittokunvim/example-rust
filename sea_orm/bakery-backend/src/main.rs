mod entities;
mod migrator;

use futures::executor::block_on;
use sea_orm::*;
use sea_orm_migration::prelude::*;

use entities::{prelude::*, *};
use migrator::Migrator;

const DB_URL: &str = "postgres://postgres:password@postgres:5432";
const DB_NAME: &str = "bakeries_db";

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

    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
