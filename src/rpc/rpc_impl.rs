use jsonrpsee::core::async_trait;
use jsonrpsee::types::ErrorObjectOwned;

use super::traits::{RpcServer, PROTOCOL_VERSION};

pub struct RpcServerImpl;

#[async_trait]
impl RpcServer for RpcServerImpl {
    fn version(&self) -> Result<String, ErrorObjectOwned> {
        Ok(PROTOCOL_VERSION.to_string())
    }

    async fn ping(&self) -> Result<String, ErrorObjectOwned> {
        Ok("Pong".to_string())
    }
}
