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

        if let Some(from) = from {
            query.insert("start_time", doc! { "$gte": from as i64 });
        } else {
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