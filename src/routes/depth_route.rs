use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;

use crate::{models::depth_history_model::PoolDepthPriceHistory, services::db::DataBase};

// Protected route
// Expensive function
//1647913096
#[actix_web::get("/fetch-depths-all")]
pub async fn fetch_all_depths_to_db(db:web::Data<DataBase>) -> impl Responder{
    let current_time_stamp = Utc::now().timestamp();
    let mut start = current_time_stamp - 3600;
    loop {
        let end_time = match PoolDepthPriceHistory::fetch_price_history(db.get_ref(), String::from("BTC.BTC"), String::from("hour"), String::from("400"), start.to_string()).await {
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
    HttpResponse::Ok().body(format!("Fetched and added records to database"))
}