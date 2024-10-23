use actix_web::{self, web::Data, App, HttpServer};
use routes::{depth_route::fetch_all_depths_to_db, earning_route::fetch_all_earnings_to_db, rune_pool_route::fetch_all_rune_pools_to_db, swap_route::fetch_all_swaps_to_db};
use services::db::DataBase;
pub mod services;
pub mod models;
pub mod routes;
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let data_base = DataBase::init().await;
    let db_data = Data::new(data_base);
    print!("Connected to DB\n");
    HttpServer::new(
        move || {
            App::new().app_data(db_data.clone()).service(fetch_all_depths_to_db).service(fetch_all_earnings_to_db).service(fetch_all_swaps_to_db).service(fetch_all_rune_pools_to_db)
        }
    ).bind("localhost:3000")?.run().await
}