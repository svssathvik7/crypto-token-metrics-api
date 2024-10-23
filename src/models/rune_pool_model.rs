use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};


#[derive(Debug,Serialize,Deserialize)]
pub struct RunePool{
    _id : ObjectId,
    count : f64,
    end_time : i64,
    start_time : i64,
    units : f64
}