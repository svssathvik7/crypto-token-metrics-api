use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};


#[derive(Debug,Serialize,Deserialize)]
pub struct RunePool{
    pub _id : ObjectId,
    pub count : f64,
    pub end_time : i64,
    pub start_time : i64,
    pub units : f64
}