use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};


#[derive(Debug,Serialize,Deserialize)]
pub struct PoolEarningSummary{
    pub avg_node_count : f64,
    pub block_rewards : u64,
    pub bonding_earnings : u64,
    pub earnings : u64,
    pub end_time : i64,
    pub liquidity_earnings : u64,
    pub liquidity_fees : u64,
    pub start_time : i64,
    pub rune_price_usd : f64,
}

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
    pub end_time : i64,
    pub earnings_summary : ObjectId
}