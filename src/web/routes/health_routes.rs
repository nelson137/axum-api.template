use axum::{http::StatusCode, response::IntoResponse};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::web::tags;

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(self_))
        .routes(routes!(ready))
}

#[utoipa::path(
    get,
    path = "/self",
    tag = tags::HEALTH,
    description = "Liveness probe.",
    responses((status = OK, description = "Alive.")),
)]
pub async fn self_() -> impl IntoResponse {
    StatusCode::OK
}

#[utoipa::path(
    get,
    path = "/ready",
    tag = tags::HEALTH,
    description = "Readiness probe.",
    responses((status = OK, description = "Ready for requests.")),
)]
pub async fn ready() -> impl IntoResponse {
    StatusCode::OK
}
