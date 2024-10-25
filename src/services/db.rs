use std::env;

use chrono::Utc;
use dotenv::dotenv;
use futures_util::StreamExt;
use mongodb::bson::Document;
use mongodb::{bson::doc, Client, Collection};

use crate::models::api_request_param_model::QueryParams;
use crate::models::custom_error_model::CustomError;
use crate::models::{
    depth_history_model::PoolDepthPriceHistory,
    earning_history_model::{PoolEarningHistory, PoolEarningSummary},
    rune_pool_model::RunePool,
    swap_history_model::SwapHistory,
};
use crate::utils::db_helper_utils::{build_query_sort_skip, get_max_start_time_of_collection, get_seconds_per_interval};

pub struct DataBase {
    pub depth_history: Collection<PoolDepthPriceHistory>,
    pub earnings: Collection<PoolEarningHistory>,
    pub earnings_summary: Collection<PoolEarningSummary>,
    pub swap_history: Collection<SwapHistory>,
    pub rune_pool_history: Collection<RunePool>,
}

impl DataBase {
    pub async fn init() -> Self {
        dotenv().ok();

        let uri = env::var("DB").unwrap();
        let client = Client::with_uri_str(&uri).await.unwrap();
        let db = client.database("token-metrics");
        let depth_history_collection = db.collection("depth_history");
        let earnings_collection = db.collection("earnings");
        let earning_summary_collection = db.collection("earnings_summary");
        let swap_history_collection = db.collection("swap_history");
        let rune_pool_collection = db.collection("rune_pool_history");

        DataBase {
            depth_history: depth_history_collection,
            earnings: earnings_collection,
            earnings_summary: earning_summary_collection,
            swap_history: swap_history_collection,
            rune_pool_history: rune_pool_collection,
        }
    }
}
