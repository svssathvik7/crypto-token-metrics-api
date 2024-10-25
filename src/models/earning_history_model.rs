use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::{openapi::schema, ToSchema};


#[derive(Debug,Serialize,Deserialize,ToSchema)]
pub struct PoolEarningSummary{
    #[schema(value_type = String, example = "60d5ec49a1c4b5048c0e5c70")]
    pub _id : ObjectId,
    #[schema(example=36.58)]
    pub avg_node_count : f64,
    #[schema(example=268913207)]
    pub block_rewards : f64,
    #[schema(example=90350204)]
    pub bonding_earnings : f64,
    #[schema(example=268913207)]
    pub earnings : u64,
    #[schema(example=1647914400)]
    pub end_time : i64,
    #[schema(example=178563003)]
    pub liquidity_earnings : f64,
    #[schema(value_type=u64,example=15949748490.0)]
    pub liquidity_fees : u64,
    #[schema(example=1647914400)]
    pub start_time : i64,
    #[schema(example=8.508409670179631)]
    pub rune_price_usd : f64,
}  

#[derive(Debug,Deserialize,Serialize,ToSchema)]
pub struct PoolEarningHistory{
    #[schema(value_type = String, example = "60d5ec49a1c4b5048c0e5c70")]
    pub _id : ObjectId,
    #[schema(example="TERRA.LUNA")]
    pub pool : String,
    #[schema(example=9756653557.0)]
    pub asset_liquidity_fees : f64,
    #[schema(example=4405821942.0)]
    pub earning : u64,
    #[schema(example=747444314.0)]
    pub rewards : f64,
    #[schema(example=2591263713.0)]
    pub rune_liquidity_fees : f64,
    #[schema(example=0)]
    pub saver_earning : f64,
    #[schema(example=36583776280.0)]
    pub total_liquidity_fees_rune : f64,
    #[schema(example=1647921600)]
    pub start_time : i64,
    #[schema(example=1647925200)]
    pub end_time : i64,
    #[schema(value_type=String,example="67186c6f8a3d488dd6050676")]
    pub earnings_summary : ObjectId
}

