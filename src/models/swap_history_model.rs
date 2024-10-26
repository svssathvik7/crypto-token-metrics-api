use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use std::error::Error as stdError;
use crate::{parse_field, services::swap_history_service::Interval};

#[derive(Debug,Serialize,Deserialize,ToSchema)]
#[schema(rename_all="camelCase")]
pub struct SwapHistory {
    #[schema(value_type = String, example = "60d5ec49a1c4b5048c0e5c70",rename="._id not exposed in the response")]
    pub _id: ObjectId,
    #[schema(example="BTC.BTC")]
    pub pool: String,
    #[schema(example=8.508409670179631)]
    pub average_slip: f64,
    #[schema(example=1647914400)]
    pub end_time: i64,
    #[schema(example=8.508409670179631)]
    pub from_trade_average_slip: f64,
    #[schema(example=8967017)]
    pub from_trade_count: i64,
    #[schema(example=8.508409670179631)]
    pub from_trade_fees: f64,
    #[schema(example=8.508409670179631)]
    pub from_trade_volume: f64,
    #[schema(example=8.508409670179631)]
    pub from_trade_volume_usd: f64,
    #[schema(example=8.508409670179631)]
    pub rune_price_usd: f64,
    pub start_time: i64,
    #[schema(example=8.508409670179631)]
    pub synth_mint_average_slip: f64,
    pub synth_mint_count: i64,
    #[schema(example=8.508409670179631)]
    pub synth_mint_fees: f64,
    #[schema(example=8.508409670179631)]
    pub synth_mint_volume: f64,
    #[schema(example=8.508409670179631)]
    pub synth_mint_volume_usd: f64,
    #[schema(example=8.508409670179631)]
    pub synth_redeem_average_slip: f64,
    #[schema(example=8.508409670179631)]
    pub synth_redeem_count: i64,
    #[schema(example=8.508409670179631)]
    pub synth_redeem_fees: f64,
    #[schema(example=8.508409670179631)]
    pub synth_redeem_volume: f64,
    #[schema(example=8.508409670179631,rename="synthRedeemVolumeUSD")]
    pub synth_redeem_volume_usd: f64,
    #[schema(example=8.508409670179631)]
    pub to_asset_average_slip: f64,
    #[schema(example=8.508409670179631)]
    pub to_asset_count: i64,
    #[schema(example=8.508409670179631)]
    pub to_asset_fees: f64,
    #[schema(example=8.508409670179631)]
    pub to_asset_volume: f64,
    #[schema(example=8.508409670179631,rename="toAssetVolumeUSD")]
    pub to_asset_volume_usd: f64,
    #[schema(example=8.508409670179631)]
    pub to_rune_average_slip: f64,
    #[schema(example=8.508409670179631)]
    pub to_rune_count: i64,
    #[schema(example=8.508409670179631)]
    pub to_rune_fees: f64,
    #[schema(example=8.508409670179631)]
    pub to_rune_volume: f64,
    #[schema(example=8.508409670179631,rename="toRuneVolumeUSD")]
    pub to_rune_volume_usd: f64,
    #[schema(example=8.508409670179631)]
    pub to_trade_average_slip: f64,
    #[schema(example=8.508409670179631)]
    pub to_trade_count: i64,
    #[schema(example=8.508409670179631)]
    pub to_trade_fees: f64,
    #[schema(example=8.508409670179631)]
    pub to_trade_volume: f64,
    #[schema(example=8.508409670179631,rename="toTradeVolumeUSD")]
    pub to_trade_volume_usd: f64,
    #[schema(example=8.508409670179631)]
    pub total_count: i64,
    #[schema(example=8.508409670179631)]
    pub total_fees: f64,
    #[schema(example=8.508409670179631)]
    pub total_volume: f64,
    #[schema(example=8.508409670179631,rename="totalVolumeUSD")]
    pub total_volume_usd: f64,
}


impl SwapHistory {
    pub fn to_swap_history(interval: Interval, pool: &str) -> Result<Self, Box<dyn stdError>> {
        let _id = ObjectId::new();
        let pool = pool.to_string();
        
        Ok(Self {
            _id,
            pool,
            average_slip: parse_field!(interval, average_slip, f64),
            end_time: parse_field!(interval, end_time, i64),
            from_trade_average_slip: parse_field!(interval, from_trade_average_slip, f64),
            from_trade_count: parse_field!(interval, from_trade_count, i64),
            from_trade_fees: parse_field!(interval, from_trade_fees, f64),
            from_trade_volume: parse_field!(interval, from_trade_volume, f64),
            from_trade_volume_usd: parse_field!(interval, from_trade_volume_usd, f64),
            rune_price_usd: parse_field!(interval, rune_price_usd, f64),
            start_time: parse_field!(interval, start_time, i64),
            synth_mint_average_slip: parse_field!(interval, synth_mint_average_slip, f64),
            synth_mint_count: parse_field!(interval, synth_mint_count, i64),
            synth_mint_fees: parse_field!(interval, synth_mint_fees, f64),
            synth_mint_volume: parse_field!(interval, synth_mint_volume, f64),
            synth_mint_volume_usd: parse_field!(interval, synth_mint_volume_usd, f64),
            synth_redeem_average_slip: parse_field!(interval, synth_redeem_average_slip, f64),
            synth_redeem_count: parse_field!(interval, synth_redeem_count, i64),
            synth_redeem_fees: parse_field!(interval, synth_redeem_fees, f64),
            synth_redeem_volume: parse_field!(interval, synth_redeem_volume, f64),
            synth_redeem_volume_usd: parse_field!(interval, synth_redeem_volume_usd, f64),
            to_asset_average_slip: parse_field!(interval, to_asset_average_slip, f64),
            to_asset_count: parse_field!(interval, to_asset_count, i64),
            to_asset_fees: parse_field!(interval, to_asset_fees, f64),
            to_asset_volume: parse_field!(interval, to_asset_volume, f64),
            to_asset_volume_usd: parse_field!(interval, to_asset_volume_usd, f64),
            to_rune_average_slip: parse_field!(interval, to_rune_average_slip, f64),
            to_rune_count: parse_field!(interval, to_rune_count, i64),
            to_rune_fees: parse_field!(interval, to_rune_fees, f64),
            to_rune_volume: parse_field!(interval, to_rune_volume, f64),
            to_rune_volume_usd: parse_field!(interval, to_rune_volume_usd, f64),
            to_trade_average_slip: parse_field!(interval, to_trade_average_slip, f64),
            to_trade_count: parse_field!(interval, to_trade_count, i64),
            to_trade_fees: parse_field!(interval, to_trade_fees, f64),
            to_trade_volume: parse_field!(interval, to_trade_volume, f64),
            to_trade_volume_usd: parse_field!(interval, to_trade_volume_usd, f64),
            total_count: parse_field!(interval, total_count, i64),
            total_fees: parse_field!(interval, total_fees, f64),
            total_volume: parse_field!(interval, total_volume, f64),
            total_volume_usd: parse_field!(interval, total_volume_usd, f64),
        })
    }
}
