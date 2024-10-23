use std::env;

use dotenv::dotenv;
use futures_util::StreamExt;
use mongodb::bson::{Bson, Document};
use mongodb::error::Error as mongoError;
use mongodb::{bson::doc, Client, Collection};

use crate::{models::{depth_history_model::PoolDepthPriceHistory, earning_history_model::{PoolEarningHistory, PoolEarningSummary}, rune_pool_model::RunePool, swap_history_model::SwapHistory}, routes::depth_route::QueryParams};

#[derive(Debug)]
pub enum MyError {
    InvalidInput(String),
    DatabaseError(String)
}
impl From<mongoError> for MyError {
    fn from(err: mongoError) -> Self {
        MyError::DatabaseError(err.to_string())
    }
}

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

    // /depths
    pub async fn get_depth_price_history_api(&self, params: QueryParams) -> Result<Vec<Document>, MyError> {
        let mut query = doc! {};
        let QueryParams {
            pool,
            interval,
            count,
            to,
            from,
            page,
            sort_by,
            sort_order,
        } = params;

        let page = page.unwrap_or(1);

        if let Some(pool) = pool {
            if pool != "BTC.BTC" {
                return Err(MyError::InvalidInput(
                    "Currently we only work for BTC.BTC depths and earning history".to_string(),
                ));
            }
            query.insert("pool", pool);
        }

        if let Some(from) = from {
            query.insert("start_time", doc! { "$gte": from as i64 });
        }

        if let Some(to) = to {
            query.insert("end_time", doc! { "$lte": to as i64 });
        }

        let limit: u32 = if let Some(count) = count {
            count
        }else{
            400
        };

        let sort_filter = if let Some(sort_by) = sort_by {
            if let Some(sort_order) = sort_order {
                doc! { sort_by: sort_order }
            } else {
                doc! { sort_by: 1 }
            }
        } else {
            doc! { "end_time": -1 }
        };
        let skip_size = (page-1)*(limit as u64);
        let interval = interval.unwrap_or(String::from("hour"));
        if interval == "hour" {
            println!("{}",query);
            let mut cursor = self
                .depth_history
                .find(query)
                .skip(skip_size as u64)
                .limit(limit as i64)
                .sort(sort_filter)
                .await?;

            let mut query_response = Vec::new();

            while let Some(result) = cursor.next().await {
                match result {
                    Ok(record) => {
                        let mut record = mongodb::bson::to_document(&record).expect("Error parsing the document");
                        record.remove("_id");
                        query_response.push(record);
                    }
                    Err(e) => {
                        eprintln!("Failed fetching data! {}", e);
                    }
                }
            }
            // println!("{:?}",query_response);
            return Ok(query_response);
        }
        let seconds_per_interval = match interval.as_str() {
            "hour" => 3600,
            "day" => 86_400,
            "week" => 604_800,
            "month" => 2_678_400,
            "year" => 31_622_400,
            _ => 3_600
        };
        let pipeline = vec![
            doc! {"$match" : query},
            doc! {
                "$group" : {
                    "_id" : {
                        "interval_start" : {
                            "$subtract" : [
                                { "$add": ["$end_time", 1] }, 
                                { "$mod": [ 
                                    { "$subtract": ["$end_time", 1] },  
                                    seconds_per_interval 
                                ]}
                            ]
                        }
                    }
                },
                "avg_asset_depth": { "$avg": "$asset_depth" },
                "avg_asset_price": { "$avg": "$asset_price" },
                "avg_asset_price_usd": { "$avg": "$asset_price_usd" },
                "avg_liquidity_units": { "$avg": "$liquidity_units" },
                "avg_luvi": { "$avg": "$luvi" },
                "avg_members_count": { "$avg": "$members_count" },
                "avg_rune_depth": { "$avg": "$rune_depth" },
                "avg_synth_supply": { "$avg": "$synth_supply" },
                "avg_synth_units": { "$avg": "$synth_units" },
                "avg_units": { "$avg": "$units" }
            },
            doc! { "$project": {
                "_id": 0,
                "start_time": {
                    "$subtract": [ "$_id.interval_start", { "$mod": [ "$_id.interval_start", seconds_per_interval ] }]
                },
                "end_time": {
                    "$add": [
                        { "$subtract": [ "$_id.interval_start", { "$mod": [ "$_id.interval_start", seconds_per_interval ] }] },
                        seconds_per_interval
                    ]
                }
            }},
            doc! { "$sort": sort_filter },
            doc! { "$skip": skip_size as i64 },
            doc! { "$limit": limit as i64 },                   
        ];
        let mut cursor = self.depth_history.aggregate(pipeline).await?;
        let mut query_response = Vec::new();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(mut record) => {
                    record.remove("_id");  
                    query_response.push(record);
                },
                Err(e) => eprintln!("Error fetching document: {:?}", e),
            }
        }
        Ok(query_response)
    }
}
