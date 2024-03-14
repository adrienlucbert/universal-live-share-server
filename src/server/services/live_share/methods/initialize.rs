use super::super::InitializeParams;
use tonic::{Request, Response, Status};

pub async fn initialize(_request: Request<InitializeParams>) -> Result<Response<()>, Status> {
    Ok(Response::new(()))
}
