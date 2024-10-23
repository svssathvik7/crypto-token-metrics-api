use serde::{Deserialize, Serialize};

use crate::models::depth_history_model::PoolDepthPriceHistory;
use mongodb::error::Error as mongoError;
use reqwest::Error as reqwestError;
use super::db::DataBase;

// due to volume issues we are sticking to BTC BTC pool type
fn generate_api_url(pool:String,interval:String,from:String,count:String) -> String{
    format!("https://midgard.ninerealms.com/v2/history/depths/{}?interval={}&from={}&count={}",pool,interval,from,count)
}


#[derive(Debug,Deserialize,Serialize)]
pub struct Meta {
    // couldnt follow snake case since that how its in api response of midgard
    pub endAssetDepth: String,
    pub endLPUnits: String,
    pub endMemberCount: String,
    pub endRuneDepth: String,
    pub endSynthUnits: String,
    pub endTime: String,
    pub luviIncrease: String,
    pub priceShiftLoss: String,
    pub startAssetDepth: String,
    pub startLPUnits: String,
    pub startMemberCount: String,
    pub startRuneDepth: String,
    pub startSynthUnits: String,
    pub startTime: String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Interval {
    // couldnt follow snake case since that how its in api response of midgard
    pub assetDepth: String,
    pub assetPrice: String,
    pub assetPriceUSD: String,
    pub endTime: String,
    pub liquidityUnits: String,
    pub luvi: String,
    pub membersCount: String,
    pub runeDepth: String,
    pub startTime: String,
    pub synthSupply: String,
    pub synthUnits: String,
    pub units: String,
}

// imitating the actual midgard api response style
#[derive(Debug,Deserialize,Serialize)]
pub struct ApiResponse{
    // if count > 1 we get vector of interval type objects
    pub intervals : Vec<Interval>,
    pub meta : Meta
}

impl PoolDepthPriceHistory{
    pub async fn store_price_history(db: &DataBase, data: ApiResponse) -> Result<(), mongoError> {
        for interval in data.intervals {
            match PoolDepthPriceHistory::try_from(interval) {
                Ok(pool_history_interval) => {
                    if let Err(e) = db.depth_history.insert_one(pool_history_interval).await {
                        eprint!("Error inserting record: {:?}", e);
                        return Err(e);  // Return error if insertion fails
                    }
                }
                Err(e) => {
                    eprint!("Error writing pool history to db: {:?}", e);
                }
            }
        }
        Ok(())
    }
    pub async fn fetch_price_history(db:&DataBase,pool:String,interval:String,count:String,from:String) -> Result<i64,reqwestError>{
        let url = generate_api_url(pool,interval,from,count);
        println!("{}",url);
        let response = reqwest::get(&url).await?.json::<ApiResponse>().await?;
        // println!("{:?}",response);
        let end_time = response.meta.endTime.clone();
        let end_time = end_time.parse::<i64>().unwrap();
        self::PoolDepthPriceHistory::store_price_history(db,response).await;
        Ok(end_time)
    }
}
