use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::services::rune_pool_service::Interval;
use std::error::Error as stdError;

fn generate_error_text(field_name:&str) -> String{
    format!("Incorrect {} format",field_name)
}

#[derive(Debug,Serialize,Deserialize)]
pub struct RunePool{
    pub _id : ObjectId,
    pub count : f64,
    pub end_time : i64,
    pub start_time : i64,
    pub units : f64
}

impl TryFrom<Interval> for RunePool{
    type Error = Box<dyn stdError>;
    fn try_from(interval: Interval) -> Result<Self, Self::Error> {
        let _id = ObjectId::new();
        let count = interval.count.parse::<f64>().expect(&generate_error_text("count"));
        let end_time = interval.endTime.parse::<i64>().expect(&generate_error_text("endTime"));
        let start_time = interval.startTime.parse::<i64>().expect(&generate_error_text("startTime"));
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