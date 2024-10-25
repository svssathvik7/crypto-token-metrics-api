use futures_util::StreamExt;
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::Collection;
use serde::de::DeserializeOwned;

use crate::models::custom_error_model::CustomError;

use super::constants::API_START_TIME;
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

// helper function
pub async fn get_max_end_time_of_collection<T>(
    collection: &Collection<T>,
) -> Result<i64, CustomError>
where
    T: DeserializeOwned + Unpin + Send + Sync,
{
    let pipeline = vec![
        doc! {
            "$group": {
                "_id": null,
                "max_start_time": { "$max": "$end_time" }
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
                eprintln!("Failed to fetch max end_time: {}", e);
                Err(CustomError::DatabaseError(e.to_string()))
            }
        }
    } else {
        Ok(API_START_TIME)
    }
}

pub async fn build_query_sort_skip(
    to: Option<u64>,
    sort_by: Option<String>,
    sort_order: Option<i8>,
    page: Option<u64>,
    limit: Option<i16>,
    count: Option<u32>,
) -> (Document, Document, i64, i16) {
    // Default values
    let page = page.unwrap_or(1);

    let limit: i16 = limit.unwrap_or_else(|| count.unwrap_or(20) as i16);

    let mut query = Document::new();


    // Constructing the "end_time" filter
    if let Some(to) = to {
        query.insert("end_time", doc! { "$lte": to as i64 });
    }

    // Constructing the sort filter
    let sort_filter = if let Some(sort_by) = sort_by {
        let sort_order = sort_order.unwrap_or(1);
        doc! { sort_by: if sort_order >= 1 { 1 } else { -1 } }
    } else {
        doc! { "end_time": -1 }
    };

    // Calculating skip size
    let skip_size = (page - 1) * (limit as u64);

    (query, sort_filter, skip_size as i64, limit)
}
