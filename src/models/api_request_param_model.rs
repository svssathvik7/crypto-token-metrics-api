use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct QueryParams{
    pub pool : Option<String>,
    pub interval : Option<String>,
    pub count : Option<u32>,
    pub to : Option<u64>,
    pub from : Option<u64>,
    pub page : Option<u64>,
    pub sort_by : Option<String>,
    pub sort_order : Option<i8>,
    pub limit : Option<i8> 
}

pub fn validate_query(query: &QueryParams) -> Result<(), HttpResponse> {
    // Check start_time and end_time
    if let (Some(start), Some(end)) = (query.from, query.to) {
        if start >= end {
            return Err(HttpResponse::BadRequest().json("start_time must be less than end_time."));
        }
    }

    // Validate page
    let page = query.page.unwrap_or(1).max(1);
    if page < 1 {
        return Err(HttpResponse::BadRequest().json("page must be a positive integer."));
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