use actix_web::{self, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(
        || {
            App::new()
        }
    ).bind("localhost:3000")?.run().await
}