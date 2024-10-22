use actix_web::{self, web::Data, App, HttpServer};
use routes::depth_route::fetch_all_depths_to_db;
use services::db::DataBase;
pub mod services;
pub mod models;
pub mod routes;
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let data_base = DataBase::init().await;
    let db_data = Data::new(data_base);
    print!("Connected to DB");
    HttpServer::new(
        move || {
            App::new().app_data(db_data.clone()).service(fetch_all_depths_to_db)
        }
    ).bind("localhost:3000")?.run().await
}