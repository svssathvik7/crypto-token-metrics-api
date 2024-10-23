use serde::{Deserialize, Serialize};

use crate::models::swap_history::SwapHistory;
use reqwest::Error as reqwestError;
use super::db::DataBase;

fn generate_api_url(pool:String,interval:String,from:String,count:String) -> String{
    format!("https://midgard.ninerealms.com/v2/history/swaps?pool={}interval={}&from={}&count={}",pool,interval,from,count)
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
    pub meta: Meta,
    pub intervals: Vec<Interval>
}

impl SwapHistory{
    pub async fn fetch_swap_history(db:&DataBase,interval:String,count:String,from:String) -> Result<i64,reqwestError>{
        let url = generate_api_url("BTC.BTC".to_string(),interval, from, count);
        print!("url - {}",url);
        let response: ApiResponse = reqwest::get(&url).await?.json::<ApiResponse>().await?;
        // println!("{:?}",response);
        let end_time = response.meta.endTime.clone();
        let end_time = end_time.parse::<i64>().unwrap();
        Ok(end_time)
    }
}