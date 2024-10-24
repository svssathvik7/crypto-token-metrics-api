use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{models::depth_history_model::PoolDepthPriceHistory, utils::db_helper_utils::get_max_start_time_of_collection};
use mongodb::error::Error as mongoError;
use reqwest::Error as reqwestError;
use super::db::DataBase;

// due to volume issues we are sticking to BTC BTC pool type
fn generate_api_url(pool:&str,interval:&str,from:&str,count:&str) -> String{
    format!("https://midgard.ninerealms.com/v2/history/depths/{}?interval={}&from={}&count={}",pool,interval,from,count)
}


#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all="camelCase")]
pub struct Meta {
    pub end_asset_depth: String,
    #[serde(rename="endLPUnits")]
    pub end_lp_units: String,
    pub end_member_count: String,
    pub end_rune_depth: String,
    pub end_synth_units: String,
    pub end_time: String,
    pub luvi_increase: String,
    pub price_shift_loss: String,
    pub start_asset_depth: String,
    #[serde(rename="startLPUnits")]
    pub start_lp_units: String,
    pub start_member_count: String,
    pub start_rune_depth: String,
    pub start_synth_units: String,
    pub start_time: String,
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Interval {
    pub asset_depth: String,
    pub asset_price: String,
    #[serde(rename="assetPriceUSD")]
    pub asset_price_usd: String,
    pub end_time: String,
    pub liquidity_units: String,
    pub luvi: String,
    pub members_count: String,
    pub rune_depth: String,
    pub start_time: String,
    pub synth_supply: String,
    pub synth_units: String,
    pub units: String,
}

// imitating the actual midgard api response style
#[derive(Debug,Deserialize,Serialize)]
pub struct ApiResponse{
    pub intervals : Vec<Interval>,
    pub meta : Meta
}

impl PoolDepthPriceHistory{
    pub async fn store_price_history(db: &DataBase, data: ApiResponse){
        for interval in data.intervals {
            match PoolDepthPriceHistory::try_from(interval) {
                Ok(pool_history_interval) => {
                    if let Err(e) = db.depth_history.insert_one(pool_history_interval).await {
                        eprint!("Error inserting record: {:?}", e);
                    }
                }
                Err(e) => {
                    eprint!("Error writing pool history to db: {:?}", e);
                }
            }
        }
        ()
    }
    pub async fn fetch_price_history(db:&DataBase,pool:&str,interval:&str,count:&str,from:&str) -> Result<i64,reqwestError>{
        let url = generate_api_url(&pool,&interval,&from,&count);
        println!("{}",url);
        let response = reqwest::get(&url).await?.json::<ApiResponse>().await?;
        println!("{:?}",response);
        let end_time = response.meta.end_time.clone();
        let end_time = end_time.parse::<i64>().unwrap();
        self::PoolDepthPriceHistory::store_price_history(db,response).await;
        println!("{}","in");
        Ok(end_time)
    }
}
