use actix_web::{web::{self, ServiceConfig}, HttpResponse, Responder};
use chrono::Utc;
use crate::{models::{api_request_param_model::{validate_query, QueryParams}, depth_history_model::PoolDepthPriceHistory}, services::db::DataBase};

#[utoipa::path(
    get,
    path = "/depths",
    params(
        ("pool" = Option<String>, Query, description = "Represents the token type"),
        ("interval" = Option<String>, Query, description = "Time interval for the data (e.g., day, week, month, quarter, year). Defaults to hour if not provided."),
        ("from" = Option<f64>, Query, description = "Start time for fetching data in Unix timestamp format."),
        ("to" = Option<f64>, Query, description = "End time for fetching data in Unix timestamp format. Defaults to 1729666800.0 if not provided."),
        ("count" = Option<i64>, Query, description = "Limits the number of documents in the response."),
        ("page" = Option<i64>, Query, description = "Page number for pagination. Defaults to 1 if not provided."),
        ("sort_by" = Option<String>, Query, description = "Field by which to sort the results (e.g., timestamp, price). Defaults to startTime if not provided."),
        ("limit" = Option<i16>, Query, description = "Works for pagination by controlling the page size, has a fallback to count")
    ),
    responses(
        (status = 200, description = "Successfully fetched depth history data", body = Vec<PoolDepthPriceHistory>),
        (status = 400, description = "Bad request!"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Depth and Price History",
    operation_id = "fetchDepthHistoryData"
)]

#[actix_web::get("")]
pub async fn get_depth_price_history(db:web::Data<DataBase>,params:web::Query<QueryParams>) -> HttpResponse{
    if let Err(validation_err) = validate_query(&params) {
        return validation_err;
    }
    match db.get_depth_price_history_api(params.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            eprint!("Error at /depths {:?}",e);
            HttpResponse::InternalServerError().json(e)
        }
    }
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
    config.service(fetch_all_depths_to_db).service(get_depth_price_history);
    ()
}