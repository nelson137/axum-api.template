use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod settings;
#[cfg(test)]
mod test_utils;
mod web;

static APP_NAME: &str = env!("CARGO_PKG_NAME");
static APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    let settings = settings::Settings::new().expect("parse config");

    let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| settings.log_filter.parse().unwrap());

    let fmt_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_target(false)
        .with_file(true)
        .with_line_number(true);

    let subscriber = tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer);

    #[cfg(feature = "loki")]
    let subscriber = subscriber.with({
        let url = url::Url::parse(&settings.loki_url).unwrap();
        let (layer, task) = tracing_loki::builder().build_url(url).unwrap();
        tokio::spawn(task);
        layer
    });

    subscriber.init();

    tracing::info!(version = APP_VERSION, "{APP_NAME}");

    let routes = match web::router() {
        Ok(r) => r,
        Err(err) => panic!("{err}"),
    };

    let listener = tokio::net::TcpListener::bind(&settings.bind_addr)
        .await
        .expect("bind to listen address");
    tracing::info!("Listening on {:?}", listener.local_addr().unwrap());

    axum::serve(listener, routes.into_make_service())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.expect("cancellation signal")
        })
        .await
        .expect("run server");
}
