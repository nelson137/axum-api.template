use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use tracing::{debug, error};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::web::{response::HelloResponse, state, tags};

pub fn routes() -> OpenApiRouter<state::HelloState> {
    OpenApiRouter::new().routes(routes!(hello))
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct HelloQuery {
    /// Response format
    #[param(inline)]
    format: Option<HelloFormat>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
enum HelloFormat {
    Html,
    Json,
}

#[utoipa::path(
    get,
    path = "/hello",
    tag = tags::HELLO,
    summary = "say hello",
    params(HelloQuery),
    responses(
        (
            status = OK,
            description = "Hello response.",
            body = HelloResponse,
            content(
                ("text/html", example = "<p>Hello 👋</p>"),
                ("application/json", example = json!({ "ip": "127.0.0.1" })),
            ),
        ),
    ),
)]
pub async fn hello(
    State(state): State<state::HelloState>,
    Query(query): Query<HelloQuery>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let ip = match state.hello_service.get_ip().await {
        Ok(ip) => ip,
        Err(err) => {
            error!("Unable to get IP address: {}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to get IP address",
            ));
        }
    };

    debug!(ip = &ip, format = ?query.format, "hello");

    let response = match query.format {
        Some(HelloFormat::Json) => HelloResponse::Json(serde_json::json!({ "ip": ip })),
        _ => HelloResponse::Html(format!("<p>🕵️ Hello <em>{ip}</em>!</p>\n")),
    };

    Ok(response)
}

#[cfg(test)]
mod tests {
    use crate::{
        test_utils::*,
        web::{service, state},
    };

    use super::*;

    #[tokio::test]
    async fn test_hello() {
        let ip = rand_string();
        let expected = format!("<p>🕵️ Hello <em>{ip}</em>!</p>\n");

        let mut hello_svc = service::MockHelloService::new();
        {
            let ip = ip.clone();
            hello_svc
                .expect_get_ip()
                .return_once(move || async_ret(Ok(ip)));
        }

        let query = HelloQuery { format: None };

        let state = state::HelloState::from_parts(hello_svc);
        let actual = hello(State(state), Query(query))
            .await
            .read_response_as_string()
            .await;

        assert!(actual.contains(&ip));
        assert_eq!(expected, actual);
    }
}
