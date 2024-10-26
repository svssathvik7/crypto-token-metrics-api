use actix_web::{web::{self, ServiceConfig}, HttpResponse, Responder};
use chrono::Utc;
use crate::{models::{api_request_param_model::{validate_query, QueryParams}, rune_pool_model::RunePool}, services::db::DataBase};

#[utoipa::path(
    get,
    path = "/runepool",
    params(
        ("from" = Option<u64>, Query, description = "Start time Unix timestamp "),
        ("to" = Option<u64>, Query, description = "End time Unix timestamp"),
        ("page" = Option<u64>, Query, description = "Page number (minimum: 1)"),
        ("limit" = Option<u32>, Query, description = "Items per page (1-400)"),
        ("sort_by" = Option<String>, Query, description = "Field to sort by"),
        ("sort_order" = Option<i8>, Query, description = "1 for ascending order and -1 for descending order"),
        ("interval" = Option<String>, Query, description = "Time interval for aggregation (hour, day, week, month, quarter, year)"),
        ("count" = Option<u32>, Query, description = "Total records that are to be fetched (1-400)")
    ),
    responses(
        (status = 200, description = "List of rune pool history", body = Vec<RunePool>),
        (status = 400, description = "Bad request - Invalid parameters"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Rune Pool History"
)]
#[actix_web::get("")]
pub async fn get_rune_pool_history(db:web::Data<DataBase>,params:web::Query<QueryParams>) -> HttpResponse{
    if let Err(validation_err) = validate_query(&params) {
        return HttpResponse::BadRequest().json(validation_err);
    }
    match db.get_rune_pool_history_api(params.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            eprint!("Error at /runepool {:?}",e);
            HttpResponse::InternalServerError().json(e)
        }
    }
}


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
                println!("Failed to fetch and update db with rune pool history! {:?}\n",e.to_string());
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
    config.service(fetch_all_rune_pools_to_db).service(get_rune_pool_history);
    ()
}