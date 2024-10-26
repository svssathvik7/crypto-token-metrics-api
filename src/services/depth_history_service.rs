use serde::{Deserialize, Serialize};

use crate::models::depth_history_model::PoolDepthPriceHistory;
use super::db::DataBase;

// due to volume issues we are sticking to BTC BTC pool type in depths fetch
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

// imitating the actual midgard api response style to parse the fetched data
#[derive(Debug,Deserialize,Serialize)]
pub struct ApiResponse{
    pub intervals : Vec<Interval>,
    pub meta : Meta
}

impl PoolDepthPriceHistory{
    pub async fn store_price_history(db: &DataBase, data: ApiResponse) -> Result<(),String>{
        for interval in data.intervals {
            match PoolDepthPriceHistory::try_from(interval) {
                Ok(pool_history_interval) => {
                    if let Err(e) = db.depth_history.insert_one(pool_history_interval).await {
                        eprint!("Error inserting record: {:?}", e);
                    }
                },
                Err(e) => {
                    return Err(format!("Error writing pool history to db: {:?}", e));
                }
            }
        }
        Ok(())
    }
    pub async fn fetch_price_history(db:&DataBase,pool:&str,interval:&str,count:&str,from:&str) -> Result<i64,String>{
        let url = generate_api_url(&pool,&interval,&from,&count);
        println!("{}",url);
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
        // println!("{:?}",response);
        match self::PoolDepthPriceHistory::store_price_history(db,response).await{
            Ok(_res) => (),
            Err(e) => return Err(format!("{}",e))
        };
        // println!("{}","in");
        Ok(end_time)
    }
}
