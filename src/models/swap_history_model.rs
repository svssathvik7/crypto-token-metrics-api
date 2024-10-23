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
        let average_slip = interval.averageSlip.parse::<f64>().expect(&generate_error_text("averageSlip"));
        let end_time = interval.endTime.parse::<i64>().expect(&generate_error_text("endTime"));
        let from_trade_average_slip = interval.fromTradeAverageSlip.parse::<f64>().expect(&generate_error_text("fromTradeAverageSlip"));
        let from_trade_count = interval.fromTradeCount.parse::<i64>().expect(&generate_error_text("fromTradeCount"));
        let from_trade_fees = interval.fromTradeFees.parse::<f64>().expect(&generate_error_text("fromTradeFees"));
        let from_trade_volume = interval.fromTradeVolume.parse::<f64>().expect(&generate_error_text("fromTradeVolume"));
        let from_trade_volume_usd = interval.fromTradeVolumeUSD.parse::<f64>().expect(&generate_error_text("fromTradeVolumeUSD"));
        let rune_price_usd = interval.runePriceUSD.parse::<f64>().expect(&generate_error_text("runePriceUSD"));
        let start_time = interval.startTime.parse::<i64>().expect(&generate_error_text("startTime"));
        let synth_mint_average_slip = interval.synthMintAverageSlip.parse::<f64>().expect(&generate_error_text("synthMintAverageSlip"));
        let synth_mint_count = interval.synthMintCount.parse::<i64>().expect(&generate_error_text("synthMintCount"));
        let synth_mint_fees = interval.synthMintFees.parse::<f64>().expect(&generate_error_text("synthMintFees"));
        let synth_mint_volume = interval.synthMintVolume.parse::<f64>().expect(&generate_error_text("synthMintVolume"));
        let synth_mint_volume_usd = interval.synthMintVolumeUSD.parse::<f64>().expect(&generate_error_text("synthMintVolumeUSD"));
        let synth_redeem_average_slip = interval.synthRedeemAverageSlip.parse::<f64>().expect(&generate_error_text("synthRedeemAverageSlip"));
        let synth_redeem_count = interval.synthRedeemCount.parse::<i64>().expect(&generate_error_text("synthRedeemCount"));
        let synth_redeem_fees = interval.synthRedeemFees.parse::<f64>().expect(&generate_error_text("synthRedeemFees"));
        let synth_redeem_volume = interval.synthRedeemVolume.parse::<f64>().expect(&generate_error_text("synthRedeemVolume"));
        let synth_redeem_volume_usd = interval.synthRedeemVolumeUSD.parse::<f64>().expect(&generate_error_text("synthRedeemVolumeUSD"));
        let to_asset_average_slip = interval.toAssetAverageSlip.parse::<f64>().expect(&generate_error_text("toAssetAverageSlip"));
        let to_asset_count = interval.toAssetCount.parse::<i64>().expect(&generate_error_text("toAssetCount"));
        let to_asset_fees = interval.toAssetFees.parse::<f64>().expect(&generate_error_text("toAssetFees"));
        let to_asset_volume = interval.toAssetVolume.parse::<f64>().expect(&generate_error_text("toAssetVolume"));
        let to_asset_volume_usd = interval.toAssetVolumeUSD.parse::<f64>().expect(&generate_error_text("toAssetVolumeUSD"));
        let to_rune_average_slip = interval.toRuneAverageSlip.parse::<f64>().expect(&generate_error_text("toRuneAverageSlip"));
        let to_rune_count = interval.toRuneCount.parse::<i64>().expect(&generate_error_text("toRuneCount"));
        let to_rune_fees = interval.toRuneFees.parse::<f64>().expect(&generate_error_text("toRuneFees"));
        let to_rune_volume = interval.toRuneVolume.parse::<f64>().expect(&generate_error_text("toRuneVolume"));
        let to_rune_volume_usd = interval.toRuneVolumeUSD.parse::<f64>().expect(&generate_error_text("toRuneVolumeUSD"));
        let to_trade_average_slip = interval.toTradeAverageSlip.parse::<f64>().expect(&generate_error_text("toTradeAverageSlip"));
        let to_trade_count = interval.toTradeCount.parse::<i64>().expect(&generate_error_text("toTradeCount"));
        let to_trade_fees = interval.toTradeFees.parse::<f64>().expect(&generate_error_text("toTradeFees"));
        let to_trade_volume = interval.toTradeVolume.parse::<f64>().expect(&generate_error_text("toTradeVolume"));
        let to_trade_volume_usd = interval.toTradeVolumeUSD.parse::<f64>().expect(&generate_error_text("toTradeVolumeUSD"));
        let total_count = interval.totalCount.parse::<i64>().expect(&generate_error_text("totalCount"));
        let total_fees = interval.totalFees.parse::<f64>().expect(&generate_error_text("totalFees"));
        let total_volume = interval.totalVolume.parse::<f64>().expect(&generate_error_text("totalVolume"));
        let total_volume_usd = interval.totalVolumeUSD.parse::<f64>().expect(&generate_error_text("totalVolumeUSD"));

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
