use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use surrealdb::{opt::RecordId, sql::Thing};

#[derive(Debug, Serialize, Deserialize)]
pub struct Run {
    pub distance: u8,
    pub time: NaiveTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DBRun {
    pub distance: u8,
    pub time: NaiveTime,
    pub id: Thing,
}
