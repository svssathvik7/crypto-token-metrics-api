use std::error::Error;

use reqwest::Error as reqwestError;
use serde::{Deserialize, Serialize};
use mongodb::{bson::oid::ObjectId, error::Error as mongoError};
use crate::models::earning_history_model::{PoolEarningHistory, PoolEarningSummary};

use super::db::DataBase;

fn generate_api_url(interval:&str,from:&str,count:&str) -> String{
    format!("https://midgard.ninerealms.com/v2/history/earnings?interval={}&from={}&count={}",interval,from,count)
}

fn generate_error_text(field_name:&str) -> String{
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
    pub async fn store_earning_history(db: &DataBase, data: ApiResponse) -> Result<(), Box<dyn Error>> {
        for interval in data.intervals {
            let _id = ObjectId::new();
    
            let avg_node_count = interval.avgNodeCount.parse::<f64>().expect(&generate_error_text("avgNodeCount"));
            let block_rewards = interval.blockRewards.parse::<f64>().expect(&generate_error_text("blockRewards"));
            let bonding_earnings = interval.bondingEarnings.parse::<f64>().expect(&generate_error_text("bondingEarnings"));
            let earnings = interval.earnings.parse::<u64>().expect(&generate_error_text("earnings"));
            let end_time = interval.endTime.as_str().parse::<i64>().expect(&generate_error_text("endTime"));
            let liquidity_earnings = interval.liquidityEarnings.parse::<f64>().expect(&generate_error_text("liquidityEarnings"));
            let liquidity_fees = interval.liquidityFees.parse::<u64>().expect(&generate_error_text("liquidityFees"));
            let start_time = interval.startTime.as_str().parse::<i64>().expect(&generate_error_text("startTime"));
            let rune_price_usd = interval.runePriceUSD.parse::<f64>().expect(&generate_error_text("runePriceUSD"));
    
            let pool_earning_summary = PoolEarningSummary {
                _id,
                avg_node_count,
                block_rewards,
                bonding_earnings,
                earnings,
                end_time,
                liquidity_earnings,
                liquidity_fees,
                start_time,
                rune_price_usd,
            };
    
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
                    asset_liquidity_fees: pool.assetLiquidityFees.parse::<f64>().expect(&generate_error_text("assetLiquidityFees")),
                    earning: pool.earnings.parse::<u64>().expect(&generate_error_text("earnings")),
                    rewards: pool.rewards.parse::<f64>().expect(&generate_error_text("rewards")),
                    rune_liquidity_fees: pool.runeLiquidityFees.parse::<f64>().expect(&generate_error_text("runeLiquidityFees")),
                    saver_earning: pool.saverEarning.parse::<f64>().expect(&generate_error_text("saverEarning")),
                    total_liquidity_fees_rune: pool.totalLiquidityFeesRune.parse::<f64>().expect(&generate_error_text("totalLiquidityFeesRune")),
                    start_time: interval.startTime.as_str().parse::<i64>().expect(&generate_error_text("startTime")),
                    end_time: interval.endTime.as_str().parse::<i64>().expect(&generate_error_text("endTime")),
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
        let response: ApiResponse = reqwest::get(&url).await?.json::<ApiResponse>().await?;
        // println!("{:?}",response);
        let end_time = response.meta.endTime.clone();
        let end_time = end_time.parse::<i64>().unwrap();
        self::PoolEarningHistory::store_earning_history(db, response).await;
        Ok(end_time)
    }
}