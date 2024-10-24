use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{models::rune_pool_model::RunePool, utils::db_helper_utils::get_max_start_time_of_collection};
use reqwest::Error as reqwestError;
use super::db::DataBase;

fn generate_api_url(interval:&str,from:&str,count:&str) -> String{
    format!("https://midgard.ninerealms.com/v2/history/runepool?interval={}&from={}&count={}",interval,from,count)
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Meta{
    pub end_count: String,
    pub end_time: String,
    pub end_units: String,
    pub start_count: String,
    pub start_time: String,
    pub start_units: String
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Interval{
    pub count: String,
    pub end_time: String,
    pub start_time: String,
    pub units: String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ApiResponse{
    pub meta : Meta,
    pub intervals : Vec<Interval>
}

impl RunePool{
    pub async fn store_rune_pool(db:&DataBase,data:ApiResponse){
        for interval in data.intervals{
            match RunePool::try_from(interval) {
                Ok(rune_pool_object) => {
                    match db.rune_pool_history.insert_one(rune_pool_object).await {
                        Ok(_record) => println!("Rune pool record writted to db!"),
                        Err(e) => eprint!("Err adding rune pool to db {}",e)
                    }
                },
                Err(e) => {
                    eprint!("Error parsing interval to rune pool object! {}",e)
                }
            }
        }
    }
    pub async fn fetch_rune_pool(db:&DataBase,interval:&str,count:&str,from:&str) -> Result<i64, reqwestError>{
        let url = generate_api_url(interval, from, count);
        println!("url - {}",&url);
        let from_time:i64 = from.parse().unwrap_or(0);
        if from_time >= get_max_start_time_of_collection(&db.depth_history).await.unwrap_or(Utc::now().timestamp()) as i64{
            eprint!("Can't access future timestamps!");
            Ok(Utc::now().timestamp())
        }
        else{
            let response = reqwest::get(&url).await?.json::<ApiResponse>().await?;
            let end_time = response.meta.end_time.clone();
            let end_time = end_time.parse::<i64>().unwrap();
            self::RunePool::store_rune_pool(db, response).await;
            Ok(end_time)
        }
    }
}