use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::error::Error as stdError;
use crate::services::swap_history_service::Interval;

fn generate_error_text(field_name:&str) -> String{
    format!("Incorrect {} format",field_name)
}

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


impl SwapHistory {
    pub fn to_swap_history(interval: Interval, pool: &str) -> Result<Self, Box<dyn stdError>> {
        let _id = ObjectId::new();
        let pool = pool.to_string();
        let average_slip = interval.average_slip.parse::<f64>().expect(&generate_error_text("averageSlip"));
        let end_time = interval.end_time.parse::<i64>().expect(&generate_error_text("endTime"));
        let from_trade_average_slip = interval.from_trade_average_slip.parse::<f64>().expect(&generate_error_text("fromTradeAverageSlip"));
        let from_trade_count = interval.from_trade_count.parse::<i64>().expect(&generate_error_text("fromTradeCount"));
        let from_trade_fees = interval.from_trade_fees.parse::<f64>().expect(&generate_error_text("fromTradeFees"));
        let from_trade_volume = interval.from_trade_volume.parse::<f64>().expect(&generate_error_text("fromTradeVolume"));
        let from_trade_volume_usd = interval.from_trade_volume_usd.parse::<f64>().expect(&generate_error_text("fromTradeVolumeUSD"));
        let rune_price_usd = interval.rune_price_usd.parse::<f64>().expect(&generate_error_text("runePriceUSD"));
        let start_time = interval.start_time.parse::<i64>().expect(&generate_error_text("startTime"));
        let synth_mint_average_slip = interval.synth_mint_average_slip.parse::<f64>().expect(&generate_error_text("synthMintAverageSlip"));
        let synth_mint_count = interval.synth_mint_count.parse::<i64>().expect(&generate_error_text("synthMintCount"));
        let synth_mint_fees = interval.synth_mint_fees.parse::<f64>().expect(&generate_error_text("synthMintFees"));
        let synth_mint_volume = interval.synth_mint_volume.parse::<f64>().expect(&generate_error_text("synthMintVolume"));
        let synth_mint_volume_usd = interval.synth_mint_volume_usd.parse::<f64>().expect(&generate_error_text("synthMintVolumeUSD"));
        let synth_redeem_average_slip = interval.synth_redeem_average_slip.parse::<f64>().expect(&generate_error_text("synthRedeemAverageSlip"));
        let synth_redeem_count = interval.synth_redeem_count.parse::<i64>().expect(&generate_error_text("synthRedeemCount"));
        let synth_redeem_fees = interval.synth_redeem_fees.parse::<f64>().expect(&generate_error_text("synthRedeemFees"));
        let synth_redeem_volume = interval.synth_redeem_volume.parse::<f64>().expect(&generate_error_text("synthRedeemVolume"));
        let synth_redeem_volume_usd = interval.synth_redeem_volume_usd.parse::<f64>().expect(&generate_error_text("synthRedeemVolumeUSD"));
        let to_asset_average_slip = interval.to_asset_average_slip.parse::<f64>().expect(&generate_error_text("toAssetAverageSlip"));
        let to_asset_count = interval.to_asset_count.parse::<i64>().expect(&generate_error_text("toAssetCount"));
        let to_asset_fees = interval.to_asset_fees.parse::<f64>().expect(&generate_error_text("toAssetFees"));
        let to_asset_volume = interval.to_asset_volume.parse::<f64>().expect(&generate_error_text("toAssetVolume"));
        let to_asset_volume_usd = interval.to_asset_volume_usd.parse::<f64>().expect(&generate_error_text("toAssetVolumeUSD"));
        let to_rune_average_slip = interval.to_rune_average_slip.parse::<f64>().expect(&generate_error_text("toRuneAverageSlip"));
        let to_rune_count = interval.to_rune_count.parse::<i64>().expect(&generate_error_text("toRuneCount"));
        let to_rune_fees = interval.to_rune_fees.parse::<f64>().expect(&generate_error_text("toRuneFees"));
        let to_rune_volume = interval.to_rune_volume.parse::<f64>().expect(&generate_error_text("toRuneVolume"));
        let to_rune_volume_usd = interval.to_rune_volume_usd.parse::<f64>().expect(&generate_error_text("toRuneVolumeUSD"));
        let to_trade_average_slip = interval.to_trade_average_slip.parse::<f64>().expect(&generate_error_text("toTradeAverageSlip"));
        let to_trade_count = interval.to_trade_count.parse::<i64>().expect(&generate_error_text("toTradeCount"));
        let to_trade_fees = interval.to_trade_fees.parse::<f64>().expect(&generate_error_text("toTradeFees"));
        let to_trade_volume = interval.to_trade_volume.parse::<f64>().expect(&generate_error_text("toTradeVolume"));
        let to_trade_volume_usd = interval.to_trade_volume_usd.parse::<f64>().expect(&generate_error_text("toTradeVolumeUSD"));
        let total_count = interval.total_count.parse::<i64>().expect(&generate_error_text("totalCount"));
        let total_fees = interval.total_fees.parse::<f64>().expect(&generate_error_text("totalFees"));
        let total_volume = interval.total_volume.parse::<f64>().expect(&generate_error_text("totalVolume"));
        let total_volume_usd = interval.total_volume_usd.parse::<f64>().expect(&generate_error_text("totalVolumeUSD"));

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
