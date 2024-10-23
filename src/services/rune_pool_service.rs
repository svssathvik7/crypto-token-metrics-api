use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::rune_pool_model::RunePool;
use reqwest::Error as reqwestError;
use super::db::DataBase;

fn generate_api_url(interval:&str,from:&str,count:&str) -> String{
    format!("https://midgard.ninerealms.com/v2/history/runepool?interval={}&from={}&count={}",interval,from,count)
}

fn generate_error_text(field_name:&str) -> String{
    format!("Incorrect {} format",field_name)
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
    pub async fn store_rune_pool(db:&DataBase,data:ApiResponse){
        for interval in data.intervals{
            let rune_pool_obj = Self {
                _id : ObjectId::new(),
                count : interval.count.parse::<f64>().expect(&generate_error_text("count")),
                end_time : interval.endTime.parse::<i64>().expect(&generate_error_text("endTime")),
                start_time : interval.startTime.parse::<i64>().expect(&generate_error_text("startTime")),
                units : interval.units.parse::<f64>().expect(&generate_error_text("untis"))
            };
            
        }
    }
    pub async fn fetch_rune_pool(db:&DataBase,interval:&str,count:&str,from:&str) -> Result<i64, reqwestError>{
        let url = generate_api_url(interval, from, count);
        println!("url - {}",&url);
        let response = reqwest::get(&url).await?.json::<ApiResponse>().await?;
        let end_time = response.meta.endTime.clone();
        let end_time = end_time.parse::<i64>().unwrap();
        Ok(end_time)
    }
}