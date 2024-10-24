use actix_web::web::Data;
use chrono::Utc;
use tokio::time::{interval, Duration};
use dotenv::dotenv;
use std::time::Instant;
use crate::models::{depth_history_model::PoolDepthPriceHistory, earning_history_model::PoolEarningHistory, rune_pool_model::RunePool, swap_history_model::SwapHistory};

use super::db::DataBase;

const ONE_HOUR_SECS: u64 = 3_600;

pub async fn run_cron_job(db: Data<DataBase>, pool: &str) {
    dotenv().ok();

    let mut interval = interval(Duration::from_secs(ONE_HOUR_SECS));

    loop {
        interval.tick().await; // Wait for the next tick
        let start_time = Instant::now();

        // Just try to perform tasks and ignore the logging part for errors
        if perform_all_tasks(&db, &pool).await.is_ok() {
            println!("All fetches completed.");
        }

        println!(
            "Data fetch cycle completed, duration: {:?}",
            start_time.elapsed()
        );
    }
}

async fn perform_all_tasks(db: &DataBase, pool: &str) -> Result<(), ()> {
    let one_hour_ago = Utc::now().timestamp() - ONE_HOUR_SECS as i64;
    let interval_str = "hour";
    let start_timer = &one_hour_ago.to_string();

    // Collect task results but discard error messages
    let tasks = vec![
        SwapHistory::fetch_swap_history(db, pool, interval_str, "400", start_timer).await,
        RunePool::fetch_rune_pool(db, "hour", interval_str, start_timer).await,
        PoolEarningHistory::fetch_earning_history(db, interval_str, "400", start_timer).await,
        PoolDepthPriceHistory::fetch_price_history(db, "BTC.BTC", interval_str, "400", start_timer).await,
    ];

    // If any task returns an error, immediately return an Err result
    for task in tasks {
        if task.is_err() {
            return Err(());
        }
    }

    Ok(())
}
