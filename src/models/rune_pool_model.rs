use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{parse_field, services::rune_pool_service::Interval};
use std::error::Error as stdError;

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

impl TryFrom<Interval> for RunePool {
    type Error = Box<dyn stdError>;

    fn try_from(interval: Interval) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            count: parse_field!(interval, count, f64),
            end_time: parse_field!(interval, end_time, i64),
            start_time: parse_field!(interval, start_time, i64),
            units: parse_field!(interval, units, f64),
        })
    }
}
