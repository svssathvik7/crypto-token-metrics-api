use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use std::error::Error as stdError;
use crate::{services::swap_history_service::Interval, utils::parser_utils::{parse_to_f64, parse_to_i64}};

#[derive(Debug,Serialize,Deserialize,ToSchema)]
pub struct SwapHistory {
    #[schema(value_type = String, example = "60d5ec49a1c4b5048c0e5c70")]
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


impl SwapHistory {
    pub fn to_swap_history(interval: Interval, pool: &str) -> Result<Self, Box<dyn stdError>> {
        let _id = ObjectId::new();
        let pool = pool.to_string();
        let average_slip = parse_to_f64(&interval.average_slip, "averageSlip")?;
        let end_time = parse_to_i64(&interval.end_time, "endTime")?;
        let from_trade_average_slip = parse_to_f64(&interval.from_trade_average_slip, "fromTradeAverageSlip")?;
        let from_trade_count = parse_to_i64(&interval.from_trade_count, "fromTradeCount")?;
        let from_trade_fees = parse_to_f64(&interval.from_trade_fees, "fromTradeFees")?;
        let from_trade_volume = parse_to_f64(&interval.from_trade_volume, "fromTradeVolume")?;
        let from_trade_volume_usd = parse_to_f64(&interval.from_trade_volume_usd, "fromTradeVolumeUSD")?;
        let rune_price_usd = parse_to_f64(&interval.rune_price_usd, "runePriceUSD")?;
        let start_time = parse_to_i64(&interval.start_time, "startTime")?;
        let synth_mint_average_slip = parse_to_f64(&interval.synth_mint_average_slip, "synthMintAverageSlip")?;
        let synth_mint_count = parse_to_i64(&interval.synth_mint_count, "synthMintCount")?;
        let synth_mint_fees = parse_to_f64(&interval.synth_mint_fees, "synthMintFees")?;
        let synth_mint_volume = parse_to_f64(&interval.synth_mint_volume, "synthMintVolume")?;
        let synth_mint_volume_usd = parse_to_f64(&interval.synth_mint_volume_usd, "synthMintVolumeUSD")?;
        let synth_redeem_average_slip = parse_to_f64(&interval.synth_redeem_average_slip, "synthRedeemAverageSlip")?;
        let synth_redeem_count = parse_to_i64(&interval.synth_redeem_count, "synthRedeemCount")?;
        let synth_redeem_fees = parse_to_f64(&interval.synth_redeem_fees, "synthRedeemFees")?;
        let synth_redeem_volume = parse_to_f64(&interval.synth_redeem_volume, "synthRedeemVolume")?;
        let synth_redeem_volume_usd = parse_to_f64(&interval.synth_redeem_volume_usd, "synthRedeemVolumeUSD")?;
        let to_asset_average_slip = parse_to_f64(&interval.to_asset_average_slip, "toAssetAverageSlip")?;
        let to_asset_count = parse_to_i64(&interval.to_asset_count, "toAssetCount")?;
        let to_asset_fees = parse_to_f64(&interval.to_asset_fees, "toAssetFees")?;
        let to_asset_volume = parse_to_f64(&interval.to_asset_volume, "toAssetVolume")?;
        let to_asset_volume_usd = parse_to_f64(&interval.to_asset_volume_usd, "toAssetVolumeUSD")?;
        let to_rune_average_slip = parse_to_f64(&interval.to_rune_average_slip, "toRuneAverageSlip")?;
        let to_rune_count = parse_to_i64(&interval.to_rune_count, "toRuneCount")?;
        let to_rune_fees = parse_to_f64(&interval.to_rune_fees, "toRuneFees")?;
        let to_rune_volume = parse_to_f64(&interval.to_rune_volume, "toRuneVolume")?;
        let to_rune_volume_usd = parse_to_f64(&interval.to_rune_volume_usd, "toRuneVolumeUSD")?;
        let to_trade_average_slip = parse_to_f64(&interval.to_trade_average_slip, "toTradeAverageSlip")?;
        let to_trade_count = parse_to_i64(&interval.to_trade_count, "toTradeCount")?;
        let to_trade_fees = parse_to_f64(&interval.to_trade_fees, "toTradeFees")?;
        let to_trade_volume = parse_to_f64(&interval.to_trade_volume, "toTradeVolume")?;
        let to_trade_volume_usd = parse_to_f64(&interval.to_trade_volume_usd, "toTradeVolumeUSD")?;
        let total_count = parse_to_i64(&interval.total_count, "totalCount")?;
        let total_fees = parse_to_f64(&interval.total_fees, "totalFees")?;
        let total_volume = parse_to_f64(&interval.total_volume, "totalVolume")?;
        let total_volume_usd = parse_to_f64(&interval.total_volume_usd, "totalVolumeUSD")?;

        Ok(Self {
            _id,
            pool,
            average_slip,
            end_time,
            from_trade_average_slip,
            from_trade_count,
            from_trade_fees,
            from_trade_volume,
            from_trade_volume_usd,
            rune_price_usd,
            start_time,
            synth_mint_average_slip,
            synth_mint_count,
            synth_mint_fees,
            synth_mint_volume,
            synth_mint_volume_usd,
            synth_redeem_average_slip,
            synth_redeem_count,
            synth_redeem_fees,
            synth_redeem_volume,
            synth_redeem_volume_usd,
            to_asset_average_slip,
            to_asset_count,
            to_asset_fees,
            to_asset_volume,
            to_asset_volume_usd,
            to_rune_average_slip,
            to_rune_count,
            to_rune_fees,
            to_rune_volume,
            to_rune_volume_usd,
            to_trade_average_slip,
            to_trade_count,
            to_trade_fees,
            to_trade_volume,
            to_trade_volume_usd,
            total_count,
            total_fees,
            total_volume,
            total_volume_usd,
        })
    }
}
