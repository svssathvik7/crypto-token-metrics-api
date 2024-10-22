use mongodb::bson::oid::ObjectId;

pub struct PoolDepthPriceHistory{
    pub _id : ObjectId,
    pub name : String,
    pub asset_depth : i64,
    pub asset_price : f64,
    pub asset_price_usd : f64,
    pub end_time : i64,
    pub liquidity_units : i64,
    pub luvi : f64,
    pub members_count : i32,
    pub rune_depth : i64,
    pub start_time : i64,
    pub synth_supply : i64,
    pub synth_units : i64,
    pub units : i64
}