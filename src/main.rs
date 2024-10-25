#![recursion_limit = "256"]

use actix_web::{self, web::{scope, Data}, App, HttpServer};
use utoipa::{openapi, OpenApi};
use routes::{depth_route, earning_route::{self}, rune_pool_route, swap_route::{self}};
use services::{db::DataBase, fetch_all_cron_service::run_cron_job};
use utoipa_swagger_ui::SwaggerUi;
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


    #[derive(OpenApi)]
    #[openapi(
        paths(
            crate::routes::depth_route::get_depth_price_history,
            crate::routes::swap_route::get_swaps_history,
            crate::routes::earning_route::get_earnings_history,
            crate::routes::rune_pool_route::get_rune_pool_history
        ),
        components(schemas(
            crate::models::depth_history_model::PoolDepthPriceHistory,
            crate::models::rune_pool_model::RunePool,
            crate::models::swap_history_model::SwapHistory,
            crate::models::earning_history_model::PoolEarningSummary,
            crate::models::api_request_param_model::QueryParams
        ))
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    HttpServer::new(
        move || {
            App::new().app_data(db_data.clone())
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()))
            .service(scope("/depths").configure(depth_route::init))
            .service(scope("/earnings").configure(earning_route::init))
            .service(scope("/swaps").configure(swap_route::init))
            .service(scope("/runepool").configure(rune_pool_route::init))
        }
    ).bind("localhost:3000")?.run().await
}