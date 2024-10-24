use std::error::Error;

use chrono::Utc;
use reqwest::Error as reqwestError;
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use crate::{models::earning_history_model::{PoolEarningHistory, PoolEarningSummary}, utils::db_helper_utils::get_max_start_time_of_collection};

use super::db::DataBase;

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

#[derive(Debug,Serialize,Deserialize)]
pub struct ApiResponse{
    pub meta : Meta,
    pub intervals : Vec<Interval>
}

impl PoolEarningHistory{
    pub async fn store_earning_history(db: &DataBase, data: ApiResponse) -> Result<(), Box<dyn Error>> {
        for interval in data.intervals {
            let _id = ObjectId::new();
    
            let avg_node_count = interval.avg_node_count.parse::<f64>().expect(&generate_error_text("avgNodeCount"));
            let block_rewards = interval.block_rewards.parse::<f64>().expect(&generate_error_text("blockRewards"));
            let bonding_earnings = interval.bonding_earnings.parse::<f64>().expect(&generate_error_text("bondingEarnings"));
            let earnings = interval.earnings.parse::<u64>().expect(&generate_error_text("earnings"));
            let end_time = interval.end_time.as_str().parse::<i64>().expect(&generate_error_text("endTime"));
            let liquidity_earnings = interval.liquidity_earnings.parse::<f64>().expect(&generate_error_text("liquidityEarnings"));
            let liquidity_fees = interval.liquidity_fees.parse::<u64>().expect(&generate_error_text("liquidityFees"));
            let start_time = interval.start_time.as_str().parse::<i64>().expect(&generate_error_text("startTime"));
            let rune_price_usd = interval.rune_price_usd.parse::<f64>().expect(&generate_error_text("runePriceUSD"));
    
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
                    asset_liquidity_fees: pool.asset_liquidity_fees.parse::<f64>().expect(&generate_error_text("assetLiquidityFees")),
                    earning: pool.earnings.parse::<u64>().expect(&generate_error_text("earnings")),
                    rewards: pool.rewards.parse::<f64>().expect(&generate_error_text("rewards")),
                    rune_liquidity_fees: pool.rune_liquidity_fees.parse::<f64>().expect(&generate_error_text("runeLiquidityFees")),
                    saver_earning: pool.saver_earning.parse::<f64>().expect(&generate_error_text("saverEarning")),
                    total_liquidity_fees_rune: pool.total_liquidity_fees_rune.parse::<f64>().expect(&generate_error_text("totalLiquidityFeesRune")),
                    start_time: interval.start_time.as_str().parse::<i64>().expect(&generate_error_text("startTime")),
                    end_time: interval.end_time.as_str().parse::<i64>().expect(&generate_error_text("endTime")),
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
        let from_time:i64 = from.parse().unwrap_or(0);
        if from_time >= get_max_start_time_of_collection(&db.depth_history).await.unwrap_or(Utc::now().timestamp()) as i64{
            eprint!("Can't access future timestamps!");
            Ok(Utc::now().timestamp())
        }
        else{
            let response: ApiResponse = reqwest::get(&url).await?.json::<ApiResponse>().await?;
            // println!("{:?}",response);
            let end_time = response.meta.end_time.clone();
            let end_time = end_time.parse::<i64>().unwrap();
            self::PoolEarningHistory::store_earning_history(db, response).await;
            Ok(end_time)
        }
    }
}