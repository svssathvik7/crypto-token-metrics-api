use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "crypto-token-metrics-api",
        description = "The API serves as a replicated interface to the Midgard Public API, providing accessible information about the THORChain network and associated chains via Bifröst. By querying the Midgard API, it fetches and processes transaction event data, storing it in a structured format that supports time-series analysis for easy, time-dependent queries. While this API makes THORChain network data available, critical protocol interactions should still be performed directly with THORNode for access to primary blockchain data.",
        version = "1.0.0"
    ),
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