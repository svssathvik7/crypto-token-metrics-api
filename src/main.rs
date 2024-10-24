#![recursion_limit = "256"]

use actix_web::{self, web::{scope, Data}, App, HttpServer};

use routes::{depth_route, earning_route::{self}, rune_pool_route, swap_route::{self}};
use services::{db::DataBase, fetch_all_cron_service::run_cron_job};
pub mod services;
pub mod models;
pub mod routes;
pub mod utils;
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let data_base = DataBase::init().await;
    let db_data = Data::new(data_base);
    actix_web::rt::spawn(run_cron_job(db_data.clone(), "BTC.BTC"));
    println!("Connected to DB\n");
    HttpServer::new(
        move || {
            App::new().app_data(db_data.clone()).service(scope("/depths").configure(depth_route::init)).service(scope("/earnings").configure(earning_route::init)).service(scope("/swaps").configure(swap_route::init)).service(scope("/runepool").configure(rune_pool_route::init))
        }
    ).bind("localhost:3000")?.run().await
}