mod hello_service;
pub use hello_service::*;

pub trait Service: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> Service for T {}
