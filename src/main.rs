use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[cfg(test)]
mod test_utils;
mod web;

static APP_NAME: &str = env!("CARGO_PKG_NAME");
static APP_VERSION: &str = env!("CARGO_PKG_VERSION");
static CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

#[cfg(not(feature = "listen_public"))]
const BIND_ADDR: &str = "localhost:8080";

#[cfg(feature = "listen_public")]
const BIND_ADDR: &str = "0.0.0.0:8080";

#[tokio::main]
async fn main() {
    let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        format!("info,axum_web=trace,{CRATE_NAME}=debug")
            .parse()
            .unwrap()
    });
    let fmt_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_target(false)
        .with_file(true)
        .with_line_number(true);
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    tracing::info!(version = APP_VERSION, "{APP_NAME}");

    let routes = match web::router() {
        Ok(r) => r,
        Err(err) => panic!("{err}"),
    };

    let listener = tokio::net::TcpListener::bind(BIND_ADDR)
        .await
        .expect("bind to listen address");
    tracing::info!("Listening on {:?}", listener.local_addr().unwrap());

    axum::serve(listener, routes.into_make_service())
        .await
        .expect("run server");
}
