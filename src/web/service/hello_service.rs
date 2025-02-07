use anyhow::Result;

use super::Service;

#[async_trait::async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait HelloService: Service {
    async fn get_ip(&self) -> Result<String>;
}

#[derive(Clone, Default)]
pub struct HelloIpService();

#[async_trait::async_trait]
impl HelloService for HelloIpService {
    async fn get_ip(&self) -> Result<String> {
        Ok("127.0.0.1".to_string())
    }
}
