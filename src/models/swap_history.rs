use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};



#[derive(Debug,Serialize,Deserialize)]
pub struct SwapHistory {
    pub _id: ObjectId,
    pub pool: String,
    pub average_slip: f64,
    pub end_time: i64,
    pub from_trade_average_slip: f64,
    pub from_trade_count: i64,
    pub from_trade_fees: f64,
    pub from_trade_volume: f64,
    pub from_trade_volume_usd: f64,
    pub rune_price_usd: f64,
    pub start_time: i64,
    pub synth_mint_average_slip: f64,
    pub synth_mint_count: i64,
    pub synth_mint_fees: f64,
    pub synth_mint_volume: f64,
    pub synth_mint_volume_usd: f64,
    pub synth_redeem_average_slip: f64,
    pub synth_redeem_count: i64,
    pub synth_redeem_fees: f64,
    pub synth_redeem_volume: f64,
    pub synth_redeem_volume_usd: f64,
    pub to_asset_average_slip: f64,
    pub to_asset_count: i64,
    pub to_asset_fees: f64,
    pub to_asset_volume: f64,
    pub to_asset_volume_usd: f64,
    pub to_rune_average_slip: f64,
    pub to_rune_count: i64,
    pub to_rune_fees: f64,
    pub to_rune_volume: f64,
    pub to_rune_volume_usd: f64,
    pub to_trade_average_slip: f64,
    pub to_trade_count: i64,
    pub to_trade_fees: f64,
    pub to_trade_volume: f64,
    pub to_trade_volume_usd: f64,
    pub total_count: i64,
    pub total_fees: f64,
    pub total_volume: f64,
    pub total_volume_usd: f64,
}
