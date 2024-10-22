use reqwest::Error as reqwestError;
use serde::{Deserialize, Serialize};

use crate::models::earning_history_model::PoolEarningHistory;

use super::db::DataBase;

fn generate_api_url(interval:String,from:String,count:String) -> String{
    format!("https://midgard.ninerealms.com/v2/history/earnings?interval={}&from={}&count={}",interval,from,count)
}

fn generate_error_text(field_name:String) -> String{
    format!("Incorrect {} format",field_name)
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Pool{
    pub pool: String,
    pub assetLiquidityFees: String,
    pub runeLiquidityFees: String,
    pub totalLiquidityFeesRune: String,
    pub saverEarning: String,
    pub rewards: String,
    pub earnings: String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Meta{
    pub avgNodeCount: String,
    pub blockRewards: String,
    pub bondingEarnings: String,
    pub earnings: String,
    pub endTime: String,
    pub liquidityEarnings: String,
    pub liquidityFees: String,
    pub pools: Vec<Pool>,
    pub runePriceUSD: String,
    pub startTime: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Interval {
    pub startTime: String,
    pub endTime: String,
    pub avgNodeCount: String,
    pub blockRewards: String,
    pub bondingEarnings: String,
    pub earnings: String,
    pub liquidityEarnings: String,
    pub liquidityFees: String,
    pub runePriceUSD: String,
    pub pools: Vec<Pool>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ApiResponse{
    pub meta : Meta,
    pub intervals : Vec<Interval>
}

impl PoolEarningHistory{
    pub async fn fetch_earning_history(db:&DataBase,interval:String,count:String,from:String) -> Result<i64,reqwestError>{
        let url = generate_api_url(interval, from, count);
        print!("{}",url);
        let response: ApiResponse = reqwest::get(&url).await?.json::<ApiResponse>().await?;
        println!("{:?}",response);
        let end_time = response.meta.endTime;
        let end_time = end_time.parse::<i64>().unwrap();
        Ok(end_time)
    }
}