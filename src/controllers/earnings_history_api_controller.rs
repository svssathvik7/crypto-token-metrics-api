use chrono::Utc;
use futures_util::StreamExt;
use mongodb::bson::{doc, Document};

use crate::{models::{api_request_param_model::QueryParams, custom_error_model::CustomError}, services::db::DataBase, utils::db_helper_utils::{build_query_sort_skip, get_seconds_per_interval}};

impl DataBase{
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

        // as per midgard api if from is not specified the from has to be fixed back relative to either current timestamp or "to" timestamp (if given) or w.r.t the latest record in the collection
        if let Some(from) = from {
            query.insert("start_time", doc! { "$gte": from as i64 });
        }
        else {
            let calc_start = if let Some(to) = to {
                to as i64
            } else {
                self.get_max_end_time(&self.earnings)
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
                doc! {"$gte": calc_start - (count * queried_interval_duration)},
            );
        }
    
        // common query building code part has been moved to a helper function
        let (query_part, sort_filter, skip_size, limit) = build_query_sort_skip(to, sort_by, sort_order, page, limit, count).await;

        // update the actual query with the query_part from builder
        query.extend(query_part.clone());
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
                    "saverEarning": {"$last" : "$saver_earning"},
                    "earning": {"$last" : "$earning"},
                    "rewards": {"$last" : "$rewards"},
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
                "assetLiquidityFees": 1,
                "earnings": 1,
                "pool": 1,
                "rewards": 1,
                "runeLiquidityFees": 1,
                "saverEarnings": 1,
                "totalLiquidityFeesRune": 1,
                "earnings_summary": {
                    "avg_node_count": 1,
                    "block_rewards": 1,
                    "bonding_earnings": 1,
                    "earnings": 1,
                    "end_time": 1,
                    "liquidity_earnings": 1,
                    "start_time": 1,
                    "rune_price_usd": 1
                }
            }},
            doc! { "$sort": sort_filter },
            doc! { "$skip": skip_size as i64 },
            doc! { "$limit": limit as i64 }
        ];
    
        let mut cursor = self.earnings.aggregate(pipeline).await?;
        let mut query_response = Vec::new();
        let mut earnings_summary = None;
        // average of all the earning summary blocks is the meta for earnings
        let mut meta = doc! {
            "avg_node_count": 0,
            "block_rewards": 0.0,
            "bonding_earnings": 0.0,
            "earnings": 0.0,
            "liquidity_earnings": 0.0,
            "rune_price_usd": 0.0,
        };
        let mut count = 0;
    
        while let Some(result) = cursor.next().await {
            match result {
                Ok(mut record) => {
                    if let Some(earnings) = record.get("earnings_summary") {
                        earnings_summary = Some(earnings.clone());
                        // Accumulate sums for meta calculations
                        if let Some(earnings_doc) = earnings.as_document() {
                            for field in ["avg_node_count".to_string(), "block_rewards".to_string(), "bonding_earnings".to_string(), "earnings".to_string(), "liquidity_earnings".to_string(), "rune_price_usd".to_string().to_string()].iter() {
                                if let Some(value) = earnings_doc.get_i64(field).ok() {
                                    meta.insert(field, meta.get_i64(field).unwrap_or(0) + value);
                                } else if let Some(value) = earnings_doc.get_f64(field).ok() {
                                    meta.insert(field, meta.get_f64(field).unwrap_or(0.0) + value);
                                }
                            }
                            count += 1;
                        }
                    }
                    record.remove("earnings_summary");
                    query_response.push(record);
                }
                Err(e) => eprintln!("Error fetching document: {:?}", e),
            }
        }
    
        // Calculate averages if count is 1 sum itself is the avg
        if count > 1 {
            for field in ["avg_node_count".to_string(), "block_rewards".to_string(), "bonding_earnings".to_string(), "earnings".to_string(), "liquidity_earnings".to_string(), "rune_price_usd".to_string()].iter() {
                if let Some(sum) = meta.get_i64(field).ok() {
                    meta.insert(field, sum / count);
                } else if let Some(sum) = meta.get_f64(field).ok() {
                    meta.insert(field, sum / count as f64);
                }
            }
        }
    
        // since earnings route has been scaled for all pools with 7L+ records, we summarize the total reponses instead of finding individual earnings summaries like in midgard
        let result = doc! {
            "meta": meta,
            "intervals": {
                "earnings_summary": earnings_summary,
                "pools": query_response
            }
        };

        Ok(result)
    }    
}