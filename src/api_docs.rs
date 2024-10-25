use utoipa::OpenApi;

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
        crate::models::earning_history_model::PoolEarningHistory,
        crate::models::earning_history_model::PoolEarningSummary,
        crate::models::api_request_param_model::QueryParams
    ))
)]
pub struct ApiDoc;