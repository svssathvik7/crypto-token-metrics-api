use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{models::{custom_error_model::CustomError, swap_history_model::SwapHistory}, utils::db_helper_utils::get_max_start_time_of_collection};
use reqwest::Error as reqwestError;
use super::db::DataBase;

fn generate_api_url(pool:&str,interval:&str,from:&str,count:&str) -> String{
    format!("https://midgard.ninerealms.com/v2/history/swaps?pool={}&interval={}&from={}&count={}",pool,interval,from,count)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Meta {
    pub start_time: String,
    pub end_time: String,
    pub to_asset_count: String,
    pub to_rune_count: String,
    pub to_trade_count: String,
    pub from_trade_count: String,
    pub synth_mint_count: String,
    pub synth_redeem_count: String,
    pub total_count: String,
    pub to_asset_volume: String,
    pub to_rune_volume: String,
    pub to_trade_volume: String,
    pub from_trade_volume: String,
    pub synth_mint_volume: String,
    pub synth_redeem_volume: String,
    pub total_volume: String,
    #[serde(rename="toAssetVolumeUSD")]
    pub to_asset_volume_usd: String,
    #[serde(rename="toRuneVolumeUSD")]
    pub to_rune_volume_usd: String,
    #[serde(rename="toTradeVolumeUSD")]
    pub to_trade_volume_usd: String,
    #[serde(rename="fromTradeVolumeUSD")]
    pub from_trade_volume_usd: String,
    #[serde(rename="synthMintVolumeUSD")]
    pub synth_mint_volume_usd: String,
    #[serde(rename="synthRedeemVolumeUSD")]
    pub synth_redeem_volume_usd: String,
    #[serde(rename="totalVolumeUSD")]
    pub total_volume_usd: String,
    pub to_asset_fees: String,
    pub to_rune_fees: String,
    pub to_trade_fees: String,
    pub from_trade_fees: String,
    pub synth_mint_fees: String,
    pub synth_redeem_fees: String,
    pub total_fees: String,
    pub to_asset_average_slip: String,
    pub to_rune_average_slip: String,
    pub to_trade_average_slip: String,
    pub from_trade_average_slip: String,
    pub synth_mint_average_slip: String,
    pub synth_redeem_average_slip: String,
    pub average_slip: String,
    #[serde(rename="runePriceUSD")]
    pub rune_price_usd: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Interval {
    pub start_time: String,
    pub end_time: String,
    pub to_asset_count: String,
    pub to_rune_count: String,
    pub to_trade_count: String,
    pub from_trade_count: String,
    pub synth_mint_count: String,
    pub synth_redeem_count: String,
    pub total_count: String,
    pub to_asset_volume: String,
    pub to_rune_volume: String,
    pub to_trade_volume: String,
    pub from_trade_volume: String,
    pub synth_mint_volume: String,
    pub synth_redeem_volume: String,
    pub total_volume: String,
    #[serde(rename = "toAssetVolumeUSD")]
    pub to_asset_volume_usd: String,
    #[serde(rename = "toRuneVolumeUSD")]
    pub to_rune_volume_usd: String,
    #[serde(rename = "toTradeVolumeUSD")]
    pub to_trade_volume_usd: String,
    #[serde(rename = "fromTradeVolumeUSD")]
    pub from_trade_volume_usd: String,
    #[serde(rename = "synthMintVolumeUSD")]
    pub synth_mint_volume_usd: String,
    #[serde(rename = "synthRedeemVolumeUSD")]
    pub synth_redeem_volume_usd: String,
    #[serde(rename = "totalVolumeUSD")]
    pub total_volume_usd: String,
    pub to_asset_fees: String,
    pub to_rune_fees: String,
    pub to_trade_fees: String,
    pub from_trade_fees: String,
    pub synth_mint_fees: String,
    pub synth_redeem_fees: String,
    pub total_fees: String,
    pub to_asset_average_slip: String,
    pub to_rune_average_slip: String,
    pub to_trade_average_slip: String,
    pub from_trade_average_slip: String,
    pub synth_mint_average_slip: String,
    pub synth_redeem_average_slip: String,
    pub average_slip: String,
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ApiResponse{
    pub intervals: Vec<Interval>,
    pub meta: Meta
}

impl SwapHistory{
    pub async fn store_swap_history(db:&DataBase,pool:&str,data:ApiResponse){
        for interval in data.intervals{
            let pool_swap_history = SwapHistory::to_swap_history(interval, pool).unwrap();
            match db.swap_history.insert_one(pool_swap_history).await {
                Ok(_record) => println!("Inserted pool swap doc {}",pool),
                Err(e) => eprint!("Error inserting swap doc to db {:?}",e)
            }
        }
    }
    pub async fn fetch_swap_history(db:&DataBase,pool:&str,interval:&str,count:&str,from:&str) -> Result<i64,reqwestError>{
        let url = generate_api_url(&pool,&interval, &from, &count);
        println!("url - {}",url);
        let response: ApiResponse = reqwest::get(&url).await?.json::<ApiResponse>().await?;
        
        let end_time = response.meta.end_time.clone();
        let end_time = end_time.parse::<i64>().unwrap();
        self::SwapHistory::store_swap_history(db, pool, response).await;
        Ok(end_time)
    }
}