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

        let seconds_per_interval = get_seconds_per_interval(interval.as_ref().unwrap_or(&"hour".to_string()).as_str());

        
        if let Some(pool) = pool {
            // for now due to volume and computation constraints our depth history is confined to only BTC.BTC pool
            if pool != "BTC.BTC" {
                return Err(CustomError::InvalidInput(
                    "Depth and price history for only BTC.BTC available".to_string(),
                ));
            }
            query.insert("pool", pool);
        }

        if let Some(from) = from {
            query.insert("start_time", doc! { "$gte": from as i64 });
        } else {
            let calc_start = if let Some(to) = to {
                to as i64
            } else {
                get_max_start_time_of_collection(&self.depth_history).await.unwrap_or(Utc::now().timestamp())
            };
            let count = count.unwrap_or(400) as i64;
            let queried_interval_duration = seconds_per_interval as i64;
            query.insert(
                "start_time",
                doc! {"$gte": calc_start-(count*queried_interval_duration) as i64},
            );
        }
        if let Some(from) = from {
            query.insert("start_time", doc! { "$gte": from as i64 });
        } else {
            let calc_start = if let Some(to) = to {
                to as i64
            } else {
                get_max_start_time_of_collection(&self.depth_history).await.unwrap_or(Utc::now().timestamp())
            };
            let count = count.unwrap_or(400) as i64;
            let queried_interval_duration = seconds_per_interval as i64;
            query.insert(
                "start_time",
                doc! {"$gte": calc_start-(count*queried_interval_duration) as i64},
            );
        }

        let (query, sort_filter, skip_size, limit) = build_query_sort_skip(to, sort_by, sort_order, page, limit, count).await;
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
                    "assetDepth": { "$last": "$asset_depth" },
                    "assetPrice": { "$last": "$asset_price" },
                    "assetPriceUSD": { "$last": "$asset_price_usd" },
                    "liquidityUnits": { "$last": "$liquidity_units" },
                    "luvi": { "$last": "$luvi" },
                    "membersCount": { "$last": "$members_count" },
                    "runeDepth": { "$last": "$rune_depth" },
                    "synthSupply": { "$last": "$synth_supply" },
                    "synthUnits": { "$last": "$synth_units" },
                    "units": { "$last": "$units" },
                    "pool" : {"$last" : "$pool"}
                }
            },
            doc! { "$project": {
                "_id": 0,
                "pool" : 1,
                "startTime": {
                    "$subtract": [ "$_id.interval_start", { "$mod": [ "$_id.interval_start", seconds_per_interval ] }]
                },
                "endTime": {
                    "$add": [
                        { "$subtract": [ "$_id.interval_start", { "$mod": [ "$_id.interval_start", seconds_per_interval ] }] },
                        seconds_per_interval
                    ]
                },
                "assetDepth": 1,
                "assetPrice": 1,
                "assetPriceUSD": 1,
                "liquidityUnits": 1,
                "luvi": 1,
                "membersCount": 1,
                "runeDepth": 1,
                "synthSupply": 1,
                "synthUnits": 1,
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

        let seconds_per_interval = get_seconds_per_interval(interval.as_ref().unwrap_or(&"hour".to_string()).as_str());


        if let Some(pool) = pool {
            // for now due to volume and computation constraints our swap history is confined to only BTC.BTC pool
            if pool != "BTC.BTC" {
                return Err(CustomError::InvalidInput(
                    "Swap history for only BTC.BTC available".to_string(),
                ));
            }
            query.insert("pool", pool);
        }


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

        let (query, sort_filter, skip_size, limit) = build_query_sort_skip(to, sort_by, sort_order, page, limit, count).await;

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
                    "averageSlip": { "$last": "$average_slip" },
                    "fromTradeAverageSlip": { "$last": "$from_trade_average_slip" },
                    "fromTradeCount": { "$last": "$from_trade_count" },
                    "fromTradeFees": { "$last": "$from_trade_fees" },
                    "fromTradeVolume": { "$last": "$from_trade_volume" },
                    "fromTradeVolumeUSD": { "$last": "$from_trade_volume_usd" },
                    "runePriceUSD": { "$last": "$rune_price_usd" },
                    "synthMintAverageSlip": { "$last": "$synth_mint_average_slip" },
                    "synthMintCount": { "$last": "$synth_mint_count" },
                    "synthMintFees": { "$last": "$synth_mint_fees" },
                    "synthMintVolume": { "$last": "$synth_mint_volume" },
                    "synthMintVolumeUSD": { "$last": "$synth_mint_volume_usd" },
                    "synthRedeemAverageSlip": { "$last": "$synth_redeem_average_slip" },
                    "synthRedeemCount": { "$last": "$synth_redeem_count" },
                    "synthRedeemFees": { "$last": "$synth_redeem_fees" },
                    "synthRedeemVolume": { "$last": "$synth_redeem_volume" },
                    "synthRedeemVolumeUSD": { "$last": "$synth_redeem_volume_usd" },
                    "toAssetAverageSlip": { "$last": "$to_asset_average_slip" },
                    "toAssetCount": { "$last": "$to_asset_count" },
                    "toAssetFees": { "$last": "$to_asset_fees" },
                    "toAssetVolume": { "$last": "$to_asset_volume" },
                    "toAssetVolumeUSD": { "$last": "$to_asset_volume_usd" },
                    "toRuneAverageSlip": { "$last": "$to_rune_average_slip" },
                    "toRuneCount": { "$last": "$to_rune_count" },
                    "toRuneFees": { "$last": "$to_rune_fees" },
                    "toRuneVolume": { "$last": "$to_rune_volume" },
                    "toRuneVolumeUSD": { "$last": "$to_rune_volume_usd" },
                    "toTradeAverageSlip": { "$last": "$to_trade_average_slip" },
                    "toTradeCount": { "$last": "$to_trade_count" },
                    "toTradeFees": { "$last": "$to_trade_fees" },
                    "toTradeVolume": { "$last": "$to_trade_volume" },
                    "toTradeVolumeUSD": { "$last": "$to_trade_volume_usd" },
                    "totalCount": { "$last": "$total_count" },
                    "totalFees": { "$last": "$total_fees" },
                    "totalVolume": { "$last": "$total_volume" },
                    "totalVolumeUSD": { "$last": "$total_volume_usd" }
                }
            },
            doc! {
                "$project": {
                "_id": 0,
                "startTime": {
                    "$subtract": [ "$_id.interval_start", { "$mod": [ "$_id.interval_start", seconds_per_interval ] }]
                },
                "endTime": {
                    "$add": [
                        { "$subtract": [ "$_id.interval_start", { "$mod": [ "$_id.interval_start", seconds_per_interval ] }] },
                        seconds_per_interval
                    ]
                },
                "averageSlip": 1,
                "fromTradeAverageSlip": 1,
                "fromTradeCount": 1,
                "fromTradeFees": 1,
                "fromTradeVolume": 1,
                "fromTradeVolumeUSD": 1,
                "runePriceUSD": 1,
                "synthMintAverageSlip": 1,
                "synthMintCount": 1,
                "synthMintFees": 1,
                "synthMintVolume": 1,
                "synthMintVolumeUSD": 1,
                "synthRedeemAverageSlip": 1,
                "synthRedeemCount": 1,
                "synthRedeemFees": 1,
                "synthRedeemVolume": 1,
                "synthRedeemVolumeUSD": 1,
                "toAssetAverageSlip": 1,
                "toAssetCount": 1,
                "toAssetFees": 1,
                "toAssetVolume": 1,
                "toAssetVolumeUSD": 1,
                "toRuneAverageSlip": 1,
                "toRuneCount": 1,
                "toRuneFees": 1,
                "toRuneVolume": 1,
                "toRuneVolumeUSD": 1,
                "toTradeAverageSlip": 1,
                "toTradeCount": 1,
                "toTradeFees": 1,
                "toTradeVolume": 1,
                "toTradeVolumeUSD": 1,
                "totalCount": 1,
                "totalFees": 1,
                "totalVolume": 1,
                "totalVolumeUSD": 1
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

        let seconds_per_interval = get_seconds_per_interval(interval.as_ref().unwrap_or(&"hour".to_string()).as_str());

        if let Some(_pool) = pool {
            return Err(CustomError::InvalidInput(
                "Invalid parameter pool!".to_string(),
            ));
        }

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

        let (query, sort_filter, skip_size, limit) = build_query_sort_skip(to, sort_by, sort_order, page, limit, count).await;

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
                "startTime": {
                    "$subtract": [ "$_id.interval_start", { "$mod": [ "$_id.interval_start", seconds_per_interval ] }]
                },
                "endTime": {
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

        let seconds_per_interval = get_seconds_per_interval(interval.as_ref().unwrap_or(&"hour".to_string()).as_str());

        if let Some(pool) = pool {
            query.insert("pool", pool);
        }

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

        let (query, sort_filter, skip_size, limit) = build_query_sort_skip(to, sort_by, sort_order, page, limit, count).await;

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
                    "assetLiquidityFees": { "$last": "$asset_liquidity_fees" },
                    "runeLiquidityFees": { "$last": "$rune_liquidity_fees" },
                    "totalLiquidityFeesRune": { "$last": "$total_liquidity_fees_rune" },
                    "saverEarning" : {"$last" : "$saver_earning"},
                    "earning" : {"$last" : "$earning"},
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
                "assetLiquidityFees" : 1,
                "earnings" : 1,
                "pool" : 1,
                "rewards" : 1,
                "runeLiquidityFees" : 1,
                "saverEarnings" : 1,
                "totalLiquidityFeesRune" : 1,
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
