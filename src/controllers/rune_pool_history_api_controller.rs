use chrono::Utc;
use futures_util::StreamExt;
use mongodb::bson::{doc, Document};

use crate::{models::{api_request_param_model::QueryParams, custom_error_model::CustomError}, services::{db::DataBase, rune_pool_service::{ApiResponse, Meta}}, utils::db_helper_utils::{build_query_sort_skip, get_seconds_per_interval}};

impl DataBase{
    pub async fn get_rune_pool_history_api(
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

        if let Some(_pool) = pool {
            return Err(CustomError::InvalidInput(
                "Invalid parameter pool!".to_string(),
            ));
        }
        
        // as per midgard api if from is not specified the from has to be fixed back relative to either current timestamp or "to" timestamp (if given) or w.r.t the latest record in the collection
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
        
        let (query_part, sort_filter, skip_size, limit) = build_query_sort_skip(to, sort_by, sort_order, page, limit, count).await;
        query.extend(query_part.clone());
        println!("{}",query);
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
        let first = query_response.first().unwrap();
        let last = query_response.last().unwrap();
        let response = doc! {
            "meta": {
                "endCount" : last.get("count").unwrap().to_string(),
                "endTime" : last.get("endTime").unwrap().to_string(),
                "endUnits" : last.get("units").unwrap().to_string(),
                "startCount" : first.get("count").unwrap().to_string(),
                "startTime" : first.get("startTime").unwrap().to_string(),
                "startUnits" : first.get("units").unwrap().to_string()
            },
            "intervals": query_response
        };
        // println!("{:?}",query_response);
        Ok(response)
    }
}