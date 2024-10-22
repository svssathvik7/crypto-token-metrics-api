use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct PoolEarningHistory{
    pub _id : ObjectId,
    pub pool : String,
    pub asset_liquidity_fees : u64,
    pub earning : u64,
    pub rewards : u64,
    pub rune_liquidity_fees : u64,
    pub saver_earning : u64,
    pub total_liquidity_fees_rune : u64,
    pub start_time : i64,
    pub end_time : i64
}