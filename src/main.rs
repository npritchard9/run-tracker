use chrono::NaiveTime;
use clap::Parser;
use run_tracker::{db::*, models::Run};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct CliRun {
    #[arg(short, long)]
    pub distance: u8,
    #[arg(short, long)]
    pub time: NaiveTime,
}

#[tokio::main]
async fn main() {
    let args = CliRun::parse();
    let run = Run {
        distance: args.distance,
        time: args.time,
    };
    let db = get_db().await.unwrap();
    let _r = insert_run(run, &db).await.unwrap();
    let runs = get_all_runs(&db).await.unwrap();
    let weekly_runs = get_weekly_stats(&db).await.unwrap();
    println!("{:?}", runs);
    println!("{:?}", weekly_runs)
}
