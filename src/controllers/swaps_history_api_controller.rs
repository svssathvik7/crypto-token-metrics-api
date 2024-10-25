use chrono::Utc;
use futures_util::StreamExt;
use mongodb::bson::{doc, Document};

use crate::{models::{api_request_param_model::QueryParams, custom_error_model::CustomError}, services::db::DataBase, utils::db_helper_utils::{build_query_sort_skip, get_seconds_per_interval}};

// /swaps
impl DataBase{
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
}