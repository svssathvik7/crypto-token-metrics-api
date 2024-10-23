use actix_web::{web::{self, ServiceConfig}, HttpResponse, Responder};
use chrono::Utc;

use crate::{models::rune_pool_model::RunePool, services::db::DataBase};
// Protected route
// Expensive function
#[actix_web::get("/fetch-rune-pools-all")]
async fn fetch_all_rune_pools_to_db(db:web::Data<DataBase>) -> impl Responder{
    let current_time_stamp = Utc::now().timestamp();
    // epoch value of the api's start derived from the midgard metadata
    let mut start = 1647913096;
    loop{
        let end_time = match RunePool::fetch_rune_pool(&db, "hour", "400", &start.to_string()).await {
            Ok(response) => response,
            Err(e) => {
                println!("Failed to fetch and update db with rune pool history! {:?}\n",e);
                current_time_stamp+10
            }
        };
        if end_time >= current_time_stamp{
            break;
        }
        start = end_time;
    }
    HttpResponse::Ok().body(format!("Fetched and added rune pool records to database"))
}

pub fn init(config:&mut ServiceConfig){
    config.service(fetch_all_rune_pools_to_db);
    ()
}