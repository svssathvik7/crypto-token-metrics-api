use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::{openapi::schema, ToSchema};

use crate::services::rune_pool_service::Interval;
use std::error::Error as stdError;

fn generate_error_text(field_name:&str) -> String{
    format!("Incorrect {} format",field_name)
}

#[derive(Debug,Serialize,Deserialize, ToSchema)]
pub struct RunePool{
    #[schema(value_type = String, example = "60d5ec49a1c4b5048c0e5c70")]
    pub _id : ObjectId,
    #[schema(example=391)]
    pub count : f64,
    #[schema(example=1727114400)]
    pub end_time : i64,
    #[schema(example=1727110800)]
    pub start_time : i64,
    #[schema(example=400984606438789.0)]
    pub units : f64
}

impl TryFrom<Interval> for RunePool{
    type Error = Box<dyn stdError>;
    fn try_from(interval: Interval) -> Result<Self, Self::Error> {
        let _id = ObjectId::new();
        let count = interval.count.parse::<f64>().expect(&generate_error_text("count"));
        let end_time = interval.end_time.parse::<i64>().expect(&generate_error_text("endTime"));
        let start_time = interval.start_time.parse::<i64>().expect(&generate_error_text("startTime"));
        let units = interval.units.parse::<f64>().expect(&generate_error_text("untis"));
        let rune_pool_obj = Self {
            _id,
            count,
            end_time,
            start_time,
            units
        };
        Ok(rune_pool_obj)
    }
}