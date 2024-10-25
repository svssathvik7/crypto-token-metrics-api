use actix_web::{web::{self, ServiceConfig}, HttpResponse, Responder};
use chrono::Utc;

use crate::{models::{api_request_param_model::{validate_query, QueryParams}, swap_history_model::SwapHistory}, services::db::DataBase};

#[utoipa::path(
    get,
    path = "/swaps",
    params(
        ("from" = Option<u64>, Query, description = "Start time Unix timestamp "),
        ("to" = Option<u64>, Query, description = "End time Unix timestamp"),
        ("pool" = Option<String>, Query, description = "Pool identifier - only BTC.BTC for now"),
        ("page" = Option<u64>, Query, description = "Page number (minimum: 1)"),
        ("limit" = Option<u32>, Query, description = "Items per page (1-400)"),
        ("sort_by" = Option<String>, Query, description = "Field to sort by"),
        ("sort_order" = Option<i8>, Query, description = "1 for ascending order and -1 for descending order"),
        ("interval" = Option<String>, Query, description = "Time interval for aggregation (hour, day, week, month, quarter, year)"),
        ("count" = Option<u32>, Query, description = "Total records that are to be fetched (1-400)")
    ),
    responses(
        (status = 200, description = "List of pool swap history", body = Vec<SwapHistory>),
        (status = 400, description = "Bad request - Invalid parameters"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Swaps History"
)]
#[actix_web::get("")]
pub async fn get_swaps_history(db:web::Data<DataBase>,params:web::Query<QueryParams>) -> HttpResponse{
    if let Err(validation_err) = validate_query(&params) {
        return validation_err;
    }
    match db.get_swaps_history_api(params.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            eprint!("Error at /swaps");
            HttpResponse::InternalServerError().json(e)
        }
    }
}

// Protected route
// Expensive function
#[actix_web::get("/fetch-swaps-all")]
pub async fn fetch_all_swaps_to_db(db:web::Data<DataBase>) -> impl Responder{
    let current_time_stamp = Utc::now().timestamp();
    // epoch value of the api's start derived from the midgard metadata
    let mut start = 1647913096;
    loop {
        let end_time = match SwapHistory::fetch_swap_history(db.get_ref(), "BTC.BTC", "hour", "400", &start.to_string()).await {
            Ok(response) => response,
            Err(e) => {
                println!("Failed to fetch and update db with swap history! {:?}\n",e);
                current_time_stamp+10
            }
        };
        if end_time >= current_time_stamp{
            break;
        }
        start = end_time;
    }
    HttpResponse::Ok().body(format!("Fetched and added swap records to database"))
}

pub fn init(config:&mut ServiceConfig){
    config.service(fetch_all_swaps_to_db).service(get_swaps_history);
    ()
}