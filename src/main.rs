#[cfg(test)]
mod test_utils;
mod web;

#[cfg(not(feature = "listen_public"))]
const BIND_ADDR: &str = "localhost:8080";

#[cfg(feature = "listen_public")]
const BIND_ADDR: &str = "0.0.0.0:8080";

#[tokio::main]
async fn main() {
    let routes = match web::router() {
        Ok(r) => r,
        Err(err) => panic!("{err}"),
    };

    let listener = tokio::net::TcpListener::bind(BIND_ADDR)
        .await
        .expect("bind to listen address");
    eprintln!("Listening on {:?}", listener.local_addr().unwrap());

    axum::serve(listener, routes.into_make_service())
        .await
        .expect("run server");
}
