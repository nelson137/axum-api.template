use std::sync::Arc;

use anyhow::Result;

use crate::web::service;

#[derive(Clone)]
pub struct HelloState {
    pub hello_service: Arc<dyn service::HelloService>,
}

impl HelloState {
    pub fn new() -> Result<Self> {
        Ok(Self {
            hello_service: Arc::new(service::HelloIpService::default()),
        })
    }

    #[cfg(test)]
    pub fn from_parts<HelloSvc: service::HelloService>(hello_svc: HelloSvc) -> Self {
        Self {
            hello_service: Arc::new(hello_svc),
        }
    }
}
