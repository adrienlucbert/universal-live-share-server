use super::live_share_server::LiveShare;
use super::{RawMessage, InitializeParams};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct LiveShareImpl {}

#[tonic::async_trait]
impl LiveShare for LiveShareImpl {
    async fn initialize(&self, request: Request<InitializeParams>) -> Result<Response<()>, Status> {
        super::methods::initialize(request).await
    }

    async fn ping(&self, request: Request<RawMessage>) -> Result<Response<RawMessage>, Status> {
        super::methods::ping(request).await
    }

    async fn version(&self, request: Request<()>) -> Result<Response<RawMessage>, Status> {
        super::methods::version(request).await
    }
}
