use crate::{
    models::{
        custom_error_model::CustomError,
        depth_history_model::PoolDepthPriceHistory,
        earning_history_model::{PoolEarningHistory, PoolEarningSummary},
        rune_pool_model::RunePool,
        swap_history_model::SwapHistory,
    },
    utils::constants::API_START_TIME,
};
use chrono::Utc;
use dotenv::dotenv;
use futures_util::StreamExt;
use mongodb::{
    bson::{doc, Bson, Document},
    Client, Collection,
};
use std::env;
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
    // helper functions
    pub async fn get_max_end_time<T>(&self, collection: &Collection<T>) -> Result<i64, CustomError>
    where
        T: serde::de::DeserializeOwned + Send + Sync,
    {
        let pipeline = vec![
            doc! {
                "$sort": {
                    "end_time": -1
                }
            },
            doc! { "$limit": 1 },
        ];

        let mut cursor = collection.aggregate(pipeline).await?;

        if let Some(result) = cursor.next().await {
            match result {
                Ok(doc) => {
                    let fetched_end_time =
                        doc.get_i64("end_time").unwrap_or(Utc::now().timestamp());
                    println!("{:?}", fetched_end_time);

                    Ok(fetched_end_time)
                }
                Err(e) => {
                    eprintln!("Failed to fetch max end_time: {}", e);
                    Err(CustomError::DatabaseError(e.to_string()))
                }
            }
        } else {
            Ok(Utc::now().timestamp())
        }
    }
}
