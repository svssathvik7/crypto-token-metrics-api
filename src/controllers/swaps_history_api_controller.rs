use chrono::Utc;
use futures_util::StreamExt;
use mongodb::bson::{doc, Document};

use crate::{models::{api_request_param_model::QueryParams, custom_error_model::CustomError}, services::db::DataBase, utils::db_helper_utils::{build_query_sort_skip, get_seconds_per_interval}};

// /swaps
impl DataBase{
    pub async fn get_swaps_history_api(
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
            // for now due to volume and computation constraints our swap history is confined to only BTC.BTC pool
            if pool != "BTC.BTC" {
                return Err(CustomError::InvalidInput(
                    "Swap history for only BTC.BTC available".to_string(),
                ));
            }
            query.insert("pool", pool);
        }
    
        // as per midgard api if from is not specified the from has to be fixed back relative to either current timestamp or "to" timestamp (if given) or w.r.t the latest record in the collection
        if let Some(from) = from {
            query.insert("start_time", doc! { "$gte": from as i64 });
        } else {
            let calc_start = if let Some(to) = to {
                to as i64
            } else {
                self.get_max_end_time(&self.swap_history)
                    .await
                    .unwrap_or(Utc::now().timestamp())
            };
            let count = count.unwrap_or(400) as i64;
            let queried_interval_duration = seconds_per_interval as i64;
            println!(
                "{} {} {}",
                calc_start,
                count * queried_interval_duration,
                calc_start - (count * queried_interval_duration)
            );
            query.insert(
                "start_time",
                doc! {"$gte": calc_start-(count*queried_interval_duration) as i64},
            );
        }
        // common query building code part has been moved to a helper function
        let (query_part, sort_filter, skip_size, limit) = build_query_sort_skip(to, sort_by, sort_order, page, limit, count).await;
        // update the actual query with the query_part from builder
        query.extend(query_part.clone());
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
        // no graceful error handling since any unwrap_or would result in wrong meta results
        let first = query_response.first().unwrap();
        let last = query_response.last().unwrap();
        let response = doc! {
            "meta": {
                "endTime": last.get("endTime"),
                "fromTradeAverageSlip": last.get("fromTradeAverageSlip"),
                "fromTradeCount": last.get("fromTradeCount"),
                "fromTradeFees": last.get("fromTradeFees"),
                "fromTradeVolume": last.get("fromTradeVolume"),
                "fromTradeVolumeUSD": last.get("fromTradeVolumeUSD"),
                "runePriceUSD": last.get("runePriceUSD"),
                "startTime": first.get("startTime"),
                "synthMintAverageSlip": last.get("synthRedeemAverageSlip"),
                "synthMintCount": last.get("synthMintCount"),
                "synthMintFees": last.get("synthMintFees"),
                "synthMintVolume": last.get("synthMintVolume"),
                "synthMintVolumeUSD": last.get("synthMintVolumeUSD"),
                "synthRedeemAverageSlip": last.get("synthRedeemAverageSlip"),
                "synthRedeemCount": last.get("synthRedeemCount"),
                "synthRedeemFees": last.get("synthRedeemFees"),
                "synthRedeemVolume": last.get("synthRedeemVolume"),
                "synthRedeemVolumeUSD": last.get("synthRedeemVolumeUSD"),
                "toAssetAverageSlip": last.get("toAssetAverageSlip"),
                "toAssetCount": last.get("toAssetCount"),
                "toAssetFees": last.get("toAssetFees"),
                "toAssetVolume": last.get("toAssetVolume"),
                "toAssetVolumeUSD": last.get("toAssetVolumeUSD"),
                "toRuneAverageSlip": last.get("toRuneAverageSlip"),
                "toRuneCount": last.get("toRuneCount"),
                "toRuneFees": last.get("toRuneFees"),
                "toRuneVolume": last.get("toRuneVolume"),
                "toRuneVolumeUSD": last.get("toRuneVolumeUSD"),
                "toTradeAverageSlip": last.get("toTradeAverageSlip"),
                "toTradeCount": last.get("toTradeCount"),
                "toTradeFees": last.get("toTradeFees"),
                "toTradeVolume": last.get("toTradeVolume"),
                "toTradeVolumeUSD": last.get("toTradeVolumeUSD"),
                "totalCount": last.get("totalCount"),
                "totalFees": last.get("totalFees"),
                "totalVolume": last.get("totalVolume"),
                "totalVolumeUSD": last.get("totalVolumeUSD")
            },
            "intervals": query_response
        };
        // println!("{:?}",query_response);
        Ok(response)
    }
}