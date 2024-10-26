use chrono::Utc;
use futures_util::StreamExt;
use mongodb::bson::{doc, Document};

use crate::{
    models::{api_request_param_model::QueryParams, custom_error_model::CustomError},
    services::db::DataBase,
    utils::{db_helper_utils::{build_query_sort_skip, get_seconds_per_interval}, parser_utils::subtract_bson_values},
};

impl DataBase {
    // /depths
    pub async fn get_depth_price_history_api(
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
    
        let seconds_per_interval =
            get_seconds_per_interval(interval.as_ref().unwrap_or(&"hour".to_string()).as_str());
    
        if let Some(pool) = pool {
            // Check pool constraint
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
                self.get_max_end_time(&self.depth_history)
                    .await
                    .unwrap_or(Utc::now().timestamp())
            };
            let count = count.unwrap_or(400) as i64;
            let queried_interval_duration = seconds_per_interval as i64;
            query.insert(
                "start_time",
                doc! {"$gte": calc_start - (count * queried_interval_duration) as i64},
            );
        }
        let (query_part, sort_filter, skip_size, limit) =
            build_query_sort_skip(to, sort_by, sort_order, page, limit, count).await;
    
        query.extend(query_part.clone());
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
        let first = query_response.first().unwrap();
        let last = query_response.last().unwrap();
        let response = doc! {
            "meta":{
                "endAssetDepth": last.get("assetDepth"),
                "endLPUnits": last.get("units"),
                "endMemberCount": last.get("membersCount"),
                "endRuneDepth": last.get("runeDepth"),
                "endSynthUnits": last.get("synthUnits"),
                "endTime": last.get("endTime"),
                "luviIncrease": subtract_bson_values(last.get("luvi").unwrap(),first.get("luvi").unwrap()),
                "priceShiftLoss": subtract_bson_values(first.get("assetPrice").unwrap(), last.get("assetPrice").unwrap()),
                "startAssetDepth": first.get("assetDepth"),
                "startLPUnits": first.get("units"),
                "startMemberCount": first.get("membersCount"),
                "startRuneDepth": first.get("runeDepth"),
                "startSynthUnits": first.get("synthUnits"),
                "startTime": first.get("startTime")
            },
            "intervals": query_response
        };
        Ok(response)
    }    
}
