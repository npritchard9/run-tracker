use chrono::Utc;
use surrealdb::engine::local::{Db, File};
use surrealdb::sql::Value;
use surrealdb::Surreal;

use super::models::*;

pub async fn get_db() -> surrealdb::Result<Surreal<Db>> {
    let db = Surreal::new::<File>("runs.db").await?;
    db.use_ns("my_ns").use_db("my_db").await?;
    Ok(db)
}

pub async fn insert_run(r: Run, db: &Surreal<Db>) -> surrealdb::Result<DBRun> {
    let id = vec![Value::from(Utc::now())];
    let run = db.create(("run", id)).content(r).await?;
    Ok(run)
}

pub async fn get_all_runs(db: &Surreal<Db>) -> surrealdb::Result<Vec<DBRun>> {
    let runs = db.select("run").await?;
    Ok(runs)
}

pub async fn get_weekly_stats(db: &Surreal<Db>) -> surrealdb::Result<Vec<DBRun>> {
    let mut res = db
        .query("select * from run:[time::now() - 1w]..[time::now()]")
        .await?;
    let runs = res.take(0)?;
    Ok(runs)
}
