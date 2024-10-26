use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::custom_error_model::CustomError;

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

pub fn validate_query(query: &QueryParams) -> Result<(), CustomError> {
    // Check start_time and end_time
    if let (Some(start), Some(end)) = (query.from, query.to) {
        if start >= end {
            return Err(CustomError::InvalidInput("start_time must be less than end_time.".to_string()));
        }
    }
    // validations if serde fails validating incorrect input
    // Validate page
    if let Some(page) = query.page {
        if page < 1{
            return Err(CustomError::InvalidInput("page must be positive number".to_string()));
        }
    }

    if let Some(count) = query.count{
        if count < 1 || count > 400{
            return Err(CustomError::InvalidInput("Count has to be [1..400]".to_string()));
        }
    }

    if let Some(limit) = query.limit{
        if limit < 1 || limit > 400{
            return Err(CustomError::InvalidInput("Limit has to be [1..400]".to_string()));
        }
    }

    // Validate interval
    let valid_interval = vec!["hour", "day", "week", "month", "quarter", "year"];
    if let Some(ref interval) = query.interval {
        if !valid_interval.contains(&interval.as_str()) {
            return Err(CustomError::InvalidInput(format!("Interval must be in {:?}",valid_interval)));
        }
    }

    Ok(())
}