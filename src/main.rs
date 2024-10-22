use actix_web::{self, web::Data, App, HttpServer};
use services::db::DataBase;
pub mod services;
pub mod models;
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let data_base = DataBase::init().await;
    let db_data = Data::new(data_base);
    print!("Connected to DB");
    HttpServer::new(
        move || {
            App::new().app_data(db_data.clone())
        }
    ).bind("localhost:3000")?.run().await
}