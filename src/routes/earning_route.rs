use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use crate::{models::{api_request_param_model::{validate_query, QueryParams}, earning_history_model::PoolEarningHistory}, services::db::DataBase};

#[actix_web::get("")]
pub async fn get_earnings_history(db:web::Data<DataBase>,params:web::Query<QueryParams>) -> HttpResponse{
    if let Err(validation_err) = validate_query(&params) {
        return validation_err;
    }
    match db.get_pool_earnings_history_api(params.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            eprint!("Error at /earnings {:?}",e);
            HttpResponse::InternalServerError().json(e)
        }
    }
}

#[actix_web::get("/fetch-earnings-all")]
pub async fn fetch_all_earnings_to_db(db:web::Data<DataBase>) -> HttpResponse{

    let current_time_stamp = Utc::now().timestamp();
    let mut start = 1647913096;
    loop {
        let end_time = match PoolEarningHistory::fetch_earning_history(db.get_ref(), "hour", "400", &(start).to_string()).await {
            Ok(response) => response,
            Err(e) => {
                println!("Failed to fetch and update db with earnings price history! {:?}\n",e);
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
    config.service(fetch_all_earnings_to_db).service(get_earnings_history);
    ()
}