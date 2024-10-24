use std::env;

use chrono::Utc;
use dotenv::dotenv;
use futures_util::StreamExt;
use mongodb::bson::Document;
use mongodb::{bson::doc, Client, Collection};
use serde::de::DeserializeOwned;

use crate::models::api_request_param_model::QueryParams;
use crate::models::custom_error_model::CustomError;
use crate::models::{
    depth_history_model::PoolDepthPriceHistory,
    earning_history_model::{PoolEarningHistory, PoolEarningSummary},
    rune_pool_model::RunePool,
    swap_history_model::SwapHistory,
};


const api_start_time:i64 = 1_647_913_096;
pub fn get_seconds_per_interval(interval: &str) -> i32 {
    match interval {
        "hour" => 3600,
        "day" => 86_400,
        "week" => 604_800,
        "month" => 2_678_400,
        "quarter" => 7_948_800,
        "year" => 31_622_400,
        _ => 3_600,
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

    // helper function
    pub async fn get_max_start_time_of_collection<T>(
        &self,
        collection: &Collection<T>,
    ) -> Result<i64, CustomError>
    where
        T: DeserializeOwned + Unpin + Send + Sync,
    {
        let pipeline = vec![
            doc! {
                "$group": {
                    "_id": null,
                    "max_start_time": { "$max": "$start_time" }
                }
            },
            doc! { "$limit": 1 },
        ];
    
        let mut cursor = collection.aggregate(pipeline).await?;
    
        if let Some(result) = cursor.next().await {
            match result {
                Ok(doc) => {
                    let max_start_time = doc.get_i64("max_start_time").unwrap_or(0);
                    Ok(max_start_time)
                }
                Err(e) => {
                    eprintln!("Failed to fetch max start_time: {}", e);
                    Err(CustomError::DatabaseError(e.to_string()))
                }
            }
        } else {
            Ok(api_start_time)
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
            limit,
        } = params;

        let seconds_per_interval = get_seconds_per_interval(interval.as_ref().unwrap().as_str());

        let page = page.unwrap_or(1);

        if let Some(pool) = pool {
            // for now due to volume and computation constraints our depth history is confined to only BTC.BTC pool
            if pool != "BTC.BTC" {
                return Err(CustomError::InvalidInput(
                    "Depth and price history for only BTC.BTC available".to_string(),
                ));
            }
            query.insert("pool", pool);
        }

        let limit: i16 = if let Some(limit) = limit { limit } else { if let Some(count) = count {count as i16} else {400} };

        if let Some(from) = from {
            query.insert("start_time", doc! { "$gte": from as i64 });
        } else {
            let calc_start = if let Some(to) = to {
                to as i64
            } else {
                self.get_max_start_time_of_collection(&self.depth_history).await.unwrap_or(Utc::now().timestamp())
            };
            let count = count.unwrap_or(400) as i64;
            let queried_interval_duration = seconds_per_interval as i64;
            query.insert(
                "start_time",
                doc! {"$gte": calc_start-(count*queried_interval_duration) as i64},
            );
        }

        if let Some(to) = to {
            query.insert("end_time", doc! { "$lte": to as i64 });
        }

        let sort_filter = if let Some(sort_by) = sort_by {
            if let Some(sort_order) = sort_order {
                let sort_order = if sort_order == 1 { 1 } else { -1 };
                doc! { sort_by: sort_order }
            } else {
                doc! { sort_by: 1 }
            }
        } else {
            doc! { "end_time": -1 }
        };
        let skip_size = (page - 1) * (limit as u64);
        println!("{}", query);
        
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

    // /swaps
    pub async fn get_swaps_history_api(
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
            limit,
        } = params;

        let seconds_per_interval = get_seconds_per_interval(interval.as_ref().unwrap().as_str());

        let page = page.unwrap_or(1);

        if let Some(pool) = pool {
            // for now due to volume and computation constraints our swap history is confined to only BTC.BTC pool
            if pool != "BTC.BTC" {
                return Err(CustomError::InvalidInput(
                    "Swap history for only BTC.BTC available".to_string(),
                ));
            }
            query.insert("pool", pool);
        }

        let limit: i16 = if let Some(limit) = limit { limit } else { if let Some(count) = count {count as i16} else {400} };

        if let Some(from) = from {
            query.insert("start_time", doc! { "$gte": from as i64 });
        } else {
            let calc_start = if let Some(to) = to {
                to as i64
            } else {
                Utc::now().timestamp() as i64
            };
            let count = count.unwrap_or(400) as i64;
            let queried_interval_duration = seconds_per_interval as i64;
            query.insert(
                "start_time",
                doc! {"$gte": calc_start-(count*queried_interval_duration) as i64},
            );
        }

        if let Some(to) = to {
            query.insert("end_time", doc! { "$lte": to as i64 });
        }

        let sort_filter = if let Some(sort_by) = sort_by {
            if let Some(sort_order) = sort_order {
                let sort_order = if sort_order == 1 { 1 } else { -1 };
                doc! { sort_by: sort_order }
            } else {
                doc! { sort_by: 1 }
            }
        } else {
            doc! { "end_time": -1 }
        };
        let skip_size = (page - 1) * (limit as u64);
        let interval = interval.unwrap_or(String::from("hour"));
        println!("{}", query);
        if interval == "hour" {
            println!("{}", query);
            let mut cursor = self
                .swap_history
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
                    "pool": { "$last": "$pool" },
                    "average_slip": { "$last": "$average_slip" },
                    "from_trade_average_slip": { "$last": "$from_trade_average_slip" },
                    "from_trade_count": { "$last": "$from_trade_count" },
                    "from_trade_fees": { "$last": "$from_trade_fees" },
                    "from_trade_volume": { "$last": "$from_trade_volume" },
                    "from_trade_volume_usd": { "$last": "$from_trade_volume_usd" },
                    "rune_price_usd": { "$last": "$rune_price_usd" },
                    "synth_mint_average_slip": { "$last": "$synth_mint_average_slip" },
                    "synth_mint_count": { "$last": "$synth_mint_count" },
                    "synth_mint_fees": { "$last": "$synth_mint_fees" },
                    "synth_mint_volume": { "$last": "$synth_mint_volume" },
                    "synth_mint_volume_usd": { "$last": "$synth_mint_volume_usd" },
                    "synth_redeem_average_slip": { "$last": "$synth_redeem_average_slip" },
                    "synth_redeem_count": { "$last": "$synth_redeem_count" },
                    "synth_redeem_fees": { "$last": "$synth_redeem_fees" },
                    "synth_redeem_volume": { "$last": "$synth_redeem_volume" },
                    "synth_redeem_volume_usd": { "$last": "$synth_redeem_volume_usd" },
                    "to_asset_average_slip": { "$last": "$to_asset_average_slip" },
                    "to_asset_count": { "$last": "$to_asset_count" },
                    "to_asset_fees": { "$last": "$to_asset_fees" },
                    "to_asset_volume": { "$last": "$to_asset_volume" },
                    "to_asset_volume_usd": { "$last": "$to_asset_volume_usd" },
                    "to_rune_average_slip": { "$last": "$to_rune_average_slip" },
                    "to_rune_count": { "$last": "$to_rune_count" },
                    "to_rune_fees": { "$last": "$to_rune_fees" },
                    "to_rune_volume": { "$last": "$to_rune_volume" },
                    "to_rune_volume_usd": { "$last": "$to_rune_volume_usd" },
                    "to_trade_average_slip": { "$last": "$to_trade_average_slip" },
                    "to_trade_count": { "$last": "$to_trade_count" },
                    "to_trade_fees": { "$last": "$to_trade_fees" },
                    "to_trade_volume": { "$last": "$to_trade_volume" },
                    "to_trade_volume_usd": { "$last": "$to_trade_volume_usd" },
                    "total_count": { "$last": "$total_count" },
                    "total_fees": { "$last": "$total_fees" },
                    "total_volume": { "$last": "$total_volume" },
                    "total_volume_usd": { "$last": "$total_volume_usd" }
                }
            },
            doc! {
                "$project": {
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
                "pool": 1,
                "average_slip": 1,
                "from_trade_average_slip": 1,
                "from_trade_count": 1,
                "from_trade_fees": 1,
                "from_trade_volume": 1,
                "from_trade_volume_usd": 1,
                "rune_price_usd": 1,
                "synth_mint_average_slip": 1,
                "synth_mint_count": 1,
                "synth_mint_fees": 1,
                "synth_mint_volume": 1,
                "synth_mint_volume_usd": 1,
                "synth_redeem_average_slip": 1,
                "synth_redeem_count": 1,
                "synth_redeem_fees": 1,
                "synth_redeem_volume": 1,
                "synth_redeem_volume_usd": 1,
                "to_asset_average_slip": 1,
                "to_asset_count": 1,
                "to_asset_fees": 1,
                "to_asset_volume": 1,
                "to_asset_volume_usd": 1,
                "to_rune_average_slip": 1,
                "to_rune_count": 1,
                "to_rune_fees": 1,
                "to_rune_volume": 1,
                "to_rune_volume_usd": 1,
                "to_trade_average_slip": 1,
                "to_trade_count": 1,
                "to_trade_fees": 1,
                "to_trade_volume": 1,
                "to_trade_volume_usd": 1,
                "total_count": 1,
                "total_fees": 1,
                "total_volume": 1,
                "total_volume_usd": 1
            }},
            doc! { "$sort": sort_filter },
            doc! { "$skip": skip_size as i64 },
            doc! { "$limit": limit as i64 },
        ];

        let mut cursor = self.swap_history.aggregate(pipeline).await?;
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

    pub async fn get_rune_pool_history_api(
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
            limit,
        } = params;

        let seconds_per_interval = match interval.as_ref().unwrap().as_str() {
            "hour" => 3600,
            "day" => 86_400,
            "week" => 604_800,
            "month" => 2_678_400,
            "quarter" => 7_948_800,
            "year" => 31_622_400,
            _ => 3_600,
        };

        let page = page.unwrap_or(1);

        if let Some(_pool) = pool {
            return Err(CustomError::InvalidInput(
                "Invalid parameter pool!".to_string(),
            ));
        }

        let limit: i16 = if let Some(limit) = limit { limit } else { if let Some(count) = count {count as i16} else {400} };

        if let Some(from) = from {
            query.insert("start_time", doc! { "$gte": from as i64 });
        } else {
            let calc_start = if let Some(to) = to {
                to as i64
            } else {
                Utc::now().timestamp() as i64
            };
            let count = count.unwrap_or(400) as i64;
            let queried_interval_duration = seconds_per_interval as i64;
            query.insert(
                "start_time",
                doc! {"$gte": calc_start-(count*queried_interval_duration) as i64},
            );
        }

        if let Some(to) = to {
            query.insert("end_time", doc! { "$lte": to as i64 });
        }

        let sort_filter = if let Some(sort_by) = sort_by {
            if let Some(sort_order) = sort_order {
                let sort_order = if sort_order == 1 { 1 } else { -1 };
                doc! { sort_by: sort_order }
            } else {
                doc! { sort_by: 1 }
            }
        } else {
            doc! { "end_time": -1 }
        };
        let skip_size = (page - 1) * (limit as u64);
        let interval = interval.unwrap_or(String::from("hour"));
        println!("{}", query);
        if interval == "hour" {
            println!("{}", query);
            let mut cursor = self
                .rune_pool_history
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
                    "count": { "$last": "$count" },
                    "units": { "$last": "$units" },
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
                "count" : 1,
                "units" : 1
            }},
            doc! { "$sort": sort_filter },
            doc! { "$skip": skip_size as i64 },
            doc! { "$limit": limit as i64 },
        ];

        let mut cursor = self.rune_pool_history.aggregate(pipeline).await?;
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

    pub async fn get_pool_earnings_history_api(
        &self,
        params: QueryParams,
    ) -> Result<Document, CustomError> {
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
            limit,
        } = params;

        let seconds_per_interval = match interval.as_ref().unwrap_or(&"hour".to_string()).as_str() {
            "hour" => 3600,
            "day" => 86_400,
            "week" => 604_800,
            "month" => 2_678_400,
            "quarter" => 7_948_800,
            "year" => 31_622_400,
            _ => 3_600,
        };

        let page = page.unwrap_or(1);

        if let Some(pool) = pool {
            query.insert("pool", pool);
        }

        let limit: i16 = if let Some(limit) = limit { limit } else { if let Some(count) = count {count as i16} else {400} };

        if let Some(from) = from {
            query.insert("start_time", doc! { "$gte": from as i64 });
        } else {
            let calc_start = if let Some(to) = to {
                to as i64
            } else {
                Utc::now().timestamp() as i64
            };
            let count = count.unwrap_or(400) as i64;
            let queried_interval_duration = seconds_per_interval as i64;
            query.insert(
                "start_time",
                doc! {"$gte": calc_start-(count*queried_interval_duration) as i64},
            );
        }

        if let Some(to) = to {
            query.insert("end_time", doc! { "$lte": to as i64 });
        }

        let sort_filter = if let Some(sort_by) = sort_by {
            if let Some(sort_order) = sort_order {
                let sort_order = if sort_order == 1 { 1 } else { -1 };
                doc! { sort_by: sort_order }
            } else {
                doc! { sort_by: 1 }
            }
        } else {
            doc! { "end_time": -1 }
        };
        let skip_size = (page - 1) * (limit as u64);
        println!("{}", query);
        
        let pipeline = vec![
            doc! { "$match": query },
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
                    "pool": { "$last": "$pool" },
                    "asset_liquidity_fees": { "$last": "$asset_liquidity_fees" },
                    "rune_liquidity_fees": { "$last": "$rune_liquidity_fees" },
                    "total_liquidity_fees_rune": { "$last": "$total_liquidity_fees_rune" },
                    "saver_earnings" : {"$last" : "$saver_earnings"},
                    "earnings" : {"$last" : "$earnings"},
                    "rewards" : {"$last" : "$rewards"},
                    "earnings_summary": { "$last": "$earnings_summary" }
                }
            },
            doc! { "$lookup": {
                "from": "earnings_summary",
                "localField": "earnings_summary",
                "foreignField": "_id",
                "as": "earnings_summary"
            }},
            doc! { "$unwind": {"path" : "$earnings_summary"} },
            doc! { "$project": {
                "_id": 0,
                "asset_liquidity_fees" : 1,
                "earnings" : 1,
                "pool" : 1,
                "rewards" : 1,
                "rune_liquidity_fees" : 1,
                "saver_earnings" : 1,
                "total_liquidity_fees_rune" : 1,
                "earnings_summary" : 1,
            }},
            doc! { "$sort": sort_filter },
            doc! { "$skip": skip_size as i64 },
            doc! { "$limit": limit as i64 },
        ];

        let mut cursor = self.earnings.aggregate(pipeline).await?;
        let mut query_response = Vec::new();
        let mut earnings_summary = None;

        while let Some(result) = cursor.next().await {
            match result {
                Ok(mut record) => {
                    if let Some(earnings) = record.get("earnings_summary") {
                        earnings_summary = Some(earnings.clone());
                    }

                    record.remove("earnings_summary");

                    query_response.push(record);
                }
                Err(e) => eprintln!("Error fetching document: {:?}", e),
            }
        }

        let result = doc! {
            "earnings_summary": earnings_summary,
            "pools": query_response
        };

        Ok(result)
    }
}
