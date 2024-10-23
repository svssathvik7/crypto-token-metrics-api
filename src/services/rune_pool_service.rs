use serde::{Deserialize, Serialize};

use crate::models::rune_pool_model::RunePool;
use reqwest::Error as reqwestError;
use super::db::DataBase;

fn generate_api_url(interval:&str,from:&str,count:&str) -> String{
    format!("https://midgard.ninerealms.com/v2/history/runepool?interval={}&from={}&count={}",interval,from,count)
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Meta{
    pub endCount: String,
    pub endTime: String,
    pub endUnits: String,
    pub startCount: String,
    pub startTime: String,
    pub startUnits: String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Interval{
    pub count: String,
    pub endTime: String,
    pub startTime: String,
    pub units: String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ApiResponse{
    pub meta : Meta,
    pub intervals : Vec<Interval>
}

impl RunePool{
    pub async fn fetch_rune_pool(db:&DataBase,interval:&str,count:&str,from:&str) -> Result<i64, reqwestError>{
        let url = generate_api_url(interval, from, count);
        println!("url - {}",&url);
        let response = reqwest::get(&url).await?.json::<ApiResponse>().await?;
        let end_time = response.meta.endTime.clone();
        let end_time = end_time.parse::<i64>().unwrap();
        Ok(end_time)
    }
}