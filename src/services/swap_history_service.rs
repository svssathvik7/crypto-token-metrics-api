use serde::{Deserialize, Serialize};
use crate::models::swap_history_model::SwapHistory;
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
    pub async fn store_swap_history(db:&DataBase,pool:&str,data:ApiResponse) -> Result<(),String>{
        for interval in data.intervals{
            let pool_swap_history = SwapHistory::to_swap_history(interval, pool).unwrap();
            match db.swap_history.insert_one(pool_swap_history).await {
                Ok(_record) => println!("Inserted pool swap doc {}",pool),
                Err(e) => return Err(format!("Error inserting swap doc to db {:?}",e))
            }
        }
        Ok(())
    }
    pub async fn fetch_swap_history(db:&DataBase,pool:&str,interval:&str,count:&str,from:&str) -> Result<i64,String>{
        let url = generate_api_url(&pool,&interval, &from, &count);
        println!("url - {}",url);
        let api_response = match reqwest::get(&url).await {
            Ok(res) => res,
            Err(e) => return Err(format!("Failed to fetch data: {}", e))
        };
    
        let raw_body = match api_response.text().await {
            Ok(res) => res,
            Err(e) => return Err(format!("Failed to read response text: {}", e)),
        };
    
        println!("Raw response: {}", raw_body);
    
        let response = match reqwest::get(&url).await {
            Ok(res) => {
                match res.json::<ApiResponse>().await {
                    Ok(res) => res,
                    Err(e) => return Err(format!("Failed to parse JSON response: {}", e)),
                }
            },
            Err(e) => return Err(format!("{}", e.to_string())),
        };        
    
        // Extract end_time and handle any potential errors
        let end_time = match response.meta.end_time.parse::<i64>() {
            Ok(time) => time,
            Err(e) => return Err(format!("Failed to parse end time: {}", e))
        };
        match self::SwapHistory::store_swap_history(db, pool, response).await{
            Ok(_res) => (),
            Err(e) => return Err(format!("{}",e))
        };
        Ok(end_time)
    }
}