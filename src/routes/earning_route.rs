use actix_web::{web, HttpResponse};
use chrono::Utc;
use crate::{models::{api_request_param_model::{validate_query, QueryParams}, earning_history_model::PoolEarningHistory}, services::db::DataBase};


#[utoipa::path(
    get,
    path = "/earnings",
    params(
        ("from" = Option<u64>, Query, description = "Start time Unix timestamp "),
        ("to" = Option<u64>, Query, description = "End time Unix timestamp"),
        ("pool" = Option<String>, Query, description = "Pool identifier like BTC.BTC"),
        ("page" = Option<u64>, Query, description = "Page number (minimum: 1)"),
        ("limit" = Option<u32>, Query, description = "Items per page (1-400)"),
        ("sort_by" = Option<String>, Query, description = "Field to sort by"),
        ("sort_order" = Option<i8>, Query, description = "1 for ascending order and -1 for descending order"),
        ("interval" = Option<String>, Query, description = "Time interval for aggregation (hour, day, week, month, quarter, year)"),
        ("count" = Option<u32>, Query, description = "Due to high volume (7Lakh+ records) free tier supports only 1 interval output with 26 pools")
    ),
    responses(
        (status = 200, description = "List of pool earnings", body = Vec<PoolEarningHistory>),
        (status = 400, description = "Bad request - Invalid parameters"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Earnings History"
)]
#[actix_web::get("")]
pub async fn get_earnings_history(db:web::Data<DataBase>,params:web::Query<QueryParams>) -> HttpResponse{
    if let Err(validation_err) = validate_query(&params) {
        return HttpResponse::BadRequest().json(validation_err);
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
    // adapting non-strict fetch, error at a bucket doesn't stop whole fetch process
    // because observed failure due to network multiple times with midgard while testing
    HttpResponse::Ok().body(format!("Fetched and added earnings to database"))
}

pub fn init(config:&mut web::ServiceConfig){
    config.service(fetch_all_earnings_to_db).service(get_earnings_history);
    ()
}