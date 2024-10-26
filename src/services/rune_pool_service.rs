use serde::{Deserialize, Serialize};
use crate::models::{custom_error_model::CustomError, rune_pool_model::RunePool};
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
    pub async fn store_rune_pool(db:&DataBase,data:ApiResponse) -> Result<(),CustomError>{
        for interval in data.intervals{
            match RunePool::try_from(interval) {
                Ok(rune_pool_object) => {
                    match db.rune_pool_history.insert_one(rune_pool_object).await {
                        Ok(_record) => println!("Rune pool record writted to db!"),
                        Err(e) => eprintln!("Err adding rune pool to db {}",e)
                    }
                },
                Err(e) => {
                    return Err(CustomError::DatabaseError(format!("Error parsing interval to rune pool object! {}",e)));
                }
            }
        }
        Ok(())
    }
    pub async fn fetch_rune_pool(db:&DataBase,interval:&str,count:&str,from:&str) -> Result<i64, CustomError>{
        let url = generate_api_url(interval, from, count);
        println!("url - {}",&url);
        let api_response = match reqwest::get(&url).await {
            Ok(res) => res,
            Err(e) => return Err(CustomError::StandardError(format!("Failed to fetch data: {}", e)))
        };
    
        let raw_body = match api_response.text().await {
            Ok(res) => res,
            Err(e) => return Err(CustomError::InvalidInput(format!("Failed to read response text: {}", e))),
        };
    
        println!("Raw response: {}", raw_body);
    
        let response = match reqwest::get(&url).await {
            Ok(res) => {
                match res.json::<ApiResponse>().await {
                    Ok(res) => res,
                    Err(e) => return Err(CustomError::InvalidInput(format!("Failed to parse JSON response: {}", e))),
                }
            },
            Err(e) => return Err(CustomError::StandardError(format!("{}", e.to_string()))),
        };        
    
        // Extract end_time and handle any potential errors
        let end_time = match response.meta.end_time.parse::<i64>() {
            Ok(time) => time,
            Err(e) => return Err(CustomError::StandardError(format!("Failed to parse end time: {}", e)))
        };
        match self::RunePool::store_rune_pool(db, response).await{
            Ok(_res) => (),
            Err(e) => return Err(e)
        };
        Ok(end_time)
    }
}