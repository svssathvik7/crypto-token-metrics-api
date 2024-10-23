use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::error::Error as stdError;
use crate::services::{depth_history_service::ApiResponse, earnings_history_service::{Interval, Pool}};


fn generate_error_text(field_name:String) -> String{
    format!("Incorrect {} format",field_name)
}

#[derive(Debug,Serialize,Deserialize)]
pub struct PoolEarningSummary{
    pub _id : ObjectId,
    pub avg_node_count : f64,
    pub block_rewards : f64,
    pub bonding_earnings : f64,
    pub earnings : u64,
    pub end_time : i64,
    pub liquidity_earnings : f64,
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

