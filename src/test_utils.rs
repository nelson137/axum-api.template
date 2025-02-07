use std::{future, pin::Pin};

use axum::{body::Body, http::Response, response::IntoResponse};

pub fn rand_string() -> String {
    rand_string_len(rand::random_range(6..=12))
}

pub fn rand_string_len(len: usize) -> String {
    rand::random_iter::<char>()
        .filter(|c| c.is_ascii_graphic())
        .take(len)
        .collect()
}

pub trait ReadResponseBody {
    async fn read_response_as_bytes(self) -> Vec<u8>;
    async fn read_response_as_string(self) -> String;
}

impl<T: IntoResponse> ReadResponseBody for T {
    async fn read_response_as_bytes(self) -> Vec<u8> {
        async fn inner(response: Response<Body>) -> Vec<u8> {
            axum::body::to_bytes(response.into_body(), 1024)
                .await
                .unwrap()
                .into_iter()
                .collect::<Vec<_>>()
        }
        inner(self.into_response()).await
    }

    async fn read_response_as_string(self) -> String {
        String::from_utf8_lossy(&self.read_response_as_bytes().await).into_owned()
    }
}

pub fn async_ret<T: Send>(value: T) -> Pin<Box<impl Future<Output = T> + Send>> {
    Box::pin(future::ready(value))
}
