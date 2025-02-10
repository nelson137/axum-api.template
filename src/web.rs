use anyhow::Result;
use axum::{Router, routing};
use axum_prometheus::PrometheusMetricLayerBuilder;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

mod response;
mod routes;
mod service;
mod state;

mod tags {
    pub const HEALTH: &str = "health";
    pub const HELLO: &str = "hello";
}

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = tags::HEALTH, description = "Health"),
        (name = tags::HELLO, description = "Hello"),
    )
)]
struct ApiDoc;

pub(crate) fn router() -> Result<Router> {
    let (prometheus_layer, metric_handle) = PrometheusMetricLayerBuilder::new()
        .with_ignore_pattern("/self")
        .with_ignore_pattern("/ready")
        .with_ignore_pattern("/metrics")
        .with_default_metrics()
        .build_pair();

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(routes::health_routes::routes())
        .merge(routes::hello_routes::routes().with_state(state::HelloState::new()?))
        .route(
            "/metrics",
            routing::get(async move || metric_handle.render()),
        )
        .layer(prometheus_layer)
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/swagger").url("/apidoc/openapi.json", api));

    Ok(router)
}
