use std::env;

use dotenv::dotenv;
use mongodb::{Client, Collection};

use crate::models::depth_history_model::PoolDepthPriceHistory;

pub struct DataBase{
    pub depth_history: Collection<PoolDepthPriceHistory>
}

impl DataBase{
    pub async fn init() -> Self{
        dotenv().ok();

        let uri = env::var("DB").unwrap();
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("token-metrics");
        // println!("{:?}",db);
        let depth_history_collection = db.collection("depth_history");
        DataBase{
            depth_history : depth_history_collection
        }
    }
}