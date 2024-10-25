use std::error::Error as stdError;
use mongodb::bson::oid::ObjectId;

use serde::{Deserialize, Serialize};
use utoipa::{openapi::schema, ToSchema};
use crate::services::depth_history_service::Interval;

fn generate_error_text(field_name:&str) -> String{
    format!("Incorrect {} format",field_name)
}

#[derive(Deserialize,Serialize,Debug,ToSchema)]
pub struct PoolDepthPriceHistory{
    #[schema(value_type = String, example = "60d5ec49a1c4b5048c0e5c70")]
    pub _id : ObjectId,
    #[schema(example= "BTC.BTC")]
    pub pool : String,
    #[schema(example = 709654.78)]
    pub asset_depth : f64,
    #[schema(example = 70.10)]
    pub asset_price : f64,
    #[schema(example = 8000.02)]
    pub asset_price_usd : f64,
    #[schema(example = 1653373410)]
    pub end_time : i64,
    #[schema(example = 700000.0)]
    pub liquidity_units : f64,
    #[schema(example = 0.015679655950478353)]
    pub luvi : f64,
    #[schema(example = 250)]
    pub members_count : i64,
    #[schema(example = 1029722955087509.0)]
    pub rune_depth : f64,
    #[schema(example = 1653373410)]
    pub start_time : i64,
    #[schema(example = 59144723874.0)]
    pub synth_supply : f64,
    #[schema(example = 215018050215853.0)]
    pub synth_units : f64,
    #[schema(example = 576047677431855.0)]
    pub units : f64
}

impl TryFrom<Interval> for PoolDepthPriceHistory{
    type Error = Box<dyn stdError>;
    fn try_from(value: Interval) -> Result<Self, Self::Error> {
        let _id = ObjectId::new();
        let pool = String::from("BTC.BTC");
        let asset_depth = value.asset_depth.parse::<f64>().expect(&generate_error_text("assetDepth"));
        let asset_price = value.asset_price.parse::<f64>().expect(&generate_error_text("assetPrice"));
        let asset_price_usd = value.asset_price_usd.parse::<f64>().expect(&generate_error_text("assetPriceUSD"));
        let end_time = value.end_time.parse::<i64>().expect(&generate_error_text("endTime"));
        let liquidity_units = value.liquidity_units.parse::<f64>().expect(&generate_error_text("liquidityUnits"));
        let luvi = value.luvi.parse::<f64>().expect(&generate_error_text("luvi"));
        let members_count = value.members_count.parse::<i64>().expect(&generate_error_text("membersCount"));
        let rune_depth = value.rune_depth.parse::<f64>().expect(&generate_error_text("runeDepth"));
        let start_time = value.start_time.parse::<i64>().expect(&generate_error_text("startTime"));
        let synth_supply = value.synth_supply.parse::<f64>().expect(&generate_error_text("synthSupply"));
        let synth_units = value.synth_units.parse::<f64>().expect(&generate_error_text("synthUnits"));
        let units = value.units.parse::<f64>().expect(&generate_error_text("units"));
        let pool_price_document = PoolDepthPriceHistory {
            _id,
            pool,
            asset_depth,
            asset_price,
            asset_price_usd,
            end_time,
            liquidity_units,
            luvi,
            members_count,
            rune_depth,
            start_time,
            synth_supply,
            synth_units,
            units,
        };
        Ok(pool_price_document)
    }
}

