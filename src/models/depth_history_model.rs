use std::error::Error as stdError;
use mongodb::bson::oid::ObjectId;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::{parse_field, services::depth_history_service::Interval};

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

impl TryFrom<Interval> for PoolDepthPriceHistory {
    type Error = Box<dyn stdError>;

    fn try_from(value: Interval) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            pool: String::from("BTC.BTC"),
            asset_depth: parse_field!(value, asset_depth, f64),
            asset_price: parse_field!(value, asset_price, f64),
            asset_price_usd: parse_field!(value, asset_price_usd, f64),
            end_time: parse_field!(value, end_time, i64),
            liquidity_units: parse_field!(value, liquidity_units, f64),
            luvi: parse_field!(value, luvi, f64),
            members_count: parse_field!(value, members_count, i64),
            rune_depth: parse_field!(value, rune_depth, f64),
            start_time: parse_field!(value, start_time, i64),
            synth_supply: parse_field!(value, synth_supply, f64),
            synth_units: parse_field!(value, synth_units, f64),
            units: parse_field!(value, units, f64),
        })
    }
}

