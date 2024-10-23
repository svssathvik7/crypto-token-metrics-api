use std::env;

use chrono::Utc;
use dotenv::dotenv;
use futures_util::StreamExt;
use mongodb::bson::Document;
use mongodb::{bson::doc, Client, Collection};

use crate::models::api_request_param_model::QueryParams;
use crate::models::custom_error_model::CustomError;
use crate::{
    models::{
        depth_history_model::PoolDepthPriceHistory,
        earning_history_model::{PoolEarningHistory, PoolEarningSummary},
        rune_pool_model::RunePool,
        swap_history_model::SwapHistory,
    },
};


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
    pub async fn get_depth_price_history_api(
        &self,
        params: QueryParams,
    ) -> Result<Vec<Document>, CustomError> {
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
            limit
        } = params;

        let seconds_per_interval = match interval.as_ref().unwrap().as_str() {
            "hour" => 3600,
            "day" => 86_400,
            "week" => 604_800,
            "month" => 2_678_400,
            "year" => 31_622_400,
            _ => 3_600,
        };

        let page = page.unwrap_or(1);

        if let Some(pool) = pool {
            // for now due to volume and computation constraints our depth history is confined to only BTC.BTC pool
            if pool != "BTC.BTC"{
                return Err(CustomError::InvalidInput("Depth and price history for only BTC.BTC available".to_string()));
            }
            query.insert("pool", pool);
        }
        
        let limit: i8 = if let Some(limit) = limit { limit } else { 20 };

        if let Some(from) = from {
            query.insert("start_time", doc! { "$gte": from as i64 });
        }
        else{
            let calc_start = if let Some(to) = to{
                to as i64
            }else{
                Utc::now().timestamp() as i64
            };
            let count = count.unwrap_or(400) as i64;
            let queried_interval_duration = seconds_per_interval as i64;
            query.insert("start_time", doc! {"$gte": calc_start-(count*queried_interval_duration) as i64});
        }

        if let Some(to) = to {
            query.insert("end_time", doc! { "$lte": to as i64 });
        }


        let sort_filter = if let Some(sort_by) = sort_by {
            if let Some(sort_order) = sort_order {
                let sort_order = if sort_order == 1 {1} else {-1};
                doc! { sort_by: sort_order }
            } else {
                doc! { sort_by: 1 }
            }
        } else {
            doc! { "end_time": -1 }
        };
        let skip_size = (page - 1) * (limit as u64);
        let interval = interval.unwrap_or(String::from("hour"));
        println!("{}",query);
        if interval == "hour" {
            println!("{}", query);
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
                        let mut record = mongodb::bson::to_document(&record)
                            .expect("Error parsing the document");
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
        let pipeline = vec![
            doc! { "$match": query }, // Match stage
            doc! {
                "$group": {
                    "_id": {
                        "interval_start": {
                            "$subtract": [
                                { "$add": ["$end_time", 1] },
                                { "$mod": [
                                    { "$subtract": ["$end_time", 1] },
                                    seconds_per_interval
                                ]}
                            ]
                        }
                    },
                    "asset_depth": { "$last": "$asset_depth" },
                    "asset_price": { "$last": "$asset_price" },
                    "asset_price_usd": { "$last": "$asset_price_usd" },
                    "liquidity_units": { "$last": "$liquidity_units" },
                    "luvi": { "$last": "$luvi" },
                    "members_count": { "$last": "$members_count" },
                    "rune_depth": { "$last": "$rune_depth" },
                    "synth_supply": { "$last": "$synth_supply" },
                    "synth_units": { "$last": "$synth_units" },
                    "units": { "$last": "$units" }
                }
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
                },
                "asset_depth": 1,
                "asset_price": 1,
                "asset_price_usd": 1,
                "liquidity_units": 1,
                "luvi": 1,
                "members_count": 1,
                "rune_depth": 1,
                "synth_supply": 1,
                "synth_units": 1,
                "units": 1,
                "pool" : 1
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
                }
                Err(e) => eprintln!("Error fetching document: {:?}", e),
            }
        }
        // println!("{:?}",query_response);
        Ok(query_response)
    }
}
