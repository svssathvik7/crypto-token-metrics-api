use actix_web::{web::{self, ServiceConfig}, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::{models::depth_history_model::PoolDepthPriceHistory, services::db::DataBase};

#[derive(Debug,Serialize,Deserialize)]
pub struct QueryParams{
    pub pool : Option<String>,
    pub interval : Option<String>,
    pub count : Option<u32>,
    pub to : Option<u64>,
    pub from : Option<u64>,
    pub page : Option<u64>,
    pub sort_by : Option<String>,
    pub sort_order : Option<i8>,
    pub limit : Option<i8> 
}

fn validate_query(query: &QueryParams) -> Result<(), HttpResponse> {

    // for now due to volume and computation constraints our depth history is confined to only BTC.BTC pool
    if let Some(ref pool) = (query.pool) {
        if pool != "BTC.BTC"{
            return Err(HttpResponse::BadRequest().json("Due to volume and computation constraints our depth history is confined to only BTC.BTC pool"));
        }
    }
    // Check start_time and end_time
    if let (Some(start), Some(end)) = (query.from, query.to) {
        if start >= end {
            return Err(HttpResponse::BadRequest().json("start_time must be less than end_time."));
        }
    }

    // Validate page
    let page = query.page.unwrap_or(1).max(1);
    if page < 1 {
        return Err(HttpResponse::BadRequest().json("page must be a positive integer."));
    }

    // Validate interval
    let valid_interval = vec!["hour", "day", "week", "month", "quarter", "year"];
    if let Some(ref interval) = query.interval {
        if !valid_interval.contains(&interval.as_str()) {
            return Err(HttpResponse::BadRequest().json(format!(
                "interval must be one of: {:?}",
                valid_interval
            )));
        }
    }

    Ok(())
}

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