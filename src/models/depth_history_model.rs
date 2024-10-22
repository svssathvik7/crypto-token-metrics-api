use std::error::Error as stdError;
use actix_web::{http::Error, Error as actixError};
use mongodb::{action::InsertOne, bson::oid::ObjectId};
use mongodb::error::Error as mongoError;
use reqwest::Error as reqwestError;
use serde::{Deserialize, Serialize};
use crate::services::{db::DataBase, depth_history_service::{ApiResponse, Interval}};
// due to volume issues we are sticking to BTC BTC pool type
fn generate_api_url(pool:String,interval:String,from:String,count:String) -> String{
    format!("https://midgard.ninerealms.com/v2/history/depths/{}?interval={}&from={}&count={}",pool,interval,from,count)
}

fn generate_error_text(field_name:String) -> String{
    format!("Incorrect {} format",field_name)
}

#[derive(Deserialize,Serialize,Debug)]
pub struct PoolDepthPriceHistory{
    pub _id : ObjectId,
    pub name : String,
    pub asset_depth : i64,
    pub asset_price : f64,
    pub asset_price_usd : f64,
    pub end_time : i64,
    pub liquidity_units : i64,
    pub luvi : f64,
    pub members_count : i32,
    pub rune_depth : i64,
    pub start_time : i64,
    pub synth_supply : i64,
    pub synth_units : i64,
    pub units : i64
}

impl TryFrom<Interval> for PoolDepthPriceHistory{
    type Error = Box<dyn stdError>;
    fn try_from(value: Interval) -> Result<Self, Self::Error> {
        let _id = ObjectId::new();
        let name = String::from("BTC.BTC");
        let asset_depth = value.assetDepth.parse::<i64>().expect(&generate_error_text(value.assetDepth));
        let asset_price = value.assetPrice.parse::<f64>().expect(&generate_error_text(value.assetPrice));
        let asset_price_usd = value.assetPriceUSD.parse::<f64>().expect(&generate_error_text(value.assetPriceUSD));
        let end_time = value.endTime.parse::<i64>().expect(&generate_error_text(value.endTime));
        let liquidity_units = value.liquidityUnits.parse::<i64>().expect(&generate_error_text(value.liquidityUnits));
        let luvi = value.luvi.parse::<f64>().expect(&generate_error_text(value.luvi));
        let members_count = value.membersCount.parse::<i32>().expect(&generate_error_text(value.membersCount));
        let rune_depth = value.runeDepth.parse::<i64>().expect(&generate_error_text(value.runeDepth));
        let start_time = value.startTime.parse::<i64>().expect(&generate_error_text(value.startTime));
        let synth_supply = value.synthSupply.parse::<i64>().expect(&generate_error_text(value.synthSupply));
        let synth_units = value.synthUnits.parse::<i64>().expect(&generate_error_text(value.synthUnits));
        let units = value.units.parse::<i64>().expect(&generate_error_text(value.units));
        let pool_price_document = PoolDepthPriceHistory {
            _id,
            name,
            asset_depth,
            asset_price,
            asset_price_usd,
            end_time,
            liquidity_units,
            luvi,
            members_count,
            rune_depth,
            start_time,
            synth_supply,
            synth_units,
            units,
        };
        Ok(pool_price_document)
    }
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
