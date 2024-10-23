use actix_web::{web::{self, ServiceConfig}, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::{models::depth_history_model::PoolDepthPriceHistory, services::db::DataBase};

#[derive(Debug,Serialize,Deserialize)]
pub struct QueryParams{
    pub pool : Option<String>,
    pub interval : Option<String>,
    pub count : Option<u16>,
    pub to : Option<u64>,
    pub from : Option<u64>
}


// Protected route
// Expensive function
#[actix_web::get("/fetch-depths-all")]
pub async fn fetch_all_depths_to_db(db:web::Data<DataBase>) -> impl Responder{
    let current_time_stamp = Utc::now().timestamp();
    // epoch value of the api's start derived from the midgard metadata
    let mut start = 1647913096;
    loop {
        let end_time = match PoolDepthPriceHistory::fetch_price_history(db.get_ref(), "BTC.BTC", "hour", "400", &start.to_string()).await {
            Ok(response) => response,
            Err(e) => {
                println!("Failed to fetch and update db with fetch price history! {:?}\n",e);
                current_time_stamp+10
            }
        };
        if end_time >= current_time_stamp{
            break;
        }
        start = end_time;
    }
    HttpResponse::Ok().body(format!("Fetched and added depth records to database"))
}

pub fn init(config:&mut ServiceConfig){
    config.service(fetch_all_depths_to_db);
    ()
}