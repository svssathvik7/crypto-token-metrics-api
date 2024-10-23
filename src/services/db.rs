use std::{collections, env};

use dotenv::dotenv;
use mongodb::{Client, Collection};

use crate::models::{depth_history_model::PoolDepthPriceHistory, earning_history_model::{PoolEarningHistory, PoolEarningSummary}, swap_history_model::SwapHistory};

pub struct DataBase{
    pub depth_history: Collection<PoolDepthPriceHistory>,
    pub earnings : Collection<PoolEarningHistory>,
    pub earnings_summary : Collection<PoolEarningSummary>,
    pub swap_history : Collection<SwapHistory>
}

impl DataBase{
    pub async fn init() -> Self{
        dotenv().ok();

        let uri = env::var("DB").unwrap();
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("token-metrics");
        let depth_history_collection = db.collection("depth_history");
        let earnings_collection = db.collection("earnings");
        let earning_summary_collection = db.collection("earnings_summary");
        let swap_history_collection = db.collection("swap_history");
        DataBase{
            depth_history : depth_history_collection,
            earnings : earnings_collection,
            earnings_summary : earning_summary_collection,
            swap_history : swap_history_collection
        }
    }
}