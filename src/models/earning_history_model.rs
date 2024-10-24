use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Debug,Serialize,Deserialize,ToSchema)]
pub struct PoolEarningSummary{
    #[schema(value_type = String, example = "60d5ec49a1c4b5048c0e5c70")]
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
    pub asset_liquidity_fees : f64,
    pub earning : u64,
    pub rewards : f64,
    pub rune_liquidity_fees : f64,
    pub saver_earning : f64,
    pub total_liquidity_fees_rune : f64,
    pub start_time : i64,
    pub end_time : i64,
    pub earnings_summary : ObjectId
}

