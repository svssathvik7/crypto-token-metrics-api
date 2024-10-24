use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use crate::{models::earning_history_model::PoolEarningHistory, services::db::DataBase};


#[actix_web::get("/fetch-earnings-all")]
pub async fn fetch_all_earnings_to_db(db:web::Data<DataBase>) -> HttpResponse{

    let current_time_stamp = Utc::now().timestamp();
    let mut start = 1647913096;
    loop {
        let end_time = match PoolEarningHistory::fetch_earning_history(db.get_ref(), "hour", "400", &(start).to_string()).await {
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
    HttpResponse::Ok().body(format!("Fetched and added earnings to database"))
}

pub fn init(config:&mut web::ServiceConfig){
    config.service(fetch_all_earnings_to_db);
    ()
}