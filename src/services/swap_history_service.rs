use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::swap_history_model::SwapHistory;
use reqwest::Error as reqwestError;
use super::db::DataBase;

fn generate_api_url(pool:String,interval:String,from:String,count:String) -> String{
    format!("https://midgard.ninerealms.com/v2/history/swaps?pool={}&interval={}&from={}&count={}",pool,interval,from,count)
}

fn generate_error_text(field_name:&str) -> String{
    format!("Incorrect {} format",field_name)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub startTime: String,
    pub endTime: String,
    pub toAssetCount: String,
    pub toRuneCount: String,
    pub toTradeCount: String,
    pub fromTradeCount: String,
    pub synthMintCount: String,
    pub synthRedeemCount: String,
    pub totalCount: String,
    pub toAssetVolume: String,
    pub toRuneVolume: String,
    pub toTradeVolume: String,
    pub fromTradeVolume: String,
    pub synthMintVolume: String,
    pub synthRedeemVolume: String,
    pub totalVolume: String,
    pub toAssetVolumeUSD: String,
    pub toRuneVolumeUSD: String,
    pub toTradeVolumeUSD: String,
    pub fromTradeVolumeUSD: String,
    pub synthMintVolumeUSD: String,
    pub synthRedeemVolumeUSD: String,
    pub totalVolumeUSD: String,
    pub toAssetFees: String,
    pub toRuneFees: String,
    pub toTradeFees: String,
    pub fromTradeFees: String,
    pub synthMintFees: String,
    pub synthRedeemFees: String,
    pub totalFees: String,
    pub toAssetAverageSlip: String,
    pub toRuneAverageSlip: String,
    pub toTradeAverageSlip: String,
    pub fromTradeAverageSlip: String,
    pub synthMintAverageSlip: String,
    pub synthRedeemAverageSlip: String,
    pub averageSlip: String,
    pub runePriceUSD: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Interval {
    pub startTime: String,
    pub endTime: String,
    pub toAssetCount: String,
    pub toRuneCount: String,
    pub toTradeCount: String,
    pub fromTradeCount: String,
    pub synthMintCount: String,
    pub synthRedeemCount: String,
    pub totalCount: String,
    pub toAssetVolume: String,
    pub toRuneVolume: String,
    pub toTradeVolume: String,
    pub fromTradeVolume: String,
    pub synthMintVolume: String,
    pub synthRedeemVolume: String,
    pub totalVolume: String,
    pub toAssetVolumeUSD: String,
    pub toRuneVolumeUSD: String,
    pub toTradeVolumeUSD: String,
    pub fromTradeVolumeUSD: String,
    pub synthMintVolumeUSD: String,
    pub synthRedeemVolumeUSD: String,
    pub totalVolumeUSD: String,
    pub toAssetFees: String,
    pub toRuneFees: String,
    pub toTradeFees: String,
    pub fromTradeFees: String,
    pub synthMintFees: String,
    pub synthRedeemFees: String,
    pub totalFees: String,
    pub toAssetAverageSlip: String,
    pub toRuneAverageSlip: String,
    pub toTradeAverageSlip: String,
    pub fromTradeAverageSlip: String,
    pub synthMintAverageSlip: String,
    pub synthRedeemAverageSlip: String,
    pub averageSlip: String,
    pub runePriceUSD: String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ApiResponse{
    pub intervals: Vec<Interval>,
    pub meta: Meta
}

impl SwapHistory{
    pub async fn store_swap_history(db:&DataBase,pool:&str,data:ApiResponse){
        for interval in data.intervals{
            let pool_swap_history = Self {
                _id: ObjectId::new(),
                pool : pool.to_string(),
                average_slip: interval.averageSlip.parse::<f64>().expect(&generate_error_text("averageSlip")),
                end_time: interval.endTime.parse::<i64>().expect(&generate_error_text("endTime")),
                from_trade_average_slip: interval.fromTradeAverageSlip.parse::<f64>().expect(&generate_error_text("fromTradeAverageSlip")),
                from_trade_count: interval.fromTradeCount.parse::<i64>().expect(&generate_error_text("fromTradeCount")),
                from_trade_fees: interval.fromTradeFees.parse::<f64>().expect(&generate_error_text("fromTradeFees")),
                from_trade_volume: interval.fromTradeVolume.parse::<f64>().expect(&generate_error_text("fromTradeVolume")),
                from_trade_volume_usd: interval.fromTradeVolumeUSD.parse::<f64>().expect(&generate_error_text("fromTradeVolumeUSD")),
                rune_price_usd: interval.runePriceUSD.parse::<f64>().expect(&generate_error_text("runePriceUSD")),
                start_time: interval.startTime.parse::<i64>().expect(&generate_error_text("startTime")),
                synth_mint_average_slip: interval.synthMintAverageSlip.parse::<f64>().expect(&generate_error_text("synthMintAverageSlip")),
                synth_mint_count: interval.synthMintCount.parse::<i64>().expect(&generate_error_text("synthMintCount")),
                synth_mint_fees: interval.synthMintFees.parse::<f64>().expect(&generate_error_text("synthMintFees")),
                synth_mint_volume: interval.synthMintVolume.parse::<f64>().expect(&generate_error_text("synthMintVolume")),
                synth_mint_volume_usd: interval.synthMintVolumeUSD.parse::<f64>().expect(&generate_error_text("synthMintVolumeUSD")),
                synth_redeem_average_slip: interval.synthRedeemAverageSlip.parse::<f64>().expect(&generate_error_text("synthRedeemAverageSlip")),
                synth_redeem_count: interval.synthRedeemCount.parse::<i64>().expect(&generate_error_text("synthRedeemCount")),
                synth_redeem_fees: interval.synthRedeemFees.parse::<f64>().expect(&generate_error_text("synthRedeemFees")),
                synth_redeem_volume: interval.synthRedeemVolume.parse::<f64>().expect(&generate_error_text("synthRedeemVolume")),
                synth_redeem_volume_usd: interval.synthRedeemVolumeUSD.parse::<f64>().expect(&generate_error_text("synthRedeemVolumeUSD")),
                to_asset_average_slip: interval.toAssetAverageSlip.parse::<f64>().expect(&generate_error_text("toAssetAverageSlip")),
                to_asset_count: interval.toAssetCount.parse::<i64>().expect(&generate_error_text("toAssetCount")),
                to_asset_fees: interval.toAssetFees.parse::<f64>().expect(&generate_error_text("toAssetFees")),
                to_asset_volume: interval.toAssetVolume.parse::<f64>().expect(&generate_error_text("toAssetVolume")),
                to_asset_volume_usd: interval.toAssetVolumeUSD.parse::<f64>().expect(&generate_error_text("toAssetVolumeUSD")),
                to_rune_average_slip: interval.toRuneAverageSlip.parse::<f64>().expect(&generate_error_text("toRuneAverageSlip")),
                to_rune_count: interval.toRuneCount.parse::<i64>().expect(&generate_error_text("toRuneCount")),
                to_rune_fees: interval.toRuneFees.parse::<f64>().expect(&generate_error_text("toRuneFees")),
                to_rune_volume: interval.toRuneVolume.parse::<f64>().expect(&generate_error_text("toRuneVolume")),
                to_rune_volume_usd: interval.toRuneVolumeUSD.parse::<f64>().expect(&generate_error_text("toRuneVolumeUSD")),
                to_trade_average_slip: interval.toTradeAverageSlip.parse::<f64>().expect(&generate_error_text("toTradeAverageSlip")),
                to_trade_count: interval.toTradeCount.parse::<i64>().expect(&generate_error_text("toTradeCount")),
                to_trade_fees: interval.toTradeFees.parse::<f64>().expect(&generate_error_text("toTradeFees")),
                to_trade_volume: interval.toTradeVolume.parse::<f64>().expect(&generate_error_text("toTradeVolume")),
                to_trade_volume_usd: interval.toTradeVolumeUSD.parse::<f64>().expect(&generate_error_text("toTradeVolumeUSD")),
                total_count: interval.totalCount.parse::<i64>().expect(&generate_error_text("totalCount")),
                total_fees: interval.totalFees.parse::<f64>().expect(&generate_error_text("totalFees")),
                total_volume: interval.totalVolume.parse::<f64>().expect(&generate_error_text("totalVolume")),
                total_volume_usd: interval.totalVolumeUSD.parse::<f64>().expect(&generate_error_text("totalVolumeUSD"))
            };
            
        }
    }
    pub async fn fetch_swap_history(db:&DataBase,pool:String,interval:String,count:String,from:String) -> Result<i64,reqwestError>{
        let url = generate_api_url(pool,interval, from, count);
        print!("url - {}",url);
        let response: ApiResponse = reqwest::get(&url).await?.json::<ApiResponse>().await?;
        
        println!("{:?}",response);
        let end_time = response.meta.endTime.clone();
        let end_time = end_time.parse::<i64>().unwrap();
        Ok(end_time)
    }
}