use std::error::Error;
use reqwest::Error as reqwestError;
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use crate::{models::earning_history_model::{PoolEarningHistory, PoolEarningSummary}, parse_field};

use super::db::DataBase;

// earnings history is designed to fetch data of all pool types (around 8L+ records)
fn generate_api_url(interval:&str,from:&str,count:&str) -> String{
    format!("https://midgard.ninerealms.com/v2/history/earnings?interval={}&from={}&count={}",interval,from,count)
}

fn generate_error_text(field_name:&str) -> String{
    format!("Incorrect {} format",field_name)
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Pool{
    pub pool: String,
    pub asset_liquidity_fees: String,
    pub rune_liquidity_fees: String,
    pub total_liquidity_fees_rune: String,
    pub saver_earning: String,
    pub rewards: String,
    pub earnings: String,
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Meta{
    pub avg_node_count: String,
    pub block_rewards: String,
    pub bonding_earnings: String,
    pub earnings: String,
    pub end_time: String,
    pub liquidity_earnings: String,
    pub liquidity_fees: String,
    pub pools: Vec<Pool>,
    #[serde(rename="runePriceUSD")]
    pub rune_price_usd: String,
    pub start_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Interval {
    pub start_time: String,
    pub end_time: String,
    pub avg_node_count: String,
    pub block_rewards: String,
    pub bonding_earnings: String,
    pub earnings: String,
    pub liquidity_earnings: String,
    pub liquidity_fees: String,
    #[serde(rename="runePriceUSD")]
    pub rune_price_usd: String,
    pub pools: Vec<Pool>,
}

// Imitate the actual midgard response style to parse data while fetching
#[derive(Debug,Serialize,Deserialize)]
pub struct ApiResponse{
    pub meta : Meta,
    pub intervals : Vec<Interval>
}

impl PoolEarningHistory{
    pub async fn store_earning_history(db: &DataBase, data: ApiResponse) -> Result<(), Box<dyn Error>> {
        for interval in data.intervals {
            // iterate over each pool data in the interval of API Response
            let pool_earning_summary = PoolEarningSummary {
                _id: ObjectId::new(),
                avg_node_count: parse_field!(interval, avg_node_count, f64),
                block_rewards: parse_field!(interval, block_rewards, f64),
                bonding_earnings: parse_field!(interval, bonding_earnings, f64),
                earnings: parse_field!(interval, earnings, u64),
                end_time: parse_field!(interval, end_time, i64),
                liquidity_earnings: parse_field!(interval, liquidity_earnings, f64),
                liquidity_fees: parse_field!(interval, liquidity_fees, u64),
                start_time: parse_field!(interval, start_time, i64),
                rune_price_usd: parse_field!(interval, rune_price_usd, f64),
            };
            // collect the earnings summary of interval result
            let earnings_summary_id = db.earnings_summary.insert_one(pool_earning_summary).await.unwrap().inserted_id;
            let mut check = true;
            for pool in interval.pools {
                if check{
                    println!("{:?}",pool);
                }
                check = false;
                let pool_earnings = PoolEarningHistory {
                    _id: ObjectId::new(),
                    pool: pool.pool.clone(),
                    asset_liquidity_fees: parse_field!(pool, asset_liquidity_fees, f64),
                    earning: parse_field!(pool, earnings, u64),
                    rewards: parse_field!(pool, rewards, f64),
                    rune_liquidity_fees: parse_field!(pool, rune_liquidity_fees, f64),
                    saver_earning: parse_field!(pool, saver_earning, f64),
                    total_liquidity_fees_rune: parse_field!(pool, total_liquidity_fees_rune, f64),
                    start_time: parse_field!(interval, start_time, i64),
                    end_time: parse_field!(interval, end_time, i64),
                    earnings_summary: earnings_summary_id.as_object_id().expect(&generate_error_text("earning summary id")),
                };
    
                match db.earnings.insert_one(pool_earnings).await {
                    Ok(_rec) => {
                        println!("Successfully inserted earnings history to db");
                    }
                    Err(_e) => eprint!("Failed inserting earnings history"),
                }
            }
        }
        Ok(())
    }    
    pub async fn fetch_earning_history(db:&DataBase,interval:&str,count:&str,from:&str) -> Result<i64,reqwestError>{
        let url = generate_api_url(interval, from, count);
        print!("url - {}",url);
        let api_response = reqwest::get(&url).await?;
        let raw_body = api_response.text().await?;

        println!("Raw response: {}", raw_body);

        let response = reqwest::get(&url).await?.json::<ApiResponse>().await?;
        // println!("{:?}",response);
        let end_time = response.meta.end_time.clone();
        let end_time = end_time.parse::<i64>().unwrap();
        self::PoolEarningHistory::store_earning_history(db, response).await;
        Ok(end_time)
    }
}