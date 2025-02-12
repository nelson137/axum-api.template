use anyhow::{Context, Result};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod settings;
#[cfg(test)]
mod test_utils;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    let settings = settings::Settings::new().context("failed to build app settings")?;

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
        let url = url::Url::parse(&settings.loki_url).context("failed to parse loki url")?;
        let (layer, task) = tracing_loki::builder()
            .extra_field("service_name", env!("CARGO_PKG_NAME"))?
            .extra_field("environment", &settings.environment)?
            .build_url(url)
            .context("failed to build loki layer")?;
        tokio::spawn(task);
        layer
    });

    subscriber.init();

    let routes = match web::router() {
        Ok(r) => r,
        Err(err) => panic!("{err}"),
    };

    let listener = tokio::net::TcpListener::bind(&settings.bind_addr)
        .await
        .with_context(|| format!("failed to bind to address: {}", settings.bind_addr))?;

    tracing::info!(
        addr = listener.local_addr().unwrap().to_string(),
        version = env!("CARGO_PKG_VERSION"),
        "Starting server",
    );

    axum::serve(listener, routes.into_make_service())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.expect("cancellation signal")
        })
        .await
        .context("failed to run server")
}
