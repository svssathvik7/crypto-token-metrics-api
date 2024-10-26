use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug,Serialize,Deserialize,ToSchema)]
pub struct QueryParams{
    #[schema(example = "BTC.BTC")]
    pub pool : Option<String>,
    #[schema(example = "day")]
    pub interval : Option<String>,
    #[schema(example="100")]
    pub count : Option<u32>,
    #[schema(example = 1653373410)]
    pub to : Option<u64>,
    #[schema(example = 1653373410)]
    pub from : Option<u64>,
    #[schema(example="2")]
    pub page : Option<u64>,
    #[schema(example="start_time")]
    pub sort_by : Option<String>,
    #[schema(example="-1")]
    pub sort_order : Option<i8>,
    #[schema(example="40")]
    pub limit : Option<i16> 
}

pub fn validate_query(query: &QueryParams) -> Result<(), HttpResponse> {
    // Check start_time and end_time
    if let (Some(start), Some(end)) = (query.from, query.to) {
        if start >= end {
            return Err(HttpResponse::BadRequest().json("start_time must be less than end_time."));
        }
    }

    // Validate page
    if let Some(page) = query.page {
        if page < 1{
            return Err(HttpResponse::BadRequest().json("page must be a positive integer."));
        }
    }

    // Validate interval
    let valid_interval = vec!["hour", "day", "week", "month", "quarter", "year"];
    if let Some(ref interval) = query.interval {
        if !valid_interval.contains(&interval.as_str()) {
            return Err(HttpResponse::BadRequest().json(format!(
                "interval must be one of: {:?}",
                valid_interval
            )));
        }
    }

    Ok(())
}