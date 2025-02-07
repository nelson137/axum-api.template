use axum::response::{self, IntoResponse};

#[derive(utoipa::ToSchema)]
pub enum HelloResponse {
    Html(String),
    Json(serde_json::Value),
}

impl IntoResponse for HelloResponse {
    fn into_response(self) -> response::Response {
        match self {
            HelloResponse::Html(html) => response::Html(html).into_response(),
            HelloResponse::Json(json) => response::Json(json).into_response(),
        }
    }
}
